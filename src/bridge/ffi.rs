// server/src/bridge/ffi.rs
use super::types::*;
use crate::*;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use spacetimedb::{Identity, Address};

/// Initialize connection to SpacetimeDB
/// This function establishes the connection from the Unreal client
#[no_mangle]
pub extern "C" fn spacetimedb_connect(
    host: *const c_char,
    port: u16,
    database_name: *const c_char
) -> FFIResult {
    let result = || -> Result<(), String> {
        let host_str = unsafe { 
            c_char_to_string(host)
                .map_err(|_| "Invalid host string".to_string())?
        };
        
        let db_name = unsafe {
            c_char_to_string(database_name)
                .map_err(|_| "Invalid database name".to_string())?
        };
        
        // In a real implementation, you'd use SpacetimeDB client SDK
        // For now, we'll simulate success
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
    email: *const c_char
) -> FFIResult {
    let result = || -> Result<(), String> {
        let username_str = unsafe {
            c_char_to_string(username)
                .map_err(|_| "Invalid username".to_string())?
        };
        
        let password_str = unsafe {
            c_char_to_string(password)
                .map_err(|_| "Invalid password".to_string())?
        };
        
        let email_str = if email.is_null() {
            None
        } else {
            Some(unsafe {
                c_char_to_string(email)
                    .map_err(|_| "Invalid email".to_string())?
            })
        };
        
        // In a real implementation, you'd call the actual register_user reducer
        // For now, we'll validate and simulate
        if username_str.len() < 3 {
            return Err("Username too short".to_string());
        }
        
        if password_str.len() < 8 {
            return Err("Password too short".to_string());
        }
        
        log::info!("User registration request: {}", username_str);
        Ok(())
    };
    
    match result() {
        Ok(_) => FFIResult::success(None),
        Err(e) => FFIResult::error(&e),
    }
}

/// Login user
#[no_mangle]
pub extern "C" fn spacetimedb_login_user(
    username: *const c_char,
    password: *const c_char
) -> FFIResult {
    let result = || -> Result<(), String> {
        let username_str = unsafe {
            c_char_to_string(username)
                .map_err(|_| "Invalid username".to_string())?
        };
        
        let password_str = unsafe {
            c_char_to_string(password)
                .map_err(|_| "Invalid password".to_string())?
        };
        
        // Simulate login validation
        if username_str.is_empty() || password_str.is_empty() {
            return Err("Username and password required".to_string());
        }
        
        log::info!("User login request: {}", username_str);
        Ok(())
    };
    
    match result() {
        Ok(_) => FFIResult::success(None),
        Err(e) => FFIResult::error(&e),
    }
}

/// Join the game
#[no_mangle]
pub extern "C" fn spacetimedb_join_game(starting_zone: *const c_char) -> FFIResult {
    let result = || -> Result<(), String> {
        let zone = unsafe {
            c_char_to_string(starting_zone)
                .map_err(|_| "Invalid zone name".to_string())?
        };
        
        log::info!("Player joining game in zone: {}", zone);
        Ok(())
    };
    
    match result() {
        Ok(_) => FFIResult::success(None),
        Err(e) => FFIResult::error(&e),
    }
}

/// Update player position
#[no_mangle]
pub extern "C" fn spacetimedb_update_position(
    x: f32,
    y: f32,
    z: f32,
    yaw: f32
) -> FFIResult {
    // Validate position values
    if !x.is_finite() || !y.is_finite() || !z.is_finite() || !yaw.is_finite() {
        return FFIResult::error("Invalid position values");
    }
    
    // In a real implementation, call the actual update_player_position reducer
    log::debug!("Position update: ({}, {}, {}) yaw: {}", x, y, z, yaw);
    
    FFIResult::success(None)
}

/// Send chat message
#[no_mangle]
pub extern "C" fn spacetimedb_send_chat(
    message: *const c_char,
    channel: *const c_char
) -> FFIResult {
    let result = || -> Result<(), String> {
        let message_str = unsafe {
            c_char_to_string(message)
                .map_err(|_| "Invalid message".to_string())?
        };
        
        let channel_str = unsafe {
            c_char_to_string(channel)
                .map_err(|_| "Invalid channel".to_string())?
        };
        
        if message_str.trim().is_empty() {
            return Err("Message cannot be empty".to_string());
        }
        
        if message_str.len() > 500 {
            return Err("Message too long".to_string());
        }
        
        log::info!("Chat message to {}: {}", channel_str, message_str);
        Ok(())
    };
    
    match result() {
        Ok(_) => FFIResult::success(None),
        Err(e) => FFIResult::error(&e),
    }
}

/// Free memory allocated by FFI functions
/// This is crucial to prevent memory leaks
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
            result_ref.error_message = std::ptr::null_mut();
        }
        
        // Free data if it exists
        if !result_ref.data.is_null() && result_ref.data_size > 0 {
            let data_slice = std::slice::from_raw_parts_mut(
                result_ref.data as *mut u8,
                result_ref.data_size
            );
            drop(Box::from_raw(data_slice));
            result_ref.data = std::ptr::null_mut();
            result_ref.data_size = 0;
        }
    }
}

/// Free a C string allocated by our FFI functions
#[no_mangle]
pub extern "C" fn spacetimedb_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
}