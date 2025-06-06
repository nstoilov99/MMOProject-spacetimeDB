// server/src/player.rs
use crate::*;

/// Join the game world as a player
/// This creates or updates the player's presence in the game
#[spacetimedb(reducer)]
pub fn join_game(ctx: ReducerContext, starting_zone: String) -> Result<(), String> {
    // Verify the user is logged in
    let session = GameSession::filter_by_identity(&ctx.sender)
        .ok_or("Must be logged in to join game")?;
    
    let user = User::filter_by_username("") // We need to find user by identity, not username
        .ok_or("User not found")?;
    
    // Actually, let's find the user by identity
    let user = User::iter().find(|u| u.identity == ctx.sender)
        .ok_or("User not found")?;
    
    // Update session activity
    let mut updated_session = session;
    updated_session.last_activity = Timestamp::now();
    GameSession::update_by_identity(&ctx.sender, updated_session);
    
    let now = Timestamp::now();
    
    // Create or update player
    if let Some(mut existing_player) = Player::filter_by_identity(&ctx.sender) {
        // Player returning to game
        existing_player.address = ctx.address.unwrap_or_default();
        existing_player.is_online = true;
        existing_player.last_seen = now;
        existing_player.current_zone = starting_zone;
        Player::update_by_identity(&ctx.sender, existing_player);
        
        log::info!("Player returned to game: {}", user.username);
    } else {
        // New player - set up starting stats
        Player::insert(Player {
            identity: ctx.sender,
            address: ctx.address.unwrap_or_default(),
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
            last_seen: now,
            current_zone: starting_zone,
        });
        
        log::info!("New player joined game: {}", user.username);
    }
    
    Ok(())
}

/// Update player's position and rotation
/// This is called frequently as players move around
#[spacetimedb(reducer)]
pub fn update_player_position(
    ctx: ReducerContext,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32
) -> Result<(), String> {
    let mut player = Player::filter_by_identity(&ctx.sender)
        .ok_or("Player not found")?;
    
    // Validate position (prevent teleporting too far too fast)
    let max_distance_per_update = 50.0; // Maximum movement per update
    let distance_moved = ((player.position_x - x).powi(2) + 
                         (player.position_y - y).powi(2) + 
                         (player.position_z - z).powi(2)).sqrt();
    
    if distance_moved > max_distance_per_update {
        return Err("Invalid movement detected".to_string());
    }
    
    // Update position
    player.position_x = x;
    player.position_y = y;
    player.position_z = z;
    player.rotation_yaw = yaw;
    player.last_seen = Timestamp::now();
    
    Player::update_by_identity(&ctx.sender, player);
    
    // Update session activity
    if let Some(mut session) = GameSession::filter_by_identity(&ctx.sender) {
        session.last_activity = Timestamp::now();
        GameSession::update_by_identity(&ctx.sender, session);
    }
    
    Ok(())
}

/// Get all players in the same zone
#[spacetimedb(reducer)]
pub fn get_players_in_zone(ctx: ReducerContext, zone: String) -> Result<Vec<Player>, String> {
    // Verify the requesting player exists
    Player::filter_by_identity(&ctx.sender)
        .ok_or("Player not found")?;
    
    let players: Vec<Player> = Player::iter()
        .filter(|p| p.is_online && p.current_zone == zone)
        .collect();
    
    Ok(players)
}

/// Leave the game (but don't logout)
#[spacetimedb(reducer)]
pub fn leave_game(ctx: ReducerContext) -> Result<(), String> {
    if let Some(mut player) = Player::filter_by_identity(&ctx.sender) {
        player.is_online = false;
        player.last_seen = Timestamp::now();
        Player::update_by_identity(&ctx.sender, player);
    }
    
    Ok(())
}