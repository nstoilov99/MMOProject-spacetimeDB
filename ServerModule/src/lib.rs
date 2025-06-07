//! ServerModule: Core game logic and state management
//! 
//! This module runs inside SpacetimeDB and handles all server-side
//! game logic, object management, and player interactions.

use spacetimedb::{reducer, ReducerContext};
use shared_module::*;

// Import our organized modules
pub mod reducers;
pub mod utils;

// Re-export table types for convenience
pub use tables::*;

// Re-export reducers
pub use reducers::*;

/// Initialize the server when the SpacetimeDB module starts
#[reducer(init)]
pub fn initialize_server(_ctx: &ReducerContext) {
    log::info!("MMO Server initializing...");
    log::info!("MMO Server initialized successfully!");
}

/// Handle new client connections to SpacetimeDB
#[reducer(client_connected)]
pub fn on_client_connected(ctx: &ReducerContext) {
    log::info!("Client connected: {:?}", ctx.sender);
    
    // Initialize any per-client state if needed
    utils::session::handle_client_connected(ctx);
}

/// Handle client disconnections
#[reducer(client_disconnected)]  
pub fn on_client_disconnected(ctx: &ReducerContext) {
    log::info!("Client disconnected: {:?}", ctx.sender);
    
    // Clean up all state associated with this client
    utils::session::handle_client_disconnected(ctx);
}

/// Periodic cleanup of inactive sessions
#[reducer]
pub fn cleanup_inactive_sessions(ctx: &ReducerContext) -> Result<(), String> {
    utils::session::cleanup_inactive_sessions(ctx)
}