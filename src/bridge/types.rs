// server/src/bridge/types.rs
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};

/// Result type that's safe to pass across FFI boundary
/// C++ doesn't understand Rust's Result type, so we create our own
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

/// Player data that's safe to pass across FFI
#[repr(C)]
pub struct FFIPlayer {
    pub identity_bytes: [u8; 32], // Identity serialized as bytes
    pub username: *mut c_char,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub rotation_yaw: f32,
    pub level: u32,
    pub health: f32,
    pub max_health: f32,
    pub is_online: bool,
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