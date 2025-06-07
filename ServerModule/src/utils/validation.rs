//! Server-side validation utilities

use spacetimedb::{ReducerContext, Identity};
use shared_module::*;
use crate::tables::*;

/// Validate that a user is properly authenticated
pub fn validate_authenticated_user(ctx: &ReducerContext) -> Result<User, String> {
    // Check if user exists
    let user = User::filter_by_identity(ctx, &ctx.sender)
        .ok_or("User not found")?;
    
    // Check if user is active
    if !user.is_active {
        return Err("Account is suspended".to_string());
    }
    
    // Check if they have an active session
    GameSession::filter_by_identity(ctx, &ctx.sender)
        .ok_or("No active session found")?;
    
    Ok(user)
}

/// Validate that a player is in the game
pub fn validate_player_in_game(ctx: &ReducerContext) -> Result<Player, String> {
    let player = Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Player not found")?;
    
    if !player.is_online {
        return Err("Player is not online".to_string());
    }
    
    Ok(player)
}

/// Validate player permissions for zone-specific actions
pub fn validate_zone_access(
    ctx: &ReducerContext,
    zone: &str
) -> Result<Player, String> {
    let player = validate_player_in_game(ctx)?;
    
    // For now, all zones are accessible
    // In a more complex system, you might check zone restrictions
    if zone.is_empty() {
        return Err("Invalid zone".to_string());
    }
    
    Ok(player)
}

/// Validate that a target player exists and is online
pub fn validate_target_player(
    ctx: &ReducerContext,
    target_username: &str
) -> Result<Player, String> {
    let target = ctx.db.game_players().iter()
        .find(|p| p.username == target_username)
        .ok_or("Target player not found")?;
    
    if !target.is_online {
        return Err("Target player is not online".to_string());
    }
    
    Ok(target.clone())
}

/// Validate admin permissions (placeholder implementation)
pub fn validate_admin_permissions(ctx: &ReducerContext) -> Result<(), String> {
    // In a real implementation, you'd check if the user has admin rights
    // For now, we'll just check if they're a valid user
    validate_authenticated_user(ctx)?;
    
    // TODO: Implement proper admin role checking
    // This is just a placeholder
    Ok(())
}

/// Validate rate limiting for actions
pub fn validate_rate_limit(
    _ctx: &ReducerContext,
    _action_type: &str
) -> Result<(), String> {
    // TODO: Implement proper rate limiting
    // This would track action counts per user per time window
    // For now, we'll just allow everything
    Ok(())
}

/// Validate game world bounds
pub fn validate_world_position(x: f32, y: f32, z: f32) -> Result<(), String> {
    // Check for valid numeric values
    if !x.is_finite() || !y.is_finite() || !z.is_finite() {
        return Err("Invalid position coordinates".to_string());
    }
    
    // Check world bounds (adjust these values for your game world)
    const WORLD_MIN: f32 = -10000.0;
    const WORLD_MAX: f32 = 10000.0;
    
    if x < WORLD_MIN || x > WORLD_MAX ||
       y < WORLD_MIN || y > WORLD_MAX ||
       z < WORLD_MIN || z > WORLD_MAX {
        return Err("Position outside world bounds".to_string());
    }
    
    Ok(())
}

/// Validate chat channel access
pub fn validate_chat_channel_access(
    ctx: &ReducerContext,
    channel: &str
) -> Result<(), String> {
    let _player = validate_player_in_game(ctx)?;
    
    match channel {
        "global" => Ok(()), // Everyone can access global
        "zone" => Ok(()),   // Everyone can access zone chat
        "guild" => {
            // TODO: Check if player is in a guild
            Ok(())
        },
        "party" => {
            // TODO: Check if player is in a party
            Ok(())
        },
        channel if channel.starts_with("whisper:") => {
            // Whisper channels are always accessible
            Ok(())
        },
        _ => Err("Invalid chat channel".to_string())
    }
}