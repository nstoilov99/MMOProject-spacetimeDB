//! Server management tools
//! 
//! This utility provides tools for managing SpacetimeDB servers,
//! including deployment, monitoring, and database operations.

use std::env;
use std::fs;
use std::path::Path;
use std::process::{self, Command};

fn main() {
    println!("SpacetimeDB MMO Template - Server Tools");
    println!("=======================================");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }
    
    match args[1].as_str() {
        "build" => build_server_module(),
        "deploy" => deploy_server(&args[2..]),
        "status" => check_server_status(&args[2..]),
        "logs" => show_server_logs(&args[2..]),
        "init-db" => initialize_database(&args[2..]),
        "generate-schema" => generate_schema(),
        "test" => run_server_tests(),
        "help" | "--help" | "-h" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: server_tools <command> [options]");
    println!();
    println!("Commands:");
    println!("  build                Build the SpacetimeDB module");
    println!("  deploy <host>        Deploy server module to SpacetimeDB");
    println!("  status <host>        Check server status");
    println!("  logs <host>          Show server logs");
    println!("  init-db <host>       Initialize database with default data");
    println!("  generate-schema      Generate database schema documentation");
    println!("  test                 Run server tests");
    println!("  help                 Show this help message");
    println!();
    println!("Environment Variables:");
    println!("  SPACETIMEDB_HOST     Default SpacetimeDB host");
    println!("  SPACETIMEDB_TOKEN    Authentication token");
    println!();
    println!("Examples:");
    println!("  server_tools build");
    println!("  server_tools deploy localhost");
    println!("  server_tools status production.spacetimedb.com");
}

fn build_server_module() {
    println!("Building SpacetimeDB server module...");
    
    let status = Command::new("cargo")
        .args(&[
            "build",
            "--release",
            "--features=server",
            "--target=wasm32-unknown-unknown"
        ])
        .status()
        .expect("Failed to execute cargo build");
    
    if !status.success() {
        eprintln!("Build failed!");
        process::exit(1);
    }
    
    // Copy the built WASM file to a known location
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let target_dir = Path::new(manifest_dir).join("target");
    let wasm_source = target_dir
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("spacetime_mmo_template.wasm");
    
    let wasm_dest = target_dir
        .join("spacetimedb")
        .join("module.wasm");
    
    if let Ok(_) = fs::create_dir_all(wasm_dest.parent().unwrap()) {
        if let Ok(_) = fs::copy(&wasm_source, &wasm_dest) {
            println!("Server module built successfully!");
            println!("WASM file: {}", wasm_dest.display());
        } else {
            eprintln!("Failed to copy WASM file");
            process::exit(1);
        }
    }
}

fn deploy_server(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: Host required for deployment");
        eprintln!("Usage: server_tools deploy <host>");
        process::exit(1);
    }
    
    let host = &args[0];
    println!("Deploying server module to {}...", host);
    
    // First, build the module
    build_server_module();
    
    println!("Use 'spacetimedb publish' command to deploy the module manually.");
    println!("Module location: target/spacetimedb/module.wasm");
}

fn check_server_status(args: &[String]) {
    let host = args.get(0)
        .or_else(|| env::var("SPACETIMEDB_HOST").ok().as_ref())
        .unwrap_or_else(|| {
            eprintln!("Error: Host required");
            eprintln!("Usage: server_tools status <host>");
            process::exit(1);
        });
    
    println!("Checking server status for {}...", host);
    println!("Use 'spacetimedb server status' command to check manually.");
}

fn show_server_logs(args: &[String]) {
    let host = args.get(0)
        .or_else(|| env::var("SPACETIMEDB_HOST").ok().as_ref())
        .unwrap_or_else(|| {
            eprintln!("Error: Host required");
            eprintln!("Usage: server_tools logs <host>");
            process::exit(1);
        });
    
    println!("Showing logs for {}...", host);
    println!("Use 'spacetimedb logs' command to view logs manually.");
}

fn initialize_database(args: &[String]) {
    let host = args.get(0)
        .or_else(|| env::var("SPACETIMEDB_HOST").ok().as_ref())
        .unwrap_or_else(|| {
            eprintln!("Error: Host required");
            eprintln!("Usage: server_tools init-db <host>");
            process::exit(1);
        });
    
    println!("Initializing database on {}...", host);
    
    // Create SQL script for initial data
    let init_script = r#"
-- Initialize default game items
-- Note: SpacetimeDB uses reducers, not SQL
-- Call these reducers to initialize data:
-- - initialize_custom_features()
-- - spawn_npc("Goblin", "goblin", 100.0, 100.0, 0.0, 1)
-- - give_item_to_player("admin", "sword_iron", 1)
    "#;
    
    let script_path = Path::new("init_db.txt");
    fs::write(script_path, init_script).expect("Failed to write init script");
    
    println!("Database initialization notes created: {}", script_path.display());
    println!("Call the listed reducers to initialize your database with default data.");
}

fn generate_schema() {
    println!("Generating database schema documentation...");
    
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let schema_doc = generate_schema_documentation();
    
    let docs_dir = Path::new(manifest_dir).join("docs");
    let output_path = docs_dir.join("schema.md");
    
    if let Ok(_) = fs::create_dir_all(&docs_dir) {
        if let Ok(_) = fs::write(&output_path, schema_doc) {
            println!("Schema documentation generated: {}", output_path.display());
        } else {
            eprintln!("Failed to write schema documentation");
        }
    }
}

fn generate_schema_documentation() -> String {
    format!(r#"# SpacetimeDB MMO Template - Database Schema

Generated on: {}

## Tables

### User Table
Stores user account information.

| Column | Type | Description |
|--------|------|-------------|
| identity | Identity (PK) | Unique user identifier |
| username | String (Unique) | User's display name |
| password_hash | String | Hashed password |
| password_salt | String | Password salt |
| email | Option<String> | Email address |
| created_at | Timestamp | Account creation time |
| last_login | Timestamp | Last login time |
| is_active | bool | Account status |

### Player Table (game_players)
Stores active player state in the game world.

| Column | Type | Description |
|--------|------|-------------|
| identity | Identity (PK) | Links to User table |
| username | String | Display name |
| position_x | f32 | X coordinate |
| position_y | f32 | Y coordinate |
| position_z | f32 | Z coordinate |
| rotation_yaw | f32 | Rotation angle |
| level | u32 | Player level |
| experience | u64 | Experience points |
| health | f32 | Current health |
| max_health | f32 | Maximum health |
| is_online | bool | Online status |
| last_seen | Timestamp | Last activity |
| current_zone | String | Current zone/area |

## Reducers

### Authentication
- `register_user(username, password, email?)` - Create new account
- `login_user(username, password, client_version)` - Authenticate user
- `logout_user()` - End session

### Player Management
- `join_game(starting_zone)` - Enter game world
- `leave_game()` - Exit game world
- `update_player_position(x, y, z, yaw)` - Update position

### Chat System
- `send_chat_message(message, channel)` - Send chat message

For complete documentation, see the source code in ServerModule and CustomServerModule.
"#, std::env::var("BUILD_DATE").unwrap_or_else(|_| "Unknown".to_string()))
}

fn run_server_tests() {
    println!("Running server tests...");
    
    let status = Command::new("cargo")
        .args(&[
            "test",
            "--features=server"
        ])
        .status()
        .expect("Failed to execute cargo test");
    
    if status.success() {
        println!("All tests passed!");
    } else {
        eprintln!("Some tests failed!");
        process::exit(1);
    }
}