//! ClientModule: FFI bridge between Rust and Unreal Engine
//! 
//! This module provides C-compatible functions that Unreal Engine
//! can call via FFI (Foreign Function Interface).

use shared_module::*;

// Import our organized modules
pub mod ffi;
pub mod bridge;
pub mod client;

// Re-export important types and functions
pub use ffi::*;
pub use bridge::*;
pub use client::*;

// Re-export shared types for convenience
pub use shared_module::*;

/// Initialize the client module
pub fn initialize_client() {
    log::info!("Client module initialized");
}

/// Get client version information
pub fn get_client_version() -> String {
    "1.0.0".to_string()
}