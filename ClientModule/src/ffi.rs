//! FFI (Foreign Function Interface) functions
//! 
//! These functions provide a C-compatible interface that Unreal Engine
//! can call. Each function acts like a diplomatic messenger, carrying
//! requests from Unreal Engine to SpacetimeDB and bringing back responses.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

/// Result structure that's safe to pass across the FFI boundary
/// This is like a standardized diplomatic pouch that can safely
/// carry messages between different programming languages
#[repr(C)]
pub struct FFIResult {
    pub success: bool,
    pub error_message: *mut c_char,
    pub data: *mut c_char,
}

impl FFIResult {
    /// Create a success result
    pub fn success(data: Option<String>) -> Self {
        let data_ptr = if let Some(data_str) = data {
            CString::new(data_str).unwrap().into_raw()
        } else {
            ptr::null_mut()
        };
        
        Self {
            success: true,
            error_message: ptr::null_mut(),
            data: data_ptr,
        }
    }
    
    /// Create an error result
    pub fn error(message: &str) -> Self {
        let error_cstring = CString::new(message).unwrap_or_else(|_| {
            CString::new("Error creating error message").unwrap()
        });
        
        Self {
            success: false,
            error_message: error_cstring.into_raw(),
            data: ptr::null_mut(),
        }
    }
}

/// Connect to SpacetimeDB server
/// This is like establishing a diplomatic connection between two countries
#[no_mangle]
pub extern "C" fn spacetimedb_connect(
    host: *const c_char,
    port: u16,
    database_name: *const c_char,
) -> FFIResult {
    let result = || -> Result<(), String> {
        // Convert C strings to Rust strings safely
        let host_str = unsafe {
            if host.is_null() {
                return Err("Host cannot be null".to_string());
            }
            CStr::from_ptr(host).to_str()
                .map_err(|_| "Invalid host string".to_string())?
        };
        
        let db_name = unsafe {
            if database_name.is_null() {
                return Err("Database name cannot be null".to_string());
            }
            CStr::from_ptr(database_name).to_str()
                .map_err(|_| "Invalid database name".to_string())?
        };
        
        // In a real implementation, you would establish the actual connection here
        // For now, we'll simulate a successful connection
        log::info!("Connecting to {}:{} database: {}", host_str, port, db_name);
        
        Ok(())
    };
    
    match result() {
        Ok(_) => FFIResult::success(None),
        Err(e) => FFIResult::error(&e),
    }
}

/// Register a new user account
#[no_mangle]
pub extern "C" fn spacetimedb_register_user(
    username: *const c_char,
    password: *const c_char,
) -> FFIResult {
    let result = || -> Result<(), String> {
        let username_str = unsafe {
            if username.is_null() {
                return Err("Username cannot be null".to_string());
            }
            CStr::from_ptr(username).to_str()
                .map_err(|_| "Invalid username".to_string())?
        };
        
        let password_str = unsafe {
            if password.is_null() {
                return Err("Password cannot be null".to_string());
            }
            CStr::from_ptr(password).to_str()
                .map_err(|_| "Invalid password".to_string())?
        };
        
        // Validate input parameters
        if username_str.len() < 3 {
            return Err("Username must be at least 3 characters".to_string());
        }
        
        if password_str.len() < 8 {
            return Err("Password must be at least 8 characters".to_string());
        }
        
        // In a real implementation, you would call the SpacetimeDB reducer here
        log::info!("User registration request: {}", username_str);
        
        Ok(())
    };
    
    match result() {
        Ok(_) => FFIResult::success(None),
        Err(e) => FFIResult::error(&e),
    }
}

/// Spawn a player character in the world
#[no_mangle]
pub extern "C" fn spacetimedb_spawn_player_character(
    zone_id: u32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
) -> FFIResult {
    // Validate input parameters
    if !position_x.is_finite() || !position_y.is_finite() || !position_z.is_finite() {
        return FFIResult::error("Invalid position coordinates");
    }
    
    // In a real implementation, you would call the SpacetimeDB reducer here
    // and return the actual object ID
    log::info!("Spawning player character at ({}, {}, {}) in zone {}", 
               position_x, position_y, position_z, zone_id);
    
    // Simulate returning an object ID
    let object_id = 12345u64; // This would be the real ID from SpacetimeDB
    FFIResult::success(Some(object_id.to_string()))
}

/// Free memory allocated by FFI functions
/// This is crucial to prevent memory leaks across the language boundary
#[no_mangle]
pub extern "C" fn spacetimedb_free_result(result: *mut FFIResult) {
    if result.is_null() {
        return;
    }
    
    unsafe {
        let result_ref = &mut *result;
        
        // Free error message if it exists
        if !result_ref.error_message.is_null() {
            drop(CString::from_raw(result_ref.error_message));
            result_ref.error_message = ptr::null_mut();
        }
        
        // Free data if it exists
        if !result_ref.data.is_null() {
            drop(CString::from_raw(result_ref.data));
            result_ref.data = ptr::null_mut();
        }
    }
}