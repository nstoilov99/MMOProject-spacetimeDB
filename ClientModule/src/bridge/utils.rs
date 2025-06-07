//! Utility functions for FFI operations

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Helper function to convert Rust string to C string
pub fn string_to_c_char(s: String) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

/// Helper function to convert C string to Rust string
pub unsafe fn c_char_to_string(ptr: *const c_char) -> Result<String, std::str::Utf8Error> {
    if ptr.is_null() {
        return Ok(String::new());
    }
    
    let c_str = CStr::from_ptr(ptr);
    c_str.to_str().map(|s| s.to_owned())
}

/// Validate that a C string pointer is not null and convert it
pub unsafe fn validate_and_convert_c_string(
    ptr: *const c_char,
    field_name: &str
) -> Result<String, String> {
    if ptr.is_null() {
        return Err(format!("{} cannot be null", field_name));
    }
    
    c_char_to_string(ptr)
        .map_err(|_| format!("Invalid {} string", field_name))
}

/// Validate numeric values for positions
pub fn validate_position(x: f32, y: f32, z: f32) -> Result<(), String> {
    if !x.is_finite() || !y.is_finite() || !z.is_finite() {
        return Err("Invalid position coordinates".to_string());
    }
    Ok(())
}

/// Log FFI function calls for debugging
pub fn log_ffi_call(function_name: &str, args: &str) {
    log::debug!("FFI Call: {}({})", function_name, args);
}

/// Handle FFI errors consistently
pub fn handle_ffi_error(error: &str, function_name: &str) -> crate::FFIResult {
    log::error!("FFI Error in {}: {}", function_name, error);
    crate::FFIResult::error(error)
}