
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let header_path = Path::new(manifest_dir).join("target").join("spacetimedb_server.h");
    
    if !header_path.exists() {
        eprintln!("Header file not found. Run 'cargo build' first.");
        std::process::exit(1);
    }
    
    let header_content = fs::read_to_string(&header_path)?;
    
    // Generate additional C++ wrapper if needed
    let cpp_wrapper = generate_cpp_wrapper(&header_content);
    let cpp_path = Path::new(manifest_dir).join("target").join("spacetimedb_server_wrapper.hpp");
    fs::write(cpp_path, cpp_wrapper)?;
    
    println!("Bindings generated successfully!");
    Ok(())
}

fn generate_cpp_wrapper(header_content: &str) -> String {
    format!(r#"
// Generated C++ wrapper for SpacetimeDB Server
#pragma once

#ifdef __cplusplus
extern "C" {{
#endif

{}

#ifdef __cplusplus
}}

// C++ convenience wrapper class
namespace SpacetimeDB {{
    
class ServerBridge {{
public:
    static bool Connect(const char* host, uint16_t port, const char* db_name) {{
        auto result = spacetimedb_connect(host, port, db_name);
        bool success = result.success;
        spacetimedb_free_result(&result);
        return success;
    }}
    
    static bool RegisterUser(const char* username, const char* password, const char* email = nullptr) {{
        auto result = spacetimedb_register_user(username, password, email);
        bool success = result.success;
        spacetimedb_free_result(&result);
        return success;
    }}
    
    static bool LoginUser(const char* username, const char* password) {{
        auto result = spacetimedb_login_user(username, password);
        bool success = result.success;
        spacetimedb_free_result(&result);
        return success;
    }}
    
    static bool JoinGame(const char* starting_zone) {{
        auto result = spacetimedb_join_game(starting_zone);
        bool success = result.success;
        spacetimedb_free_result(&result);
        return success;
    }}
    
    static bool UpdatePosition(float x, float y, float z, float yaw) {{
        auto result = spacetimedb_update_position(x, y, z, yaw);
        bool success = result.success;
        spacetimedb_free_result(&result);
        return success;
    }}
    
    static bool SendChat(const char* message, const char* channel) {{
        auto result = spacetimedb_send_chat(message, channel);
        bool success = result.success;
        spacetimedb_free_result(&result);
        return success;
    }}
}};

}} // namespace SpacetimeDB

#endif // __cplusplus
"#, header_content)
}