// server/src/lib.rs
use spacetimedb::{table, reducer, Identity, Timestamp, ReducerContext, ConnectionId, Table};

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

/// Active player in the game world
/// This represents a player who is currently online and in the game
#[derive(Clone, Debug)]
#[table(name = game_players, public)]
pub struct Player {
    /// Links to the User table - this is the same as User.identity
    #[primary_key]
    pub identity: Identity,
    
    /// Display name (copied from User for quick access)
    pub username: String,
    
    /// Player's position in 3D space
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    
    /// Player's rotation (yaw angle in degrees)
    pub rotation_yaw: f32,
    
    /// Game-specific stats
    pub level: u32,
    pub experience: u64,
    pub health: f32,
    pub max_health: f32,
    
    /// Online status
    pub is_online: bool,
    pub last_seen: Timestamp,
    
    /// Current zone or area
    pub current_zone: String,
}

/// Chat message storage
/// We store recent chat messages for players who join mid-conversation
#[derive(Clone, Debug)]
#[table(name = chatmessage, public)]
pub struct ChatMessage {
    /// Unique message ID
    #[primary_key]
    pub message_id: u64,
    
    /// Who sent this message
    pub sender_identity: Identity,
    pub sender_username: String,
    
    /// The actual message content
    pub message: String,
    
    /// Chat channel (global, guild, whisper, etc.)
    pub channel: String,
    
    /// When this message was sent
    pub timestamp: Timestamp,
}

/// Active game sessions
/// Tracks who is currently connected for cleanup and management
#[derive(Clone, Debug)]
#[table(name = gamesession, public)]
pub struct GameSession {
    /// User's identity
    #[primary_key]
    pub identity: Identity,
    
    /// Connection ID (if available)
    pub connection_id: Option<ConnectionId>,
    
    /// When they connected
    pub login_time: Timestamp,
    
    /// Last activity timestamp (for timeout detection)
    pub last_activity: Timestamp,
    
    /// Connection metadata
    pub client_version: String,
    pub ip_address: String,
}

/// Called when the SpacetimeDB module starts
#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    log::info!("MMO Server initializing...");
    log::info!("MMO Server initialized successfully");
}

/// Called when a client connects
#[reducer(client_connected)]
pub fn on_connect(ctx: &ReducerContext) {
    log::info!("Client connected: {:?}", ctx.sender);
}

/// Called when a client disconnects
#[reducer(client_disconnected)]
pub fn on_disconnect(ctx: &ReducerContext) {
    log::info!("Client disconnected: {:?}", ctx.sender);
    
    // Cleanup player state when they disconnect
    if let Some(player) = ctx.db.game_players().identity().find(&ctx.sender) {
        let mut updated_player = player.clone();
        updated_player.is_online = false;
        updated_player.last_seen = ctx.timestamp;
        ctx.db.game_players().identity().update(updated_player);
    }
    
    // Remove their game session
    ctx.db.gamesession().identity().delete(&ctx.sender);
}

/// Periodic cleanup of inactive sessions
/// This should be called regularly to remove stale sessions and mark players offline
#[reducer]
pub fn cleanup_inactive_sessions(ctx: &ReducerContext) -> Result<(), String> {
    let timeout_seconds = 300; // 5 minutes
    let cutoff_time = ctx.timestamp - std::time::Duration::from_secs(timeout_seconds);
    
    let mut cleaned_count = 0;
    
    // Find sessions that haven't been active recently
    let inactive_sessions: Vec<GameSession> = ctx.db.gamesession().iter()
        .filter(|session| session.last_activity < cutoff_time)
        .collect();
    
    for session in inactive_sessions {
        // Mark associated player as offline
        if let Some(player) = ctx.db.game_players().identity().find(&session.identity) {
            let mut updated_player = player.clone();
            updated_player.is_online = false;
            updated_player.last_seen = ctx.timestamp;
            ctx.db.game_players().identity().update(updated_player);
        }
        
        // Remove the expired session
        ctx.db.gamesession().identity().delete(&session.identity);
        cleaned_count += 1;
    }
    
    if cleaned_count > 0 {
        log::info!("Cleaned up {} inactive sessions", cleaned_count);
    }
    
    Ok(())
}

// Module organization - files are in modules/ folder but re-exported at top level
mod modules {
    pub mod auth;
    pub mod player;
    pub mod chat;
}

pub use modules::auth;
pub use modules::player;
pub use modules::chat;

// Helper functions for common database queries
impl User {
    pub fn filter_by_username(ctx: &ReducerContext, username: &str) -> Option<User> {
        ctx.db.user().iter().find(|user| user.username == username)
    }
    
    pub fn filter_by_identity(ctx: &ReducerContext, identity: &Identity) -> Option<User> {
        if let Some(user) = ctx.db.user().identity().find(identity) {
            Some(user.clone())
        } else {
            None
        }
    }
}

impl Player {
    pub fn get_online_players(ctx: &ReducerContext) -> Vec<Player> {
        ctx.db.game_players().iter().filter(|p| p.is_online).collect()
    }
    
    pub fn get_players_in_zone(ctx: &ReducerContext, zone: &str) -> Vec<Player> {
        ctx.db.game_players().iter()
            .filter(|p| p.is_online && p.current_zone == zone)
            .collect()
    }
    
    pub fn filter_by_identity(ctx: &ReducerContext, identity: &Identity) -> Option<Player> {
        if let Some(player) = ctx.db.game_players().identity().find(identity) {
            Some(player.clone())
        } else {
            None
        }
    }
}

impl GameSession {
    pub fn filter_by_identity(ctx: &ReducerContext, identity: &Identity) -> Option<GameSession> {
        if let Some(session) = ctx.db.gamesession().identity().find(identity) {
            Some(session.clone())
        } else {
            None
        }
    }
}