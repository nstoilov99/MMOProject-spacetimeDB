//! Game session table definition

use spacetimedb::{table, Identity, Timestamp, ReducerContext, ConnectionId};

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

impl GameSession {
    /// Find session by identity
    pub fn filter_by_identity(ctx: &ReducerContext, identity: &Identity) -> Option<GameSession> {
        ctx.db.gamesession().identity().find(identity).cloned()
    }
    
    /// Get all active sessions
    pub fn get_all_sessions(ctx: &ReducerContext) -> Vec<GameSession> {
        ctx.db.gamesession().iter().cloned().collect()
    }
    
    /// Update session activity timestamp
    pub fn update_activity(ctx: &ReducerContext, identity: &Identity, timestamp: Timestamp) {
        if let Some(mut session) = Self::filter_by_identity(ctx, identity) {
            session.last_activity = timestamp;
            ctx.db.gamesession().identity().update(session);
        }
    }
    
    /// Create a new session
    pub fn create_session(
        ctx: &ReducerContext,
        identity: Identity,
        connection_id: Option<ConnectionId>,
        client_version: String,
        ip_address: String,
        timestamp: Timestamp
    ) {
        let session = GameSession {
            identity,
            connection_id,
            login_time: timestamp,
            last_activity: timestamp,
            client_version,
            ip_address,
        };
        
        ctx.db.gamesession().insert(session);
    }
    
    /// Get sessions that haven't been active since the cutoff time
    pub fn get_inactive_sessions(
        ctx: &ReducerContext,
        cutoff_time: Timestamp
    ) -> Vec<GameSession> {
        ctx.db.gamesession().iter()
            .filter(|session| session.last_activity < cutoff_time)
            .cloned()
            .collect()
    }
    
    /// Remove a session
    pub fn remove_session(ctx: &ReducerContext, identity: &Identity) {
        ctx.db.gamesession().identity().delete(identity);
    }
}