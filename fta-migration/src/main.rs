use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use std::fs;
use std::path::Path;

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

async fn ensure_migrations_table(db: &sea_orm::DatabaseConnection) -> Result<()> {
    db.execute(Statement::from_string(
        DbBackend::Postgres,
        "CREATE TABLE IF NOT EXISTS _sea_orm_migrations (
            version BIGINT PRIMARY KEY,
            description TEXT NOT NULL,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"
        .to_string(),
    ))
    .await
    .context("Failed to create migrations table")?;
    Ok(())
}

async fn get_applied_migrations(
    db: &sea_orm::DatabaseConnection,
) -> Result<Vec<(i64, String)>> {
    let rows = db
        .query_all(Statement::from_string(
            DbBackend::Postgres,
            "SELECT version, description FROM _sea_orm_migrations ORDER BY version".to_string(),
        ))
        .await
        .unwrap_or_default();

    let mut migrations = Vec::new();
    for row in rows {
        let version: i64 = sea_orm::TryGetable::try_get_by_index(&row, 0)
            .map_err(|e| anyhow::anyhow!("Failed to get version: {:?}", e))?;
        let description: String = sea_orm::TryGetable::try_get_by_index(&row, 1)
            .map_err(|e| anyhow::anyhow!("Failed to get description: {:?}", e))?;
        migrations.push((version, description));
    }
    Ok(migrations)
}

fn parse_migration_files() -> Result<Vec<(i64, String, String)>> {
    let migrations_dir = Path::new("./migrations");
    if !migrations_dir.exists() {
        return Ok(Vec::new());
    }

    let mut migrations = Vec::new();

    for entry in fs::read_dir(migrations_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                // Parse version from filename (format: YYYYMMDDHHMMSS_description.sql)
                if let Some(version_str) = filename.split('_').next() {
                    if let Ok(version) = version_str.parse::<i64>() {
                        let description = filename
                            .strip_prefix(version_str)
                            .unwrap_or("")
                            .trim_start_matches('_')
                            .to_string();
                        let content = fs::read_to_string(&path)?;
                        migrations.push((version, description, content));
                    }
                }
            }
        }
    }

    migrations.sort_by_key(|(v, _, _)| *v);
    Ok(migrations)
}

async fn run_migrations(database_url: &str) -> Result<()> {
    println!("Connecting to database...");

    let db = Database::connect(database_url)
        .await
        .context("Failed to connect to database")?;

    ensure_migrations_table(&db).await?;

    println!("Running migrations...");

    let applied = get_applied_migrations(&db).await?;
    let applied_versions: Vec<i64> = applied.iter().map(|(v, _)| *v).collect();

    let migrations = parse_migration_files()?;
    let mut applied_count = 0;

    for (version, description, content) in migrations {
        if applied_versions.contains(&version) {
            continue;
        }

        println!("  Applying: {} - {}", version, description);

        // Execute migration
        db.execute(Statement::from_string(DbBackend::Postgres, content))
            .await
            .with_context(|| format!("Failed to apply migration: {}", description))?;

        // Record migration
        db.execute(Statement::from_string(
            DbBackend::Postgres,
            format!(
                "INSERT INTO _sea_orm_migrations (version, description) VALUES ({}, '{}')",
                version, description
            ),
        ))
        .await?;

        applied_count += 1;
    }

    if applied_count == 0 {
        println!("All migrations are already applied!");
    } else {
        println!("Applied {} migration(s) successfully!", applied_count);
    }

    Ok(())
}

async fn revert_migration(database_url: &str) -> Result<()> {
    println!("Connecting to database...");

    let db = Database::connect(database_url)
        .await
        .context("Failed to connect to database")?;

    ensure_migrations_table(&db).await?;

    println!("Reverting last migration...");

    let applied = get_applied_migrations(&db).await?;
    if let Some((version, description)) = applied.last() {
        println!("  Reverting: {} - {}", version, description);

        // Remove migration record
        db.execute(Statement::from_string(
            DbBackend::Postgres,
            format!("DELETE FROM _sea_orm_migrations WHERE version = {}", version),
        ))
        .await?;

        println!("Migration record removed. Note: Schema changes are NOT automatically reverted.");
        println!("You may need to manually revert the schema changes.");
    } else {
        println!("No migrations to revert!");
    }

    Ok(())
}

async fn show_migration_info(database_url: &str) -> Result<()> {
    println!("Connecting to database...");

    let db = Database::connect(database_url)
        .await
        .context("Failed to connect to database")?;

    ensure_migrations_table(&db).await?;

    println!("\nMigration Status:\n");

    let applied = get_applied_migrations(&db).await?;
    let applied_versions: Vec<i64> = applied.iter().map(|(v, _)| *v).collect();

    let migrations = parse_migration_files()?;
    let total_count = migrations.len();
    let applied_count = applied.len();
    let pending_count = total_count - applied_count;

    println!("  Total migrations:   {}", total_count);
    println!("  Applied:            {}", applied_count);
    println!("  Pending:            {}", pending_count);
    println!("\nMigrations:\n");

    for (version, description, _) in &migrations {
        let is_applied = applied_versions.contains(version);
        let status = if is_applied {
            "Applied"
        } else {
            "Pending"
        };

        println!("  {} - {} ({})", version, description, status);
    }

    println!();

    Ok(())
}

fn create_migration(name: &str) -> Result<()> {
    let version = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();

    let safe_name = name
        .to_lowercase()
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect::<String>();

    let filename = format!("{}_{}.sql", version, safe_name);
    let filepath = format!("./migrations/{}", filename);

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

    println!("Created new migration: {}", filename);
    println!("   Path: {}", filepath);
    println!("\nEdit the file to add your SQL statements, then run: make migrate");

    Ok(())
}
