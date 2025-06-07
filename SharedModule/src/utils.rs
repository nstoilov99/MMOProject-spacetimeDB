//! Utility functions shared across all modules

use spacetimedb::{Identity, Timestamp};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Import constants from our constants module
use crate::constants::*;

/// Generate a unique ID using various entropy sources
pub fn generate_unique_id(identity: &Identity, timestamp: Timestamp) -> u64 {
    let mut hasher = DefaultHasher::new();
    format!("{:?}", identity).hash(&mut hasher);
    format!("{:?}", timestamp).hash(&mut hasher);
    
    // Add additional entropy if available
    if let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        now.subsec_nanos().hash(&mut hasher);
    }
    
    hasher.finish()
}

/// Validate username according to game rules
pub fn validate_username(username: &str) -> Result<(), String> {
    let trimmed = username.trim();
    
    if trimmed.is_empty() {
        return Err("Username cannot be empty".to_string());
    }
    
    if trimmed.len() < MIN_USERNAME_LENGTH {
        return Err(format!("Username must be at least {} characters", MIN_USERNAME_LENGTH));
    }
    
    if trimmed.len() > MAX_USERNAME_LENGTH {
        return Err(format!("Username cannot exceed {} characters", MAX_USERNAME_LENGTH));
    }
    
    // Check for valid characters (alphanumeric and underscore only)
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username can only contain letters, numbers, and underscores".to_string());
    }
    
    Ok(())
}

/// Validate password strength
pub fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(format!("Password must be at least {} characters", MIN_PASSWORD_LENGTH));
    }
    
    // Add more password strength checks here if needed
    Ok(())
}

/// Validate email format (basic check)
pub fn validate_email(email: &str) -> Result<(), String> {
    if !email.contains('@') || !email.contains('.') {
        return Err("Invalid email format".to_string());
    }
    Ok(())
}

/// Calculate distance between two 3D points
pub fn calculate_distance(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2) + (z2 - z1).powi(2)).sqrt()
}

/// Check if movement is valid (not teleporting)
pub fn validate_movement(
    old_x: f32, old_y: f32, old_z: f32,
    new_x: f32, new_y: f32, new_z: f32,
    max_distance: f32
) -> Result<(), String> {
    let distance = calculate_distance(old_x, old_y, old_z, new_x, new_y, new_z);
    
    if distance > max_distance {
        return Err("Invalid movement detected: distance too large".to_string());
    }
    
    // Check for NaN or infinite values
    if !new_x.is_finite() || !new_y.is_finite() || !new_z.is_finite() {
        return Err("Invalid position coordinates".to_string());
    }
    
    Ok(())
}

/// Sanitize chat message content
pub fn sanitize_chat_message(message: &str) -> Result<String, String> {
    let trimmed = message.trim();
    
    if trimmed.is_empty() {
        return Err("Message cannot be empty".to_string());
    }
    
    if trimmed.len() > MAX_CHAT_MESSAGE_LENGTH {
        return Err("Message too long".to_string());
    }
    
    // Remove any potentially harmful characters or sequences
    // This is a basic implementation - you might want more sophisticated filtering
    let sanitized = trimmed
        .chars()
        .filter(|&c| c.is_ascii_graphic() || c.is_ascii_whitespace())
        .collect();
    
    Ok(sanitized)
}