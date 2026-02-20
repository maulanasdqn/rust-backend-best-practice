use anyhow::{anyhow, Result};
use sea_orm::{Database, DatabaseConnection};

pub mod entities;

pub use entities::*;
pub use sea_orm;

pub type DbPool = DatabaseConnection;

pub async fn create_pool(database_url: &str) -> Result<DbPool> {
    let db = Database::connect(database_url).await;

    match db {
        Ok(conn) => Ok(conn),
        Err(err) => Err(anyhow!("Failed to connect to database: {err}")),
    }
}
