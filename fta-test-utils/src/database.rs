use sqlx::postgres::{PgPool, PgPoolOptions};
use std::sync::Arc;
use uuid::Uuid;

pub struct TestDatabase {
    pub pool: PgPool,
    db_name: String,
    admin_pool: PgPool,
}

impl TestDatabase {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let base_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432".to_string());

        let admin_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&format!("{}/postgres", base_url))
            .await?;

        let db_name = format!("test_db_{}", Uuid::new_v4().to_string().replace('-', "_"));

        sqlx::query(&format!("CREATE DATABASE {}", db_name))
            .execute(&admin_pool)
            .await?;

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&format!("{}/{}", base_url, db_name))
            .await?;

        Ok(Self {
            pool,
            db_name,
            admin_pool,
        })
    }

    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        Ok(())
    }

    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub fn arc_pool(&self) -> Arc<PgPool> {
        Arc::new(self.pool.clone())
    }
}

impl Drop for TestDatabase {
    fn drop(&mut self) {
        let db_name = self.db_name.clone();
        let admin_pool = self.admin_pool.clone();

        let pool = self.pool.clone();
        tokio::task::spawn(async move {
            pool.close().await;
        });

        tokio::task::spawn(async move {
            let _ = sqlx::query(&format!(
                "SELECT pg_terminate_backend(pg_stat_activity.pid) \
                 FROM pg_stat_activity \
                 WHERE pg_stat_activity.datname = '{}' \
                 AND pid <> pg_backend_pid()",
                db_name
            ))
            .execute(&admin_pool)
            .await;

            let _ = sqlx::query(&format!("DROP DATABASE IF EXISTS {}", db_name))
                .execute(&admin_pool)
                .await;
        });
    }
}

pub async fn with_test_db<F, Fut>(test_fn: F)
where
    F: FnOnce(PgPool) -> Fut,
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
