//! CustomServerModule: Game-specific server features
//! 
//! This module extends the base ServerModule with custom MMO features
//! like procedural world generation, advanced AI, and custom game mechanics.

use spacetimedb::{table, reducer, ReducerContext};
use shared_module::*;
use server_module::*;

// Import our custom modules
pub mod world;
pub mod ai;
pub mod mechanics;

// Re-export custom functionality
pub use world::*;
pub use ai::*;
pub use mechanics::*;

/// Initialize custom server features
#[reducer]
pub fn initialize_custom_features(ctx: &ReducerContext) -> Result<(), String> {
    log::info!("Initializing custom MMO features...");
    
    // Initialize world generation
    world::initialize_world_generator(ctx)?;
    
    // Initialize AI systems
    ai::initialize_ai_systems(ctx)?;
    
    // Initialize custom mechanics
    mechanics::initialize_game_mechanics(ctx)?;
    
    log::info!("Custom MMO features initialized successfully!");
    Ok(())
}

/// Custom table for world chunks
#[derive(Clone, Debug)]
#[table(name = world_chunks, public)]
pub struct WorldChunk {
    #[primary_key]
    pub chunk_id: u64,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub biome_type: String,
    pub generated: bool,
    pub data_compressed: Vec<u8>, // Compressed chunk data
}

/// Custom table for NPCs
#[derive(Clone, Debug)]  
#[table(name = npcs, public)]
pub struct NPC {
    #[primary_key]
    pub npc_id: u64,
    pub name: String,
    pub npc_type: String,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub zone_id: u32,
    pub health: f32,
    pub max_health: f32,
    pub ai_state: String,
    pub respawn_time: spacetimedb::Timestamp,
}