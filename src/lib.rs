//! SpacetimeDB MMO Template - Main Integration Module
//! 
//! This is the main entry point that brings together all the modules
//! and provides a unified interface for both SpacetimeDB and FFI clients.

// Re-export all modules for easy access
pub use shared_module;

#[cfg(feature = "server")]
pub use server_module;

#[cfg(feature = "client")]
pub use client_module;

#[cfg(feature = "server")]
pub use custom_server_module;

// Bridge modules for FFI
#[cfg(feature = "client")]
pub mod bridge {
    pub use client_module::bridge::*;
}

// FFI exports for Unreal Engine
#[cfg(feature = "client")]
pub mod ffi {
    pub use client_module::ffi::*;
}

// Server exports for SpacetimeDB
#[cfg(feature = "server")]
pub mod server {
    pub use server_module::*;
    pub use custom_server_module::*;
}

// Conditional compilation based on target
#[cfg(feature = "server")]
pub use server_module::*;
#[cfg(feature = "server")]
pub use custom_server_module::*;

/// Initialize the entire MMO system
/// This function sets up all subsystems and prepares the environment
pub fn initialize_mmo_system() -> Result<(), String> {
    // Initialize logging
    env_logger::init();
    
    log::info!("Initializing SpacetimeDB MMO Template...");
    
    // Initialize client module (for FFI builds)
    #[cfg(feature = "client")]
    {
        client_module::initialize_client();
        log::info!("Client module initialized");
    }
    
    log::info!("SpacetimeDB MMO Template initialized successfully");
    Ok(())
}

/// Get version information
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Get build information
pub fn get_build_info() -> BuildInfo {
    BuildInfo {
        version: get_version(),
        build_date: env!("BUILD_DATE"),
        git_hash: env!("GIT_HASH"),
        target: std::env::consts::TARGET,
        profile: if cfg!(debug_assertions) { "debug" } else { "release" },
    }
}

/// Build information structure
#[derive(Debug, Clone)]
pub struct BuildInfo {
    pub version: &'static str,
    pub build_date: &'static str,
    pub git_hash: &'static str,
    pub target: &'static str,
    pub profile: &'static str,
}

// Feature-gated exports for different build types
#[cfg(feature = "client")]
pub mod client_exports {
    pub use crate::client_module::*;
    #[cfg(feature = "client")]
    pub use crate::ffi::*;
}

#[cfg(feature = "server")]
pub mod server_exports {
    pub use crate::server_module::*;
    pub use crate::custom_server_module::*;
}

// Test module for integration testing
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_initialization() {
        assert!(initialize_mmo_system().is_ok());
    }
    
    #[test]
    fn test_version_info() {
        let version = get_version();
        assert!(!version.is_empty());
        
        let build_info = get_build_info();
        assert!(!build_info.version.is_empty());
        assert!(!build_info.target.is_empty());
    }
}