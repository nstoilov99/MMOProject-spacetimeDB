//! FFI-safe type definitions for crossing the language boundary

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use shared_module::*;

/// Result type that's safe to pass across FFI boundary
#[repr(C)]
pub struct FFIResult {
    pub success: bool,
    pub error_message: *mut c_char,
    pub data: *mut c_void,
    pub data_size: usize,
}

impl FFIResult {
    /// Create a success result with optional data
    pub fn success(data: Option<Vec<u8>>) -> Self {
        if let Some(data_vec) = data {
            let data_size = data_vec.len();
            let data_ptr = Box::into_raw(data_vec.into_boxed_slice()) as *mut c_void;
            
            FFIResult {
                success: true,
                error_message: std::ptr::null_mut(),
                data: data_ptr,
                data_size,
            }
        } else {
            FFIResult {
                success: true,
                error_message: std::ptr::null_mut(),
                data: std::ptr::null_mut(),
                data_size: 0,
            }
        }
    }
    
    /// Create an error result with a message
    pub fn error(message: &str) -> Self {
        let error_cstring = CString::new(message).unwrap_or_else(|_| {
            CString::new("Error creating error message").unwrap()
        });
        
        FFIResult {
            success: false,
            error_message: error_cstring.into_raw(),
            data: std::ptr::null_mut(),
            data_size: 0,
        }
    }
}

/// Identity wrapper for FFI
#[repr(C)]
pub struct FFIIdentity {
    pub bytes: [u8; 32],
}

impl From<Identity> for FFIIdentity {
    fn from(identity: Identity) -> Self {
        let mut bytes = [0u8; 32];
        let identity_str = format!("{:?}", identity);
        let identity_bytes = identity_str.as_bytes();
        let copy_len = identity_bytes.len().min(32);
        bytes[..copy_len].copy_from_slice(&identity_bytes[..copy_len]);
        FFIIdentity { bytes }
    }
}

/// Player data that's safe to pass across FFI
#[repr(C)]
pub struct FFIPlayer {
    pub identity: FFIIdentity,
    pub username: *mut c_char,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub rotation_yaw: f32,
    pub level: u32,
    pub experience: u64,
    pub health: f32,
    pub max_health: f32,
    pub is_online: bool,
    pub current_zone: *mut c_char,
}

/// Chat message that's safe to pass across FFI
#[repr(C)]
pub struct FFIChatMessage {
    pub message_id: u64,
    pub sender_username: *mut c_char,
    pub message: *mut c_char,
    pub channel: *mut c_char,
    pub timestamp: u64, // Unix timestamp
}

/// Connection status for FFI
#[repr(C)]
pub enum FFIConnectionState {
    Disconnected = 0,
    Connecting = 1,
    Connected = 2,
    Authenticated = 3,
    InGame = 4,
    Error = 5,
}