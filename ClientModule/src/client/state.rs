//! Client-side state management

use shared_module::*;
use crate::bridge::*;
use crate::client::ConnectionManager;
use parking_lot::RwLock;
use std::sync::Arc;

/// Global client state
pub struct ClientState {
    pub connection: Arc<RwLock<ConnectionManager>>,
    pub current_user: Arc<RwLock<Option<String>>>,
    pub current_player: Arc<RwLock<Option<FFIPlayer>>>,
    pub chat_messages: Arc<RwLock<Vec<FFIChatMessage>>>,
}

impl ClientState {
    /// Create new client state
    pub fn new() -> Self {
        Self {
            connection: Arc::new(RwLock::new(ConnectionManager::new())),
            current_user: Arc::new(RwLock::new(None)),
            current_player: Arc::new(RwLock::new(None)),
            chat_messages: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Set current user
    pub fn set_current_user(&self, username: Option<String>) {
        *self.current_user.write() = username;
    }
    
    /// Get current user
    pub fn get_current_user(&self) -> Option<String> {
        self.current_user.read().clone()
    }
    
    /// Set current player
    pub fn set_current_player(&self, player: Option<FFIPlayer>) {
        *self.current_player.write() = player;
    }
    
    /// Get current player
    pub fn get_current_player(&self) -> Option<FFIPlayer> {
        self.current_player.read().clone()
    }
    
    /// Add chat message
    pub fn add_chat_message(&self, message: FFIChatMessage) {
        let mut messages = self.chat_messages.write();
        messages.push(message);
        
        // Keep only recent messages to prevent memory bloat
        if messages.len() > 100 {
            messages.remove(0);
        }
    }
    
    /// Get recent chat messages
    pub fn get_chat_messages(&self) -> Vec<FFIChatMessage> {
        self.chat_messages.read().clone()
    }
    
    /// Clear all state (for logout)
    pub fn clear_state(&self) {
        self.set_current_user(None);
        self.set_current_player(None);
        self.chat_messages.write().clear();
    }
    
    /// Check if user is logged in
    pub fn is_logged_in(&self) -> bool {
        self.current_user.read().is_some()
    }
    
    /// Check if player is in game
    pub fn is_in_game(&self) -> bool {
        self.current_player.read().is_some()
    }
}

impl Default for ClientState {
    fn default() -> Self {
        Self::new()
    }
}

// Global client state instance
lazy_static::lazy_static! {
    pub static ref GLOBAL_CLIENT_STATE: ClientState = ClientState::new();
}

/// Get the global client state
pub fn get_client_state() -> &'static ClientState {
    &GLOBAL_CLIENT_STATE
}