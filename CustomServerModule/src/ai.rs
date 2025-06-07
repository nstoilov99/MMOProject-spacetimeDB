//! AI system for NPCs and game entities

use spacetimedb::{reducer, ReducerContext};
use shared_module::*;
use crate::*;

/// AI states for NPCs
#[derive(Clone, Debug)]
pub enum AIState {
    Idle,
    Patrolling,
    Chasing,
    Attacking,
    Fleeing,
    Dead,
}

impl AIState {
    pub fn to_string(&self) -> String {
        match self {
            AIState::Idle => "idle".to_string(),
            AIState::Patrolling => "patrolling".to_string(),
            AIState::Chasing => "chasing".to_string(),
            AIState::Attacking => "attacking".to_string(),
            AIState::Fleeing => "fleeing".to_string(),
            AIState::Dead => "dead".to_string(),
        }
    }
    
    pub fn from_string(s: &str) -> AIState {
        match s {
            "idle" => AIState::Idle,
            "patrolling" => AIState::Patrolling,
            "chasing" => AIState::Chasing,
            "attacking" => AIState::Attacking,
            "fleeing" => AIState::Fleeing,
            "dead" => AIState::Dead,
            _ => AIState::Idle,
        }
    }
}

/// Initialize AI systems
pub fn initialize_ai_systems(_ctx: &ReducerContext) -> Result<(), String> {
    log::info!("AI systems initialized");
    Ok(())
}

/// Spawn an NPC
#[reducer]
pub fn spawn_npc(
    ctx: &ReducerContext,
    name: String,
    npc_type: String,
    x: f32,
    y: f32,
    z: f32,
    zone_id: u32
) -> Result<u64, String> {
    // Validate position
    validate_world_position(x, y, z)?;
    
    // Generate unique NPC ID
    let npc_id = generate_unique_id(&ctx.sender, ctx.timestamp);
    
    // Determine stats based on NPC type
    let (health, max_health) = match npc_type.as_str() {
        "goblin" => (50.0, 50.0),
        "orc" => (100.0, 100.0),
        "dragon" => (1000.0, 1000.0),
        "merchant" => (100.0, 100.0),
        _ => (50.0, 50.0), // Default
    };
    
    // Create the NPC
    ctx.db.npcs().insert(NPC {
        npc_id,
        name: name.clone(),
        npc_type: npc_type.clone(),
        position_x: x,
        position_y: y,
        position_z: z,
        zone_id,
        health,
        max_health,
        ai_state: AIState::Idle.to_string(),
        respawn_time: ctx.timestamp,
    });
    
    log::info!("Spawned NPC '{}' of type '{}' at ({}, {}, {})", name, npc_type, x, y, z);
    Ok(npc_id)
}

/// Update NPC AI - this would be called periodically
#[reducer]
pub fn update_npc_ai(ctx: &ReducerContext, npc_id: u64) -> Result<(), String> {
    let mut npc = ctx.db.npcs().npc_id().find(&npc_id)
        .ok_or("NPC not found")?
        .clone();
    
    if npc.health <= 0.0 {
        npc.ai_state = AIState::Dead.to_string();
        ctx.db.npcs().npc_id().update(npc);
        return Ok(());
    }
    
    let current_state = AIState::from_string(&npc.ai_state);
    
    // Get nearby players to determine AI behavior
    let nearby_players = get_nearby_players(ctx, npc.position_x, npc.position_y, npc.position_z, 10.0);
    
    // Simple AI state machine
    let new_state = match current_state {
        AIState::Idle => {
            if !nearby_players.is_empty() {
                match npc.npc_type.as_str() {
                    "goblin" | "orc" => AIState::Chasing,
                    "merchant" => AIState::Idle, // Merchants don't chase
                    _ => AIState::Patrolling,
                }
            } else {
                AIState::Patrolling
            }
        },
        AIState::Patrolling => {
            if !nearby_players.is_empty() && is_aggressive_npc(&npc.npc_type) {
                AIState::Chasing
            } else {
                AIState::Patrolling
            }
        },
        AIState::Chasing => {
            if nearby_players.is_empty() {
                AIState::Idle
            } else if npc.health < npc.max_health * 0.2 {
                AIState::Fleeing
            } else {
                AIState::Attacking
            }
        },
        AIState::Attacking => {
            if nearby_players.is_empty() {
                AIState::Idle
            } else if npc.health < npc.max_health * 0.2 {
                AIState::Fleeing
            } else {
                AIState::Attacking
            }
        },
        AIState::Fleeing => {
            if nearby_players.is_empty() {
                AIState::Idle
            } else {
                AIState::Fleeing
            }
        },
        AIState::Dead => {
            // Check if respawn time has passed
            let respawn_duration = std::time::Duration::from_secs(30); // 30 seconds respawn
            if ctx.timestamp >= npc.respawn_time + respawn_duration {
                npc.health = npc.max_health;
                AIState::Idle
            } else {
                AIState::Dead
            }
        },
    };
    
    // Update NPC state if changed
    if new_state.to_string() != npc.ai_state {
        npc.ai_state = new_state.to_string();
        
        // Perform state-specific actions
        match AIState::from_string(&npc.ai_state) {
            AIState::Patrolling => {
                // Move to a random nearby position
                let (new_x, new_y) = get_patrol_position(npc.position_x, npc.position_y);
                npc.position_x = new_x;
                npc.position_y = new_y;
            },
            AIState::Chasing | AIState::Attacking => {
                // Move towards nearest player
                if let Some(target) = nearby_players.first() {
                    let (new_x, new_y) = move_towards_target(
                        npc.position_x, npc.position_y,
                        target.position_x, target.position_y,
                        2.0 // movement speed
                    );
                    npc.position_x = new_x;
                    npc.position_y = new_y;
                }
            },
            AIState::Fleeing => {
                // Move away from nearest player
                if let Some(threat) = nearby_players.first() {
                    let (new_x, new_y) = move_away_from_target(
                        npc.position_x, npc.position_y,
                        threat.position_x, threat.position_y,
                        3.0 // flee speed (faster than normal movement)
                    );
                    npc.position_x = new_x;
                    npc.position_y = new_y;
                }
            },
            _ => {}, // No movement for other states
        }
        
        ctx.db.npcs().npc_id().update(npc);
    }
    
    Ok(())
}

