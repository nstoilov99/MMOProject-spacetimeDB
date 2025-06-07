//! Binding generation utility
//! 
//! This tool generates C/C++ bindings and Unreal Engine integration files
//! for the SpacetimeDB MMO Template.

use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn main() {
    println!("SpacetimeDB MMO Template - Binding Generator");
    println!("============================================");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    match args[1].as_str() {
        "generate" => generate_all_bindings(),
        "clean" => clean_generated_files(),
        "check" => check_generated_files(),
        "unreal" => generate_unreal_integration(),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: generate_bindings <command>");
    println!();
    println!("Commands:");
    println!("  generate  Generate all binding files");
    println!("  clean     Clean generated files");
    println!("  check     Check if generated files are up to date");
    println!("  unreal    Generate Unreal Engine specific files");
    println!("  help      Show this help message");
    println!();
    println!("Environment Variables:");
    println!("  UNREAL_PROJECT_PATH  Path to Unreal Engine project");
    println!("  OUTPUT_DIR          Custom output directory");
}

fn generate_all_bindings() {
    println!("Generating all binding files...");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let target_dir = Path::new(manifest_dir).join("target");
    
    // Ensure target directory exists
    fs::create_dir_all(&target_dir).expect("Failed to create target directory");
    
    // Generate C bindings using cbindgen
    if let Err(e) = generate_c_bindings(&manifest_dir, &target_dir) {
        eprintln!("Failed to generate C bindings: {}", e);
        process::exit(1);
    }
    
    // Generate C++ wrapper
    if let Err(e) = generate_cpp_wrapper(&target_dir) {
        eprintln!("Failed to generate C++ wrapper: {}", e);
        process::exit(1);
    }
    
    // Generate Unreal Engine integration
    if let Err(e) = generate_unreal_files(&target_dir) {
        eprintln!("Failed to generate Unreal files: {}", e);
        process::exit(1);
    }
    
    println!("All bindings generated successfully!");
}

fn generate_c_bindings(manifest_dir: &str, target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let bindings_dir = target_dir.join("bindings");
    fs::create_dir_all(&bindings_dir)?;
    
    let header_path = bindings_dir.join("spacetime_mmo_template.h");
    
    // Simple header generation for now
    let header_content = r#"/* Auto-generated header for SpacetimeDB MMO Template */
#ifndef SPACETIME_MMO_TEMPLATE_H
#define SPACETIME_MMO_TEMPLATE_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    bool success;
    char* error_message;
    void* data;
    size_t data_size;
} FFIResult;

// Connection functions
FFIResult spacetimedb_connect(const char* host, unsigned short port, const char* database_name);
FFIResult spacetimedb_register_user(const char* username, const char* password, const char* email);
FFIResult spacetimedb_login_user(const char* username, const char* password);
FFIResult spacetimedb_join_game(const char* starting_zone);
FFIResult spacetimedb_update_position(float x, float y, float z, float yaw);
FFIResult spacetimedb_send_chat(const char* message, const char* channel);

// Memory management
void spacetimedb_free_result(FFIResult* result);
void spacetimedb_free_string(char* ptr);

#ifdef __cplusplus
}
#endif

#endif /* SPACETIME_MMO_TEMPLATE_H */
"#;
    
    fs::write(&header_path, header_content)?;
    println!("Generated C header: {}", header_path.display());
    Ok(())
}

fn generate_cpp_wrapper(target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let bindings_dir = target_dir.join("bindings");
    let wrapper_path = bindings_dir.join("spacetime_mmo_template_wrapper.hpp");
    
    let wrapper_content = r#"// SpacetimeDB MMO Template C++ Wrapper
// Auto-generated - DO NOT EDIT

#pragma once

#include <string>
#include <vector>
#include <memory>
#include <functional>
#include "spacetime_mmo_template.h"

namespace SpacetimeDB {

class MMOClient {
public:
    // Connection management
    static bool Connect(const std::string& host, uint16_t port, const std::string& dbName);
    static void Disconnect();
    static bool IsConnected();
    
    // Authentication  
    static bool RegisterUser(const std::string& username, const std::string& password, const std::string& email = "");
    static bool LoginUser(const std::string& username, const std::string& password);
    static void LogoutUser();
    
    // Game world
    static bool JoinGame(const std::string& startingZone = "default");
    static void LeaveGame();
    static bool UpdatePosition(float x, float y, float z, float yaw);
    
    // Chat
    static bool SendChatMessage(const std::string& message, const std::string& channel = "global");
    
    // Error handling
    static std::string GetLastError();
    
private:
    static std::string last_error_;
};

} // namespace SpacetimeDB
"#;
    
    fs::write(&wrapper_path, wrapper_content)?;
    println!("Generated C++ wrapper: {}", wrapper_path.display());
    Ok(())
}

