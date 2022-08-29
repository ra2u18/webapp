//! src/routes/admin/mod.rs

mod dashboard;
mod logout;
mod password;
mod newsletter;

pub use dashboard::admin_dashboard;
pub use logout::logout;
pub use password::*;
pub use newsletter::*;
