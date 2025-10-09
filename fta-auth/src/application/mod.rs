// Authentication use cases
pub mod change_password;
pub mod disable_2fa;
pub mod enable_2fa;
pub mod google_oauth_callback;
pub mod google_oauth_login;
pub mod login;
pub mod logout;
pub mod logout_all;
pub mod refresh_access_token;
pub mod register_user;
pub mod request_password_reset;
pub mod reset_password;
pub mod verify_2fa;
pub mod verify_email;

// Re-exports (make all public)
pub use change_password::*;
pub use disable_2fa::*;
pub use enable_2fa::*;
pub use google_oauth_callback::*;
pub use google_oauth_login::*;
pub use login::*;
pub use logout::*;
pub use logout_all::*;
pub use refresh_access_token::*;
pub use register_user::*;
pub use request_password_reset::*;
pub use reset_password::*;
pub use verify_2fa::*;
pub use verify_email::*;
