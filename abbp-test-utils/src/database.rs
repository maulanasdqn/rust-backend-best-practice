use abbp_database::DbPool;
use sea_orm::{ConnectionTrait, Database, DbBackend, Statement};
use std::sync::Arc;
use uuid::Uuid;

pub struct TestDatabase {
    pub pool: DbPool,
    db_name: String,
    admin_pool: DbPool,
}

impl std::fmt::Debug for TestDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestDatabase")
            .field("db_name", &self.db_name)
            .finish_non_exhaustive()
    }
}

impl TestDatabase {
    pub async fn new() -> Result<Self, sea_orm::DbErr> {
        let base_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432".to_string());

        let admin_pool = Database::connect(&format!("{}/postgres", base_url)).await?;

        let db_name = format!("test_db_{}", Uuid::new_v4().to_string().replace('-', "_"));

        admin_pool
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!("CREATE DATABASE {}", db_name),
            ))
            .await?;

        let pool = Database::connect(&format!("{}/{}", base_url, db_name)).await?;

        Ok(Self {
            pool,
            db_name,
            admin_pool,
        })
    }

    pub async fn run_migrations(&self) -> Result<(), sea_orm::DbErr> {
        Ok(())
    }

    pub fn pool(&self) -> DbPool {
        self.pool.clone()
    }

    pub fn arc_pool(&self) -> Arc<DbPool> {
        Arc::new(self.pool.clone())
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let db_name = self.db_name.clone();
        let admin_pool = self.admin_pool.clone();

        tokio::task::spawn(async move {
            let _ = admin_pool
                .execute(Statement::from_string(
                    DbBackend::Postgres,
                    format!(
                        "SELECT pg_terminate_backend(pg_stat_activity.pid) \
                     FROM pg_stat_activity \
                     WHERE pg_stat_activity.datname = '{}' \
                     AND pid <> pg_backend_pid()",
                        db_name
                    ),
                ))
                .await;

            let _ = admin_pool
                .execute(Statement::from_string(
                    DbBackend::Postgres,
                    format!("DROP DATABASE IF EXISTS {}", db_name),
                ))
                .await;
        });
    }
}

pub async fn with_test_db<F, Fut>(test_fn: F)
where
    F: FnOnce(DbPool) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let test_db = TestDatabase::new()
        .await
        .expect("Failed to create test database");

    test_db
        .run_migrations()
        .await
        .expect("Failed to run migrations");

    test_fn(test_db.pool()).await;
}

pub async fn setup_test_db() -> TestDatabase {
    let test_db = TestDatabase::new()
        .await
        .expect("Failed to create test database");

    test_db
        .run_migrations()
        .await
        .expect("Failed to run migrations");

    test_db
}
