//! Player table definition

use spacetimedb::{table, Identity, Timestamp, ReducerContext};

/// Active player in the game world
/// This represents a player who is currently online and in the game
#[derive(Clone, Debug)]
#[table(name = player, public)]
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

impl Player {
    /// Get all online players
    pub fn get_online_players(ctx: &ReducerContext) -> Vec<Player> {
        ctx.db.game_players().iter().filter(|p| p.is_online).cloned().collect()
    }
    
    /// Get all players in a specific zone
    pub fn get_players_in_zone(ctx: &ReducerContext, zone: &str) -> Vec<Player> {
        ctx.db.game_players().iter()
            .filter(|p| p.is_online && p.current_zone == zone)
            .cloned()
            .collect()
    }
    
    /// Find player by identity
    pub fn filter_by_identity(ctx: &ReducerContext, identity: &Identity) -> Option<Player> {
        ctx.db.game_players().identity().find(identity).cloned()
    }
    
    /// Update player position
    pub fn update_position(
        ctx: &ReducerContext,
        identity: &Identity,
        x: f32,
        y: f32,
        z: f32,
        yaw: f32,
        timestamp: Timestamp
    ) {
        if let Some(mut player) = Self::filter_by_identity(ctx, identity) {
            player.position_x = x;
            player.position_y = y;
            player.position_z = z;
            player.rotation_yaw = yaw;
            player.last_seen = timestamp;
            ctx.db.game_players().identity().update(player);
        }
    }
    
    /// Set player online status
    pub fn set_online_status(
        ctx: &ReducerContext,
        identity: &Identity,
        is_online: bool,
        timestamp: Timestamp
    ) {
        if let Some(mut player) = Self::filter_by_identity(ctx, identity) {
            player.is_online = is_online;
            player.last_seen = timestamp;
            ctx.db.game_players().identity().update(player);
        }
    }
    
    /// Create a new player
    pub fn create_player(
        ctx: &ReducerContext,
        identity: Identity,
        username: String,
        starting_zone: String,
        timestamp: Timestamp
    ) {
        let player = Player {
            identity,
            username,
            position_x: 0.0,
            position_y: 0.0,
            position_z: 0.0,
            rotation_yaw: 0.0,
            level: 1,
            experience: 0,
            health: 100.0,
            max_health: 100.0,
            is_online: true,
            last_seen: timestamp,
            current_zone: starting_zone,
        };
        
        ctx.db.game_players().insert(player);
    }
}