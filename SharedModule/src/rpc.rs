//! RPC (Remote Procedure Call) system
//! 
//! This handles function calls between client and server.
//! Think of RPCs like a telephone system where different parts
//! of your game can call each other to request actions.

use serde::{Deserialize, Serialize};
use crate::types::ObjectId;

/// Represents a function call from one part of the system to another
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcCall {
    pub function_name: String,
    pub target_object: Option<ObjectId>,
    pub arguments: Vec<RpcArgument>,
    pub call_type: RpcType,
}

/// Individual arguments passed to an RPC function
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RpcArgument {
    pub name: String,
    pub value_json: String,
    pub arg_type: String, // Type information for validation
}

/// Different types of RPC calls, each with different delivery guarantees
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RpcType {
    ClientToServer,    // Client asks server to do something
    ServerToClient,    // Server tells client to do something  
    Multicast,         // Server tells multiple clients to do something
    Reliable,          // Must arrive, in order (important events)
    Unreliable,        // May be lost, out of order (position updates)
}

impl RpcCall {
    /// Create a simple client-to-server RPC call
    pub fn client_to_server(function_name: String) -> Self {
        Self {
            function_name,
            target_object: None,
            arguments: Vec::new(),
            call_type: RpcType::ClientToServer,
        }
    }
    
    /// Add an argument to this RPC call
    pub fn with_arg(mut self, name: String, value_json: String) -> Self {
        self.arguments.push(RpcArgument {
            name,
            value_json,
            arg_type: "json".to_string(),
        });
        self
    }
    
    /// Set the target object for this RPC call
    pub fn targeting(mut self, object_id: ObjectId) -> Self {
        self.target_object = Some(object_id);
        self
    }
}