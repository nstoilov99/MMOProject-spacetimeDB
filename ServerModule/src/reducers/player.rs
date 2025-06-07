//! Player-related reducers

use spacetimedb::{reducer, ReducerContext};
use shared_module::*;
use crate::tables::*;

/// Join the game world as a player
#[reducer]
pub fn join_game(
    ctx: &ReducerContext,
    starting_zone: String
) -> Result<(), String> {
    // Verify the user is logged in
    let _session = GameSession::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Must be logged in to join game")?;
    
    // Get user information
    let user = User::filter_by_identity(ctx, &ctx.sender)
        .ok_or("User not found")?;
    
    // Update session activity
    GameSession::update_activity(ctx, &ctx.sender, ctx.timestamp);
    
    // Create or update player
    if let Some(_existing_player) = Player::filter_by_identity(ctx, &ctx.sender) {
        // Player returning to game - update their status
        Player::set_online_status(ctx, &ctx.sender, true, ctx.timestamp);
        
        // Update their zone if different
        if let Some(mut player) = Player::filter_by_identity(ctx, &ctx.sender) {
            player.current_zone = starting_zone;
            ctx.db.player().identity().update(player);
        }
        
        log::info!("Player returned to game: {}", user.username);
    } else {
        // New player - set up starting stats
        Player::create_player(
            ctx,
            ctx.sender,
            user.username.clone(),
            starting_zone,
            ctx.timestamp
        );
        
        log::info!("New player joined game: {}", user.username);
    }
    
    Ok(())
}

/// Update player's position and rotation
#[reducer]
pub fn update_player_position(
    ctx: &ReducerContext,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32
) -> Result<(), String> {
    // Find the player
    let player = Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Player not found")?;
    
    // Validate movement using shared utility
    validate_movement(
        player.position_x, player.position_y, player.position_z,
        x, y, z,
        MAX_MOVEMENT_DISTANCE
    )?;
    
    // Update position
    Player::update_position(ctx, &ctx.sender, x, y, z, yaw, ctx.timestamp);
    
    // Update session activity
    GameSession::update_activity(ctx, &ctx.sender, ctx.timestamp);
    
    Ok(())
}

/// Get all players in the same zone
#[reducer]
pub fn get_players_in_zone(
    ctx: &ReducerContext,
    zone: String
) -> Result<Vec<Player>, String> {
    // Verify the requesting player exists
    Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Player not found")?;
    
    let players = Player::get_players_in_zone(ctx, &zone);
    Ok(players)
}

/// Leave the game (but don't logout)
#[reducer]
pub fn leave_game(ctx: &ReducerContext) -> Result<(), String> {
    Player::set_online_status(ctx, &ctx.sender, false, ctx.timestamp);
    Ok(())
}

/// Update player stats (level, experience, health, etc.)
#[reducer]
pub fn update_player_stats(
    ctx: &ReducerContext,
    level: Option<u32>,
    experience: Option<u64>,
    health: Option<f32>,
    max_health: Option<f32>
) -> Result<(), String> {
    let mut player = Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Player not found")?;
    
    // Update only the provided stats
    if let Some(new_level) = level {
        player.level = new_level;
    }
    
    if let Some(new_experience) = experience {
        player.experience = new_experience;
    }
    
    if let Some(new_health) = health {
        // Ensure health doesn't exceed max health
        player.health = new_health.min(player.max_health);
    }
    
    if let Some(new_max_health) = max_health {
        player.max_health = new_max_health;
        // Adjust current health if it now exceeds max
        if player.health > player.max_health {
            player.health = player.max_health;
        }
    }
    
    player.last_seen = ctx.timestamp;
    ctx.db.player().identity().update(player);
    
    // Update session activity
    GameSession::update_activity(ctx, &ctx.sender, ctx.timestamp);
    
    Ok(())
}

/// Change player's current zone
#[reducer]
pub fn change_zone(
    ctx: &ReducerContext,
    new_zone: String,
    spawn_x: f32,
    spawn_y: f32,
    spawn_z: f32
) -> Result<(), String> {
    let mut player = Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Player not found")?;
    
    // Validate spawn position
    if !spawn_x.is_finite() || !spawn_y.is_finite() || !spawn_z.is_finite() {
        return Err("Invalid spawn position".to_string());
    }
    
    // Update zone and position
    player.current_zone = new_zone.clone();
    player.position_x = spawn_x;
    player.position_y = spawn_y;
    player.position_z = spawn_z;
    player.last_seen = ctx.timestamp;
    
    ctx.db.player().identity().update(player);
    
    // Update session activity
    GameSession::update_activity(ctx, &ctx.sender, ctx.timestamp);
    
    log::info!("Player {:?} changed to zone: {}", ctx.sender, new_zone);
    Ok(())
}