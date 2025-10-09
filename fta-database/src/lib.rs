use anyhow::{anyhow, Result};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await;

    match pool {
        Ok(pool) => Ok(pool),
        Err(err) => Err(anyhow!("Failed to connect to database: {err}")),
    }
}
