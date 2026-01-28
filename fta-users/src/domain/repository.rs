//! Repository trait for user persistence operations.

use async_trait::async_trait;
use fta_errors::AppError;
use uuid::Uuid;

use super::User;

/// Repository trait defining persistence operations for users.
///
/// Implementations handle the actual database interactions while
/// the domain layer remains agnostic to storage details.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Creates a new user in the database.
    async fn create(&self, user: User) -> Result<User, AppError>;

    /// Finds a user by their unique identifier.
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<User>, AppError>;

    /// Finds a user by their email address.
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;

    /// Retrieves a paginated list of users with filtering and sorting.
    async fn find_all(
        &self,
        filters: &crate::infrastructure::http::filters::UserFilters,
        sort_by: Option<&str>,
        sort_order: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<User>, AppError>;

    /// Counts all users matching the given filters.
    async fn count_all(
        &self,
        filters: &crate::infrastructure::http::filters::UserFilters,
    ) -> Result<i64, AppError>;

    /// Updates an existing user.
    async fn update(&self, user: User) -> Result<User, AppError>;

    /// Deletes a user by their unique identifier.
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}