/// Attack an NPC (player action)
#[reducer]
pub fn attack_npc(
    ctx: &ReducerContext,
    npc_id: u64,
    damage: f32
) -> Result<(), String> {
    // Verify player is in game
    let player = ctx.db.game_players().identity().find(&ctx.sender)
        .ok_or("Player not found")?
        .clone();
    
    if !player.is_online {
        return Err("Must be online to attack".to_string());
    }
    
    let mut npc = ctx.db.npcs().npc_id().find(&npc_id)
        .ok_or("NPC not found")?
        .clone();
    
    // Check if player is close enough to attack
    let distance = calculate_distance(
        player.position_x, player.position_y, player.position_z,
        npc.position_x, npc.position_y, npc.position_z
    );
    
    if distance > 5.0 {
        return Err("Too far away to attack".to_string());
    }
    
    // Apply damage
    npc.health = (npc.health - damage).max(0.0);
    
    // If NPC dies, set respawn timer
    if npc.health <= 0.0 {
        npc.ai_state = AIState::Dead.to_string();
        npc.respawn_time = ctx.timestamp;
        log::info!("NPC '{}' killed by player '{}'", npc.name, player.username);
    } else {
        // NPC becomes aggressive if not already
        if npc.ai_state == AIState::Idle.to_string() {
            npc.ai_state = AIState::Chasing.to_string();
        }
    }
    
    ctx.db.npcs().npc_id().update(npc);
    Ok(())
}

/// Get nearby players to determine AI behavior
fn get_nearby_players(
    ctx: &ReducerContext,
    x: f32,
    y: f32,
    z: f32,
    radius: f32
) -> Vec<server_module::Player> {
    ctx.db.game_players().iter()
        .filter(|player| {
            player.is_online && {
                let distance = calculate_distance(
                    x, y, z,
                    player.position_x, player.position_y, player.position_z
                );
                distance <= radius
            }
        })
        .cloned()
        .collect()
}

fn is_aggressive_npc(npc_type: &str) -> bool {
    matches!(npc_type, "goblin" | "orc" | "dragon")
}

fn get_patrol_position(current_x: f32, current_y: f32) -> (f32, f32) {
    // Simple random patrol within 5 units
    // Use a simple hash of current position as seed
    let seed = ((current_x * 1000.0) as u64).wrapping_add((current_y * 1000.0) as u64);
    let angle = (seed % 628) as f32 / 100.0; // Approximate 2*PI
    let distance = 5.0;
    
    (
        current_x + distance * angle.cos(),
        current_y + distance * angle.sin()
    )
}

fn move_towards_target(
    current_x: f32, current_y: f32,
    target_x: f32, target_y: f32,
    speed: f32
) -> (f32, f32) {
    let dx = target_x - current_x;
    let dy = target_y - current_y;
    let distance = (dx * dx + dy * dy).sqrt();
    
    if distance > 0.0 {
        let move_distance = speed.min(distance);
        (
            current_x + (dx / distance) * move_distance,
            current_y + (dy / distance) * move_distance
        )
    } else {
        (current_x, current_y)
    }
}

fn move_away_from_target(
    current_x: f32, current_y: f32,
    threat_x: f32, threat_y: f32,
    speed: f32
) -> (f32, f32) {
    let dx = current_x - threat_x;
    let dy = current_y - threat_y;
    let distance = (dx * dx + dy * dy).sqrt();
    
    if distance > 0.0 {
        (
            current_x + (dx / distance) * speed,
            current_y + (dy / distance) * speed
        )
    } else {
        // If at same position, move in random direction
        (current_x + speed, current_y)
    }
}