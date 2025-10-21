use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use sqlx::postgres::PgPoolOptions;

#[derive(Parser)]
#[command(name = "fta-migration")]
#[command(about = "Financial Tracker API - Database Migration Tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run,

    Revert,

    Info,

    Add { name: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    let database_url = std::env::var("DATABASE_URL")
        .context("DATABASE_URL must be set in .env file or environment")?;

    match cli.command {
        Commands::Run => run_migrations(&database_url).await?,
        Commands::Revert => revert_migration(&database_url).await?,
        Commands::Info => show_migration_info(&database_url).await?,
        Commands::Add { name } => create_migration(&name)?,
    }

    Ok(())
}

async fn run_migrations(database_url: &str) -> Result<()> {
    println!("🔄 Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to connect to database")?;

    println!("📦 Running migrations...");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("Failed to run migrations")?;

    println!("✅ All migrations applied successfully!");

    Ok(())
}

async fn revert_migration(database_url: &str) -> Result<()> {
    println!("🔄 Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to connect to database")?;

    println!("⏮️  Reverting last migration...");

    sqlx::migrate!("./migrations")
        .undo(&pool, 1)
        .await
        .context("Failed to revert migration")?;

    println!("✅ Migration reverted successfully!");

    Ok(())
}

async fn show_migration_info(database_url: &str) -> Result<()> {
    println!("🔄 Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .context("Failed to connect to database")?;

    println!("\n📊 Migration Status:\n");

    let migrator = sqlx::migrate!("./migrations");
    let migrations = migrator.migrations.as_ref();

    let applied: Vec<i64> =
        sqlx::query_scalar("SELECT version FROM _sqlx_migrations ORDER BY version")
            .fetch_all(&pool)
            .await
            .unwrap_or_default();

    let applied_count = applied.len();
    let total_count = migrations.len();
    let pending_count = total_count - applied_count;

    println!("  Total migrations:   {total_count}");
    println!("  Applied:            {applied_count}");
    println!("  Pending:            {pending_count}");
    println!("\n📝 Migrations:\n");

    for migration in migrations {
        let version = migration.version;
        let is_applied = applied.contains(&version);
        let status = if is_applied {
            "✅ Applied"
        } else {
            "⏳ Pending"
        };

        println!("  {} - {} ({})", version, migration.description, status);
    }

    println!();

    Ok(())
}

fn create_migration(name: &str) -> Result<()> {
    use std::fs;

    let version = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();

    let safe_name = name
        .to_lowercase()
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>();

    let filename = format!("{version}_{safe_name}.sql");
    let filepath = format!("./migrations/{filename}");

    let template = format!(
        "-- Migration: {}\n\
         -- Created: {}\n\
         \n\
         -- Add your SQL statements here\n\
         \n",
        safe_name,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );

    fs::write(&filepath, template).context("Failed to create migration file")?;

    println!("✅ Created new migration: {filename}");
    println!("   Path: {filepath}");
    println!("\n💡 Edit the file to add your SQL statements, then run: make migrate");

    Ok(())
}
