//! Table definitions for the SpacetimeDB schema

pub mod user;
pub mod player;
pub mod chat;
pub mod session;

// Re-export all table types
pub use user::*;
pub use player::*;
pub use chat::*;
pub use session::*;