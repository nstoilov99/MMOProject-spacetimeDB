//! Core type definitions shared across all modules

use serde::{Deserialize, Serialize};
use spacetimedb::{Identity, Timestamp};

/// Core object identification system
/// Every object in your game world gets a unique ID and class definition
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ObjectId {
    pub id: u64,
    pub class_name: String,
}

impl ObjectId {
    pub fn new(id: u64, class_name: String) -> Self {
        Self { id, class_name }
    }
    
    pub fn is_valid(&self) -> bool {
        self.id != 0 && !self.class_name.is_empty()
    }
    
    /// Create a player character ID
    pub fn player(id: u64) -> Self {
        Self::new(id, "PlayerCharacter".to_string())
    }
    
    /// Create an NPC ID
    pub fn npc(id: u64) -> Self {
        Self::new(id, "NPC".to_string())
    }
    
    /// Create an item ID
    pub fn item(id: u64) -> Self {
        Self::new(id, "Item".to_string())
    }
}

/// Property system - the heart of object replication
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyValue {
    pub name: String,
    pub property_type: PropertyType,
    pub value_json: String,
    pub replication_mode: ReplicationMode,
    pub owner_only: bool,
}

/// The different types of data we can store as properties
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PropertyType {
    Bool,
    Int32,
    Int64,
    Float,
    Double,
    String,
    Vector3,       // For positions, velocities, etc.
    Rotator,       // For rotations  
    Transform,     // For complete object transforms
    Json,          // For complex data structures
}

/// Different ways properties can be replicated
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReplicationMode {
    None,          // Never replicated (server-only secrets)
    Always,        // Replicated to everyone who can see the object  
    OwnerOnly,     // Only replicated to the object's owner
    Conditional,   // Replicated based on custom conditions
}

/// Relevancy system - determines who should know about what
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RelevancyInfo {
    pub object_id: ObjectId,
    pub relevancy_type: RelevancyType,
    pub zone_id: Option<u32>,
    pub max_distance: Option<f32>,
    pub custom_rules: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RelevancyType {
    Global,        // Everyone can see this (use sparingly!)
    Zone,          // Only players in the same zone
    Distance,      // Only players within a certain distance
    Owner,         // Only the owner can see this
    Party,         // Only party members can see this
    Guild,         // Only guild members can see this
    Custom,        // Use custom logic to determine relevancy
}

/// Connection state for tracking client status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Authenticated,
    InGame,
    Error(String),
}

impl PropertyValue {
    /// Create a new property with sensible defaults
    pub fn new_simple(name: String, value_json: String) -> Self {
        Self {
            name,
            property_type: PropertyType::Json,
            value_json,
            replication_mode: ReplicationMode::Always,
            owner_only: false,
        }
    }
    
    /// Create an owner-only property (like private inventory)
    pub fn new_owner_only(name: String, value_json: String) -> Self {
        Self {
            name,
            property_type: PropertyType::Json,
            value_json,
            replication_mode: ReplicationMode::OwnerOnly,
            owner_only: true,
        }
    }
}