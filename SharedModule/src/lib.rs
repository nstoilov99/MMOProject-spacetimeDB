//! SharedModule: Common types and utilities for the MMO system
//! 
//! This module defines the "contract" between client and server.
//! Think of it as the common language that all parts of your system speak.

use spacetimedb::{Identity, Timestamp};

// Module organization
pub mod types;
pub mod constants;
pub mod rpc;
pub mod utils;

// Re-export important types at the crate level
pub use types::*;
pub use constants::*;
pub use rpc::*;
pub use utils::*;