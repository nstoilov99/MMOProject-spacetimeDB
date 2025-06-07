//! User account table definition

use spacetimedb::{table, Identity, Timestamp, ReducerContext};

/// User account information
/// This table stores persistent user data that survives across sessions
#[derive(Clone, Debug)]
#[table(name = user, public)]
pub struct User {
    /// Unique identifier for this user - this is their permanent ID
    #[primary_key]
    pub identity: Identity,
    
    /// Display name chosen by the user
    #[unique]
    pub username: String,
    
    /// Hashed password (never store passwords in plain text!)
    pub password_hash: String,
    
    /// Salt used for password hashing
    pub password_salt: String,
    
    /// Email address for account recovery
    pub email: Option<String>,
    
    /// When this account was created
    pub created_at: Timestamp,
    
    /// When the user last logged in
    pub last_login: Timestamp,
    
    /// Whether this account is currently active
    pub is_active: bool,
}

impl User {
    /// Find user by username
    pub fn filter_by_username(ctx: &ReducerContext, username: &str) -> Option<User> {
        ctx.db.user().iter().find(|user| user.username == username).cloned()
    }
    
    /// Find user by identity
    pub fn filter_by_identity(ctx: &ReducerContext, identity: &Identity) -> Option<User> {
        ctx.db.user().identity().find(identity).cloned()
    }
    
    /// Get all active users
    pub fn get_active_users(ctx: &ReducerContext) -> Vec<User> {
        ctx.db.user().iter().filter(|user| user.is_active).cloned().collect()
    }
    
    /// Update user's last login time
    pub fn update_last_login(ctx: &ReducerContext, identity: &Identity, timestamp: Timestamp) {
        if let Some(mut user) = Self::filter_by_identity(ctx, identity) {
            user.last_login = timestamp;
            ctx.db.user().identity().update(user);
        }
    }
}