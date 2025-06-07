//! Session management utilities

use spacetimedb::{ReducerContext, Identity};
use shared_module::*;
use crate::tables::*;

/// Handle client connection
pub fn handle_client_connected(ctx: &ReducerContext) {
    // For now, just log the connection
    // More complex initialization could be added here
    log::debug!("Client connected: {:?}", ctx.sender);
}

/// Handle client disconnection and cleanup
pub fn handle_client_disconnected(ctx: &ReducerContext) {
    cleanup_client_state(ctx, &ctx.sender);
}

/// Clean up all state associated with a client
pub fn cleanup_client_state(ctx: &ReducerContext, identity: &Identity) {
    // Mark player as offline if they exist
    if let Some(player) = Player::filter_by_identity(ctx, identity) {
        if player.is_online {
            Player::set_online_status(ctx, identity, false, ctx.timestamp);
            log::info!("Player {} marked as offline due to disconnection", player.username);
        }
    }
    
    // Remove their game session
    if GameSession::filter_by_identity(ctx, identity).is_some() {
        GameSession::remove_session(ctx, identity);
        log::debug!("Removed session for disconnected client: {:?}", identity);
    }
}

/// Cleanup inactive sessions based on timeout
pub fn cleanup_inactive_sessions(ctx: &ReducerContext) -> Result<(), String> {
    let timeout_duration = std::time::Duration::from_secs(INACTIVITY_TIMEOUT_SECONDS);
    let cutoff_time = ctx.timestamp - timeout_duration;
    
    let inactive_sessions = GameSession::get_inactive_sessions(ctx, cutoff_time);
    let mut cleaned_count = 0;
    
    for session in inactive_sessions {
        // Mark associated player as offline
        Player::set_online_status(ctx, &session.identity, false, ctx.timestamp);
        
        // Remove the expired session
        GameSession::remove_session(ctx, &session.identity);
        cleaned_count += 1;
    }
    
    if cleaned_count > 0 {
        log::info!("Cleaned up {} inactive sessions", cleaned_count);
    }
    
    Ok(())
}

/// Check if a user is currently online
pub fn is_user_online(ctx: &ReducerContext, identity: &Identity) -> bool {
    if let Some(player) = Player::filter_by_identity(ctx, identity) {
        player.is_online
    } else {
        false
    }
}

/// Get session duration for a user
pub fn get_session_duration(ctx: &ReducerContext, identity: &Identity) -> Option<std::time::Duration> {
    if let Some(session) = GameSession::filter_by_identity(ctx, identity) {
        Some(ctx.timestamp.duration_since(session.login_time))
    } else {
        None
    }
}

/// Update session with new activity
pub fn update_session_activity(ctx: &ReducerContext, identity: &Identity) {
    GameSession::update_activity(ctx, identity, ctx.timestamp);
}