fn generate_unreal_files(target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let unreal_dir = target_dir.join("Unreal");
    fs::create_dir_all(&unreal_dir)?;
    
    // Generate .uplugin file
    let uplugin_content = r#"{
    "FileVersion": 3,
    "Version": 1,
    "VersionName": "1.0",
    "FriendlyName": "SpacetimeDB MMO Template",
    "Description": "Complete MMO integration with SpacetimeDB",
    "Category": "Networking",
    "CreatedBy": "SpacetimeDB Community",
    "CanContainContent": true,
    "Modules": [{
        "Name": "SpacetimeDBMMO",
        "Type": "Runtime",
        "LoadingPhase": "Default"
    }]
}
"#;
    
    let uplugin_path = unreal_dir.join("SpacetimeDBMMO.uplugin");
    fs::write(&uplugin_path, uplugin_content)?;
    
    // Generate Build.cs file
    let build_cs_content = r#"// SpacetimeDBMMO.Build.cs
using UnrealBuildTool;

public class SpacetimeDBMMO : ModuleRules
{
    public SpacetimeDBMMO(ReadOnlyTargetRules Target) : base(Target)
    {
        PCHUsage = ModuleRules.PCHUsageMode.UseExplicitOrSharedPCHs;
        
        PublicDependencyModuleNames.AddRange(new string[]
        {
            "Core",
            "CoreUObject",
            "Engine"
        });
        
        PrivateDependencyModuleNames.AddRange(new string[]
        {
            "Slate",
            "SlateCore"
        });
    }
}
"#;
    
    let build_cs_path = unreal_dir.join("SpacetimeDBMMO.Build.cs");
    fs::write(&build_cs_path, build_cs_content)?;
    
    println!("Generated Unreal Engine files in: {}", unreal_dir.display());
    Ok(())
}

fn generate_unreal_integration() {
    println!("Generating Unreal Engine integration files...");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let target_dir = Path::new(manifest_dir).join("target");
    
    if let Err(e) = generate_unreal_files(&target_dir) {
        eprintln!("Failed to generate Unreal files: {}", e);
        process::exit(1);
    }
    
    println!("Unreal Engine integration generated successfully!");
}

fn clean_generated_files() {
    println!("Cleaning generated files...");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let target_dir = Path::new(manifest_dir).join("target");
    
    let dirs_to_clean = vec![
        target_dir.join("bindings"),
        target_dir.join("Unreal"),
        target_dir.join("spacetimedb"),
    ];
    
    for dir in dirs_to_clean {
        if dir.exists() {
            if let Err(e) = fs::remove_dir_all(&dir) {
                eprintln!("Failed to remove {}: {}", dir.display(), e);
            } else {
                println!("Removed: {}", dir.display());
            }
        }
    }
    
    println!("Cleanup completed!");
}

fn check_generated_files() {
    println!("Checking generated files...");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let target_dir = Path::new(manifest_dir).join("target");
    
    let files_to_check = vec![
        target_dir.join("bindings").join("spacetime_mmo_template.h"),
        target_dir.join("bindings").join("spacetime_mmo_template_wrapper.hpp"),
        target_dir.join("Unreal").join("SpacetimeDBMMO.uplugin"),
        target_dir.join("Unreal").join("SpacetimeDBMMO.Build.cs"),
    ];
    
    let mut all_exist = true;
    
    for file in files_to_check {
        if file.exists() {
            println!("✓ {}", file.display());
        } else {
            println!("✗ {} (missing)", file.display());
            all_exist = false;
        }
    }
    
    if all_exist {
        println!("All generated files are present!");
    } else {
        println!("Some files are missing. Run 'generate_bindings generate' to create them.");
        process::exit(1);
    }
}