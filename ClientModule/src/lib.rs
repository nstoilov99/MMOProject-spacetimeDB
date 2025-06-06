//! ClientModule: FFI bridge between Rust and Unreal Engine
//! 
//! This module provides C-compatible functions that Unreal Engine
//! can call via FFI (Foreign Function Interface). Think of this
//! as the embassy between two countries that speak different languages.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use shared_module::*;

// Import our FFI module
pub mod ffi;
pub use ffi::*;

// Re-export shared types for convenience
pub use shared_module::*;

/// Initialize the client module
/// This sets up any client-side state needed for communication
pub fn initialize_client() {
    log::info!("Client module initialized");
}