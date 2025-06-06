//! SharedModule: Common types and utilities for the MMO system
//! 
//! This module defines the "contract" between client and server.
//! Think of it as the common language that all parts of your system speak.

use serde::{Deserialize, Serialize};
use spacetimedb::{Identity, Timestamp};

// Re-export commonly used types for convenience
pub use spacetimedb::{Identity, Timestamp};

/// Core object identification system
/// Every object in your game world gets a unique ID and class definition
/// This is like giving every citizen in your virtual world both a social security number
/// and a job title - you know both who they are and what they do
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
}

/// Property system - the heart of object replication
/// Instead of hardcoding specific fields for each object type,
/// we use a flexible property bag system. This is like having
/// a universal form where any object can write down any attribute
/// it needs to track, without us having to redesign the form each time.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PropertyValue {
    pub name: String,
    pub property_type: PropertyType,
    pub value_json: String,
    pub replication_mode: ReplicationMode,
    pub owner_only: bool,
}

/// The different types of data we can store as properties
/// This enum acts like a type system for our property values
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
/// This gives you fine-grained control over network traffic
/// Think of it like different postal services - some things need
/// to be delivered to everyone, others only to specific people
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReplicationMode {
    None,          // Never replicated (server-only secrets)
    Always,        // Replicated to everyone who can see the object  
    OwnerOnly,     // Only replicated to the object's owner
    Conditional,   // Replicated based on custom conditions
}

/// Relevancy system - determines who should know about what
/// In a large game world, not everyone needs to know about everything
/// This is like having different news channels for different neighborhoods
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

/// Constants that are shared across the entire system
/// These are like the "building codes" for your virtual world
pub mod constants {
    // Performance limits to prevent system overload
    pub const MAX_OBJECTS_PER_ZONE: u32 = 10000;
    pub const MAX_PROPERTIES_PER_OBJECT: u32 = 100;
    pub const MAX_ZONES: u32 = 1000;
    
    // Network limits to prevent abuse
    pub const MAX_RPC_CALLS_PER_SECOND: u32 = 30;
    pub const MAX_PROPERTY_UPDATES_PER_SECOND: u32 = 60;
    pub const MAX_MESSAGE_SIZE_BYTES: u32 = 65536;
    
    // Timing constants for various systems
    pub const HEARTBEAT_INTERVAL_SECONDS: u64 = 30;
    pub const INACTIVITY_TIMEOUT_SECONDS: u64 = 300;
    pub const POSITION_UPDATE_INTERVAL_MS: u64 = 100;
    
    // Default values that make sense for most situations
    pub const DEFAULT_ZONE_ID: u32 = 1;
    pub const DEFAULT_MAX_DISTANCE: f32 = 1000.0;
}

/// Utility functions that are useful across modules
impl PropertyValue {
    /// Create a new property with sensible defaults
    pub fn new_simple(name: String, value_json: String) -> Self {
        Self {
            name,
            property_type: PropertyType::Json, // Default to JSON for flexibility
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

/// Helper functions for working with object IDs
impl ObjectId {
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