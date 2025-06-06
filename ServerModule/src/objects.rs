//! Object management system
//! 
//! This handles the lifecycle of all game objects - creating them,
//! updating them, and cleaning them up when no longer needed.

use spacetimedb::{table, ReducerContext, Identity, Timestamp};
use shared_module::*;

/// The main object table - every game object has an entry here
/// This is like the master registry for everything in your world
#[derive(Clone, Debug)]
#[table(name = game_objects, public)]
pub struct GameObject {
    #[primary_key]
    pub object_id: u64,
    pub class_name: String,
    pub owner_identity: Identity,
    pub zone_id: u32,
    pub is_active: bool,
    pub created_at: Timestamp,
    pub last_updated: Timestamp,
}

/// Spawn a player character in the specified zone
/// This creates both the main object record and sets up initial properties
pub fn spawn_player_character(
    ctx: &ReducerContext,
    zone_id: u32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
) -> Result<u64, String> {
    // First, verify the player is logged in and the zone exists
    let connection = ctx.db.connections().identity().find(&ctx.sender)
        .ok_or("Must be logged in to spawn character")?;
    
    if !connection.is_online {
        return Err("Must be online to spawn character".to_string());
    }
    
    let zone = ctx.db.zones().zone_id().find(&zone_id)
        .ok_or("Zone not found")?;
    
    if !zone.is_active {
        return Err("Zone is not active".to_string());
    }
    
    // Generate a unique ID for this object
    let object_id = generate_unique_id(ctx);
    
    // Create the main game object record
    ctx.db.game_objects().insert(GameObject {
        object_id,
        class_name: "PlayerCharacter".to_string(),
        owner_identity: ctx.sender,
        zone_id,
        is_active: true,
        created_at: ctx.timestamp,
        last_updated: ctx.timestamp,
    });
    
    // Set up initial properties for this player character
    // These properties will be automatically replicated to relevant clients
    crate::properties::set_object_property_internal(
        ctx, 
        object_id, 
        "Position", 
        &format!("{},{},{}", position_x, position_y, position_z)
    )?;
    
    crate::properties::set_object_property_internal(ctx, object_id, "Health", "100")?;
    crate::properties::set_object_property_internal(ctx, object_id, "MaxHealth", "100")?;
    crate::properties::set_object_property_internal(ctx, object_id, "Level", "1")?;
    
    log::info!("Player character spawned: {} in zone {}", object_id, zone_id);
    Ok(object_id)
}

/// Handle player movement RPC calls
/// This validates and processes requests to move a player character
pub fn handle_move_player_rpc(
    ctx: &ReducerContext,
    target_object: Option<u64>,
    arguments_json: &str,
) -> Result<String, String> {
    let object_id = target_object.ok_or("Target object required for move_player")?;
    
    // Parse the movement arguments from JSON
    let args: serde_json::Value = serde_json::from_str(arguments_json)
        .map_err(|_| "Invalid JSON arguments")?;
    
    let x = args["x"].as_f64().ok_or("Missing x coordinate")? as f32;
    let y = args["y"].as_f64().ok_or("Missing y coordinate")? as f32;
    let z = args["z"].as_f64().ok_or("Missing z coordinate")? as f32;
    
    // Verify the object exists and the caller owns it
    let object = ctx.db.game_objects().object_id().find(&object_id)
        .ok_or("Object not found")?;
    
    if object.owner_identity != ctx.sender {
        return Err("You don't own this object".to_string());
    }
    
    // Update the position property
    // This will automatically replicate to all relevant clients
    let new_position = format!("{},{},{}", x, y, z);
    crate::properties::set_object_property_internal(ctx, object_id, "Position", &new_position)?;
    
    Ok(format!("Player moved to ({}, {}, {})", x, y, z))
}

/// Generate a unique ID for new objects
/// This uses a combination of sender identity, timestamp, and randomness
/// to ensure IDs are unique across the entire system
pub fn generate_unique_id(ctx: &ReducerContext) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    format!("{:?}", ctx.sender).hash(&mut hasher);
    format!("{:?}", ctx.timestamp).hash(&mut hasher);
    
    // Add some additional entropy if available
    if let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        now.subsec_nanos().hash(&mut hasher);
    }
    
    hasher.finish()
}