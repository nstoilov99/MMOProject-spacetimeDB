// server/src/modules/player.rs
use crate::*;
use spacetimedb::{reducer, ReducerContext};

/// Join the game world as a player
/// This creates or updates the player's presence in the game
#[reducer]
pub fn join_game(
    ctx: &ReducerContext,
    starting_zone: String
) -> Result<(), String> {
    // Verify the user is logged in
    let session = GameSession::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Must be logged in to join game")?;
    
    // Get user information
    let user = User::filter_by_identity(ctx, &ctx.sender)
        .ok_or("User not found")?;
    
    // Update session activity
    let mut updated_session = session;
    updated_session.last_activity = ctx.timestamp;
    ctx.db.gamesession().identity().update(updated_session);
    
    // Create or update player
    if let Some(existing_player) = Player::filter_by_identity(ctx, &ctx.sender) {
        // Player returning to game - update their status
        let mut updated_player = existing_player;
        updated_player.is_online = true;
        updated_player.last_seen = ctx.timestamp;
        updated_player.current_zone = starting_zone;
        ctx.db.game_players().identity().update(updated_player);
        
        log::info!("Player returned to game: {}", user.username);
    } else {
        // New player - set up starting stats
        ctx.db.game_players().insert(Player {
            identity: ctx.sender,
            username: user.username.clone(),
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,
            rotation_yaw: 0.0,
            level: 1,
            experience: 0,
            health: 100.0,
            max_health: 100.0,
            is_online: true,
            last_seen: ctx.timestamp,
            current_zone: starting_zone,
        });
        
        log::info!("New player joined game: {}", user.username);
    }
    
    Ok(())
}

/// Update player's position and rotation
/// This is called frequently as players move around
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
    
    // Validate position to prevent cheating (teleporting too far too fast)
    let max_distance_per_update = 50.0; // Maximum movement per update in game units
    let distance_moved = ((player.position_x - x).powi(2) + 
                         (player.position_y - y).powi(2) + 
                         (player.position_z - z).powi(2)).sqrt();
    
    if distance_moved > max_distance_per_update {
        return Err("Invalid movement detected".to_string());
    }
    
    // Update position
    let mut updated_player = player;
    updated_player.position_x = x;
    updated_player.position_y = y;
    updated_player.position_z = z;
    updated_player.rotation_yaw = yaw;
    updated_player.last_seen = ctx.timestamp;
    
    ctx.db.game_players().identity().update(updated_player);
    
    // Update session activity to show the player is still active
    if let Some(session) = GameSession::filter_by_identity(ctx, &ctx.sender) {
        let mut updated_session = session;
        updated_session.last_activity = ctx.timestamp;
        ctx.db.gamesession().identity().update(updated_session);
    }
    
    Ok(())
}

/// Helper function to get all players in the same zone
/// Note: This is not a reducer - clients should query the database directly using:
/// SELECT * FROM game_players WHERE current_zone = ? AND is_online = true
pub fn get_players_in_zone_helper(
    ctx: &ReducerContext,
    zone: String
) -> Result<Vec<Player>, String> {
    // Verify the requesting player exists
    Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Player not found")?;
    
    // Get players from the specified zone
    let players: Vec<Player> = ctx.db.game_players().iter()
        .filter(|p| p.is_online && p.current_zone == zone)
        .collect();
    
    Ok(players)
}

/// Leave the game (but don't logout from the system)
/// This marks the player as offline but keeps their session active
#[reducer]
pub fn leave_game(ctx: &ReducerContext) -> Result<(), String> {
    // Find and update player status
    if let Some(player) = Player::filter_by_identity(ctx, &ctx.sender) {
        let mut updated_player = player;
        updated_player.is_online = false;
        updated_player.last_seen = ctx.timestamp;
        ctx.db.game_players().identity().update(updated_player);
    }
    
    Ok(())
}