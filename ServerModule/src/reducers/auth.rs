//! Authentication reducers

use spacetimedb::{reducer, ReducerContext};
use shared_module::*;
use crate::tables::*;
use sha2::{Sha256, Digest};

/// WASM-compatible password hashing using SHA256
fn hash_password(password: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());
    hasher.update(b"mmo_server_secret_salt"); // Additional server-side salt
    
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Verify password against stored hash and salt
fn verify_password(password: &str, stored_hash: &str, salt: &str) -> bool {
    let computed_hash = hash_password(password, salt);
    computed_hash == stored_hash
}

/// Generate a salt using the user's identity and current timestamp
fn generate_salt(identity: &spacetimedb::Identity, timestamp: spacetimedb::Timestamp) -> String {
    let mut hasher = Sha256::new();
    
    // Use string representations for hashing since direct byte access isn't available
    let identity_string = format!("{:?}", identity);
    hasher.update(identity_string.as_bytes());
    
    let timestamp_string = format!("{:?}", timestamp);
    hasher.update(timestamp_string.as_bytes());
    
    hasher.update(b"salt_generation_key");
    
    let result = hasher.finalize();
    format!("{:x}", result)[..16].to_string() // Take first 16 characters as salt
}

/// Register a new user account
#[reducer]
pub fn register_user(
    ctx: &ReducerContext,
    username: String,
    password: String,
    email: Option<String>
) -> Result<(), String> {
    // Input validation using shared utilities
    validate_username(&username)?;
    validate_password(&password)?;
    
    if let Some(ref email_addr) = email {
        validate_email(email_addr)?;
    }
    
    // Check if username is already taken
    if User::filter_by_username(ctx, &username).is_some() {
        return Err("Username is already taken".to_string());
    }
    
    // Generate salt and hash password
    let salt = generate_salt(&ctx.sender, ctx.timestamp);
    let password_hash = hash_password(&password, &salt);
    
    // Create the new user
    ctx.db.user().insert(User {
        identity: ctx.sender,
        username: username.clone(),
        password_hash,
        password_salt: salt,
        email,
        created_at: ctx.timestamp,
        last_login: ctx.timestamp,
        is_active: true,
    });
    
    log::info!("New user registered: {}", username);
    Ok(())
}

/// Authenticate a user and create a game session
#[reducer]
pub fn login_user(
    ctx: &ReducerContext,
    username: String,
    password: String,
    client_version: String
) -> Result<(), String> {
    // Find the user by username
    let user = User::filter_by_username(ctx, &username)
        .ok_or("Invalid username or password")?;
    
    // Check if account is active
    if !user.is_active {
        return Err("Account is suspended".to_string());
    }
    
    // Verify the password using stored salt
    if !verify_password(&password, &user.password_hash, &user.password_salt) {
        log::warn!("Failed login attempt for user: {}", username);
        return Err("Invalid username or password".to_string());
    }
    
    // Update last login time
    User::update_last_login(ctx, &ctx.sender, ctx.timestamp);
    
    // Get client IP (placeholder since detailed connection info isn't available)
    let client_ip = "unknown".to_string();
    
    // Create or update game session
    if GameSession::filter_by_identity(ctx, &ctx.sender).is_some() {
        // Update existing session
        GameSession::update_activity(ctx, &ctx.sender, ctx.timestamp);
    } else {
        // Create new session
        GameSession::create_session(
            ctx,
            ctx.sender,
            Some(ctx.connection_id),
            client_version,
            client_ip,
            ctx.timestamp
        );
    }
    
    log::info!("User logged in: {}", username);
    Ok(())
}

/// End the current session
#[reducer]
pub fn logout_user(ctx: &ReducerContext) -> Result<(), String> {
    // Remove the game session
    GameSession::remove_session(ctx, &ctx.sender);
    
    // Mark player as offline if they exist
    Player::set_online_status(ctx, &ctx.sender, false, ctx.timestamp);
    
    log::info!("User logged out: {:?}", ctx.sender);
    Ok(())
}