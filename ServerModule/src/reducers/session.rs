//! Session management reducers

use spacetimedb::{reducer, ReducerContext};
use shared_module::*;
use crate::tables::*;

/// Heartbeat to keep session alive
#[reducer]
pub fn heartbeat(ctx: &ReducerContext) -> Result<(), String> {
    // Update session activity
    GameSession::update_activity(ctx, &ctx.sender, ctx.timestamp);
    
    // Also update player last seen time if they're in game
    if let Some(player) = Player::filter_by_identity(ctx, &ctx.sender) {
        if player.is_online {
            Player::set_online_status(ctx, &ctx.sender, true, ctx.timestamp);
        }
    }
    
    Ok(())
}

/// Get session information for the current user
#[reducer]
pub fn get_session_info(ctx: &ReducerContext) -> Result<Option<GameSession>, String> {
    let session = GameSession::filter_by_identity(ctx, &ctx.sender);
    Ok(session)
}

/// Get all online players (for admin/debug purposes)
#[reducer]
pub fn get_online_players_count(ctx: &ReducerContext) -> Result<u32, String> {
    // In a real implementation, you'd check for admin privileges
    let count = Player::get_online_players(ctx).len() as u32;
    Ok(count)
}

/// Force disconnect a user (admin function)
#[reducer]
pub fn force_disconnect_user(
    ctx: &ReducerContext,
    target_username: String
) -> Result<(), String> {
    // In a real implementation, you'd check for admin privileges here
    
    // Find the user by username
    let user = User::filter_by_username(ctx, &target_username)
        .ok_or("User not found")?;
    
    // Remove their session
    GameSession::remove_session(ctx, &user.identity);
    
    // Mark them as offline
    Player::set_online_status(ctx, &user.identity, false, ctx.timestamp);
    
    log::info!("Force disconnected user: {}", target_username);
    Ok(())
}

/// Update client version information
#[reducer]
pub fn update_client_version(
    ctx: &ReducerContext,
    client_version: String
) -> Result<(), String> {
    if let Some(mut session) = GameSession::filter_by_identity(ctx, &ctx.sender) {
        session.client_version = client_version;
        session.last_activity = ctx.timestamp;
        ctx.db.gamesession().identity().update(session);
    }
    
    Ok(())
}