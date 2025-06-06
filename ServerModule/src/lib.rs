//! ServerModule: Core game logic and state management
//! 
//! This module runs inside SpacetimeDB and handles all server-side
//! game logic, object management, and player interactions.
//! Think of this as the "brain" of your MMO.

use spacetimedb::{table, reducer, ReducerContext, Identity, Timestamp};
use shared_module::*;

// Re-export shared types for convenience
pub use shared_module::*;

// Import our sub-modules
pub mod objects;
pub mod zones;
pub mod connections;
pub mod properties;

// Re-export important types from sub-modules
pub use objects::*;
pub use zones::*;
pub use connections::*;
pub use properties::*;

/// Initialize the server when the SpacetimeDB module starts
/// This is like the grand opening ceremony for your virtual world
#[reducer(init)]
pub fn initialize_server(ctx: &ReducerContext) {
    log::info!("MMO Server initializing...");
    
    // Set up the default world structure
    zones::create_default_zones(ctx);
    
    log::info!("MMO Server initialized successfully!");
}

/// Handle new client connections to SpacetimeDB
/// This is like someone walking up to the front gate of your world
#[reducer(client_connected)]
pub fn on_client_connected(ctx: &ReducerContext) {
    log::info!("Client connected: {:?}", ctx.sender);
    
    // At this point, we just know someone connected to SpacetimeDB
    // They haven't logged in yet, so we don't create any game objects
    // This is like someone arriving at the parking lot but not yet
    // checking in at the front desk
}

/// Handle client disconnections
/// This is like someone leaving the building - we need to clean up after them
#[reducer(client_disconnected)]  
pub fn on_client_disconnected(ctx: &ReducerContext) {
    log::info!("Client disconnected: {:?}", ctx.sender);
    
    // Clean up all objects and state associated with this client
    // This ensures that disconnected players don't leave ghost objects
    // floating around in your world
    connections::cleanup_disconnected_client(ctx, &ctx.sender);
}

/// User registration - create a new account
/// This is like filling out an application to join your virtual world
#[reducer]
pub fn register_user(
    ctx: &ReducerContext,
    username: String,
    password_hash: String, // Always hash passwords, never store plain text!
) -> Result<(), String> {
    connections::register_new_user(ctx, username, password_hash)
}

/// User login - authenticate and mark as online
/// This is like showing your ID card at the front desk
#[reducer]
pub fn login_user(
    ctx: &ReducerContext,
    username: String,
    password_hash: String,
) -> Result<(), String> {
    connections::authenticate_user(ctx, username, password_hash)
}

/// Spawn a player character in the world
/// This is like a new resident moving into your virtual neighborhood
#[reducer]
pub fn spawn_player_character(
    ctx: &ReducerContext,
    zone_id: u32,
    position_x: f32,
    position_y: f32,
    position_z: f32,
) -> Result<u64, String> {
    objects::spawn_player_character(ctx, zone_id, position_x, position_y, position_z)
}

/// Update an object's property
/// This is like updating someone's address or phone number in the city records
#[reducer]
pub fn set_object_property(
    ctx: &ReducerContext,
    object_id: u64,
    property_name: String,
    value_json: String,
) -> Result<(), String> {
    properties::set_object_property(ctx, object_id, property_name, value_json)
}

/// Handle RPC function calls
/// This is like a telephone operator routing calls to the right department
#[reducer]
pub fn call_rpc_function(
    ctx: &ReducerContext,
    function_name: String,
    target_object: Option<u64>,
    arguments_json: String,
) -> Result<String, String> {
    // Route the RPC call to the appropriate handler
    // This allows for modular, extensible game functionality
    match function_name.as_str() {
        "move_player" => objects::handle_move_player_rpc(ctx, target_object, &arguments_json),
        "chat_message" => connections::handle_chat_message_rpc(ctx, &arguments_json),
        // Add more RPC handlers here as you build new features
        _ => Err(format!("Unknown RPC function: {}", function_name)),
    }
}