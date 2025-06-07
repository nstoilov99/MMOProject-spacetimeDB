//! Client connection management

use shared_module::*;
use crate::bridge::*;

/// Client connection manager
pub struct ConnectionManager {
    pub state: FFIConnectionState,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub database_name: Option<String>,
    pub last_error: Option<String>,
}

impl ConnectionManager {
    /// Create a new connection manager
    pub fn new() -> Self {
        Self {
            state: FFIConnectionState::Disconnected,
            host: None,
            port: None,
            database_name: None,
            last_error: None,
        }
    }
    
    /// Attempt to connect to SpacetimeDB
    pub fn connect(&mut self, host: String, port: u16, database_name: String) -> Result<(), String> {
        log::info!("Attempting connection to {}:{} database: {}", host, port, database_name);
        
        self.state = FFIConnectionState::Connecting;
        self.host = Some(host.clone());
        self.port = Some(port);
        self.database_name = Some(database_name.clone());
        
        // In a real implementation, you would:
        // 1. Establish WebSocket connection to SpacetimeDB
        // 2. Handle authentication handshake
        // 3. Set up message handlers
        
        // For now, simulate successful connection
        self.state = FFIConnectionState::Connected;
        self.last_error = None;
        
        log::info!("Successfully connected to SpacetimeDB");
        Ok(())
    }
    
    /// Disconnect from SpacetimeDB
    pub fn disconnect(&mut self) {
        log::info!("Disconnecting from SpacetimeDB");
        
        self.state = FFIConnectionState::Disconnected;
        self.last_error = None;
        
        // In a real implementation, you would:
        // 1. Send disconnect message
        // 2. Close WebSocket connection
        // 3. Clean up resources
    }
    
    /// Check if currently connected
    pub fn is_connected(&self) -> bool {
        matches!(self.state, 
            FFIConnectionState::Connected | 
            FFIConnectionState::Authenticated | 
            FFIConnectionState::InGame
        )
    }
    
    /// Get current connection state
    pub fn get_state(&self) -> FFIConnectionState {
        self.state
    }
    
    /// Set error state
    pub fn set_error(&mut self, error: String) {
        log::error!("Connection error: {}", error);
        self.state = FFIConnectionState::Error;
        self.last_error = Some(error);
    }
    
    /// Get last error message
    pub fn get_last_error(&self) -> Option<&String> {
        self.last_error.as_ref()
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}