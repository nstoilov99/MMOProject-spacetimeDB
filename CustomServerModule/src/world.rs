//! World generation and management

use spacetimedb::{reducer, ReducerContext};
use shared_module::*;
use crate::*;

/// Initialize the world generation system
pub fn initialize_world_generator(_ctx: &ReducerContext) -> Result<(), String> {
    log::info!("World generation system initialized");
    Ok(())
}

/// Generate a new world chunk
#[reducer]
pub fn generate_world_chunk(
    ctx: &ReducerContext,
    x: i32,
    y: i32,
    z: i32
) -> Result<u64, String> {
    // Check if chunk already exists
    let existing_chunk = ctx.db.world_chunks().iter()
        .find(|chunk| chunk.x == x && chunk.y == y && chunk.z == z);
    
    if existing_chunk.is_some() {
        return Err("Chunk already exists".to_string());
    }
    
    // Generate unique chunk ID
    let chunk_id = generate_unique_id(&ctx.sender, ctx.timestamp);
    
    // Determine biome type based on coordinates
    let biome_type = determine_biome_type(x, y, z);
    
    // Generate chunk data (placeholder - use noise library for real generation)
    let chunk_data = generate_chunk_data(x, y, z, &biome_type);
    
    // Create the chunk
    ctx.db.world_chunks().insert(WorldChunk {
        chunk_id,
        x,
        y,
        z,
        biome_type,
        generated: true,
        data_compressed: chunk_data,
    });
    
    log::info!("Generated world chunk at ({}, {}, {})", x, y, z);
    Ok(chunk_id)
}

/// Determine biome type based on coordinates
fn determine_biome_type(x: i32, y: i32, _z: i32) -> String {
    // Simple biome generation based on distance from origin
    let distance = ((x * x + y * y) as f64).sqrt();
    
    match distance {
        d if d < 10.0 => "grasslands".to_string(),
        d if d < 50.0 => "forest".to_string(),
        d if d < 100.0 => "mountains".to_string(),
        _ => "desert".to_string(),
    }
}

/// Generate chunk data (placeholder implementation)
fn generate_chunk_data(_x: i32, _y: i32, _z: i32, _biome_type: &str) -> Vec<u8> {
    // In a real implementation, you would:
    // 1. Use noise::Fbm for terrain generation
    // 2. Generate block types based on biome
    // 3. Add structures, ores, etc.
    // 4. Compress the data
    
    // For now, return placeholder data
    vec![0u8; 1024] // 1KB of placeholder chunk data
}