//! Constants shared across the entire system

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

// Chat system limits
pub const MAX_CHAT_MESSAGE_LENGTH: usize = 500;
pub const MAX_CHAT_HISTORY: usize = 100;

// Player limits
pub const MIN_USERNAME_LENGTH: usize = 3;
pub const MAX_USERNAME_LENGTH: usize = 20;
pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_MOVEMENT_DISTANCE: f32 = 50.0;

// Zone and world limits
pub const DEFAULT_STARTING_ZONE: &str = "default";
pub const MAX_PLAYERS_PER_ZONE: u32 = 2000;