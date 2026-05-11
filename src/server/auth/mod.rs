pub mod crypto;
pub mod handlers;
pub mod jwt;
pub mod session;

pub use crypto::{hash_password, normalize_username, validate_password, verify_password};
pub use handlers::{login, me, register};
pub use jwt::{Claims, JwtService};
pub use session::require_user_id;
