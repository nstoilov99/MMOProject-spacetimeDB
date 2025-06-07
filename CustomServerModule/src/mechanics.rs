//! Game mechanics like inventory, skills, and progression

use spacetimedb::{table, reducer, ReducerContext, Identity, Timestamp};
use shared_module::*;
use server_module::*;

/// Player inventory system
#[derive(Clone, Debug)]
#[table(name = player_inventory, public)]
pub struct PlayerInventory {
    #[primary_key]
    pub inventory_id: u64,
    pub player_identity: Identity,
    pub item_type: String,
    pub item_id: String,
    pub quantity: u32,
    pub slot_index: u32,
}

/// Player skills and progression
#[derive(Clone, Debug)]
#[table(name = player_skills, public)]
pub struct PlayerSkill {
    #[primary_key]
    pub skill_id: u64,
    pub player_identity: Identity,
    pub skill_name: String,
    pub skill_level: u32,
    pub experience: u64,
    pub last_updated: Timestamp,
}

/// Game items definitions
#[derive(Clone, Debug)]
#[table(name = game_items, public)]
pub struct GameItem {
    #[primary_key]
    pub item_id: String,
    pub item_name: String,
    pub item_type: String,
    pub description: String,
    pub max_stack_size: u32,
    pub value: u32,
    pub properties_json: String, // JSON string for flexible item properties
}

/// Initialize game mechanics
pub fn initialize_game_mechanics(ctx: &ReducerContext) -> Result<(), String> {
    log::info!("Initializing game mechanics...");
    
    // Create default items if they don't exist
    create_default_items(ctx);
    
    log::info!("Game mechanics initialized");
    Ok(())
}

/// Create default game items
fn create_default_items(ctx: &ReducerContext) {
    let default_items = vec![
        ("sword_iron", "Iron Sword", "weapon", "A sturdy iron sword", 1, 100, r#"{"damage": 25, "durability": 100}"#),
        ("potion_health", "Health Potion", "consumable", "Restores 50 health", 10, 25, r#"{"heal_amount": 50}"#),
        ("armor_leather", "Leather Armor", "armor", "Basic leather protection", 1, 50, r#"{"defense": 10, "durability": 50}"#),
        ("ore_iron", "Iron Ore", "material", "Raw iron ore for crafting", 50, 10, r#"{"crafting_material": true}"#),
        ("food_bread", "Bread", "consumable", "Restores 20 health and reduces hunger", 20, 5, r#"{"heal_amount": 20, "hunger_reduction": 30}"#),
    ];
    
    for (id, name, item_type, desc, stack_size, value, properties) in default_items {
        // Check if item already exists
        if ctx.db.game_items().item_id().find(&id.to_string()).is_none() {
            ctx.db.game_items().insert(GameItem {
                item_id: id.to_string(),
                item_name: name.to_string(),
                item_type: item_type.to_string(),
                description: desc.to_string(),
                max_stack_size: stack_size,
                value,
                properties_json: properties.to_string(),
            });
        }
    }
}

/// Give item to player
#[reducer]
pub fn give_item_to_player(
    ctx: &ReducerContext,
    target_username: String,
    item_id: String,
    quantity: u32
) -> Result<(), String> {
    // Verify the item exists
    let _item = ctx.db.game_items().item_id().find(&item_id)
        .ok_or("Item not found")?;
    
    // Find target player
    let target_player = ctx.db.game_players().iter()
        .find(|p| p.username == target_username)
        .ok_or("Player not found")?;
    
    // Add item to player's inventory
    add_item_to_inventory(ctx, &target_player.identity, &item_id, quantity)?;
    
    log::info!("Gave {} x{} to player {}", item_id, quantity, target_username);
    Ok(())
}

/// Add item to player inventory
#[reducer]
pub fn add_item_to_inventory(
    ctx: &ReducerContext,
    player_identity: &Identity,
    item_id: &str,
    quantity: u32
) -> Result<(), String> {
    let item = ctx.db.game_items().item_id().find(&item_id.to_string())
        .ok_or("Item not found")?;
    
    // Find existing stack of this item
    if let Some(existing_item) = find_inventory_item(ctx, player_identity, item_id) {
        let mut updated_item = existing_item.clone();
        let new_quantity = updated_item.quantity + quantity;
        
        if new_quantity <= item.max_stack_size {
            // Add to existing stack
            updated_item.quantity = new_quantity;
            ctx.db.player_inventory().inventory_id().update(updated_item);
        } else {
            // Split into multiple stacks or create new stack
            let remaining = new_quantity - item.max_stack_size;
            updated_item.quantity = item.max_stack_size;
            ctx.db.player_inventory().inventory_id().update(updated_item);
            
            // Create new stack for remaining items
            if remaining > 0 {
                create_new_inventory_slot(ctx, player_identity, item_id, remaining)?;
            }
        }
    } else {
        // Create new inventory slot
        create_new_inventory_slot(ctx, player_identity, item_id, quantity)?;
    }
    
    Ok(())
}

/// Remove item from player inventory
#[reducer]
pub fn remove_item_from_inventory(
    ctx: &ReducerContext,
    item_id: String,
    quantity: u32
) -> Result<(), String> {
    let player = ctx.db.game_players().identity().find(&ctx.sender)
        .ok_or("Player not found")?
        .clone();
    
    let inventory_item = find_inventory_item(ctx, &player.identity, &item_id)
        .ok_or("Item not found in inventory")?;
    
    if inventory_item.quantity < quantity {
        return Err("Not enough items in inventory".to_string());
    }
    
    if inventory_item.quantity == quantity {
        // Remove entire stack
        ctx.db.player_inventory().inventory_id().delete(&inventory_item.inventory_id);
    } else {
        // Reduce quantity
        let mut updated_item = inventory_item.clone();
        updated_item.quantity -= quantity;
        ctx.db.player_inventory().inventory_id().update(updated_item);
    }
    
    Ok(())
}

/// Use consumable item
#[reducer]
pub fn use_item(
    ctx: &ReducerContext,
    item_id: String
) -> Result<(), String> {
    let mut player = ctx.db.game_players().identity().find(&ctx.sender)
        .ok_or("Player not found")?
        .clone();
    
    let item = ctx.db.game_items().item_id().find(&item_id)
        .ok_or("Item not found")?;
    
    // Check if player has the item
    let _inventory_item = find_inventory_item(ctx, &player.identity, &item_id)
        .ok_or("Item not found in inventory")?;
    
    // Apply item effects based on type
    match item.item_type.as_str() {
        "consumable" => {
            // Simple parsing for heal_amount property
            if item.properties_json.contains("heal_amount") {
                // Extract heal amount from the JSON string (simple parsing)
                if let Some(start) = item.properties_json.find("\"heal_amount\":") {
                    let rest = &item.properties_json[start + 14..];
                    if let Some(end) = rest.find(&[',', '}'][..]) {
                        let heal_str = &rest[..end].trim();
                        if let Ok(heal_amount) = heal_str.parse::<f32>() {
                            player.health = (player.health + heal_amount).min(player.max_health);
                            log::info!("Player {} used {} and healed for {} HP", player.username, item.item_name, heal_amount);
                        }
                    }
                }
            }
            
            // Remove one item from inventory
            remove_item_from_inventory(ctx, item_id, 1)?;
            
            // Update player health
            ctx.db.game_players().identity().update(player);
        },
        _ => {
            return Err("This item cannot be used".to_string());
        }
    }
    
    Ok(())
}

/// Get player inventory
#[reducer]
pub fn get_player_inventory(ctx: &ReducerContext) -> Result<Vec<PlayerInventory>, String> {
    let player = ctx.db.game_players().identity().find(&ctx.sender)
        .ok_or("Player not found")?;
    
    let inventory: Vec<PlayerInventory> = ctx.db.player_inventory().iter()
        .filter(|item| item.player_identity == player.identity)
        .cloned()
        .collect();
    
    Ok(inventory)
}

/// Level up player skill
#[reducer]
pub fn gain_skill_experience(
    ctx: &ReducerContext,
    skill_name: String,
    experience_gained: u64
) -> Result<(), String> {
    let player = ctx.db.game_players().identity().find(&ctx.sender)
        .ok_or("Player not found")?;
    
    // Find or create skill
    if let Some(mut skill) = find_player_skill(ctx, &player.identity, &skill_name) {
        skill.experience += experience_gained;
        
        // Check for level up
        let required_exp = calculate_required_experience(skill.skill_level);
        if skill.experience >= required_exp {
            skill.skill_level += 1;
            skill.experience -= required_exp;
            log::info!("Player {} leveled up {} to level {}", player.username, skill_name, skill.skill_level);
        }
        
        skill.last_updated = ctx.timestamp;
        ctx.db.player_skills().skill_id().update(skill);
    } else {
        // Create new skill
        let skill_id = generate_unique_id(&ctx.sender, ctx.timestamp);
        ctx.db.player_skills().insert(PlayerSkill {
            skill_id,
            player_identity: player.identity,
            skill_name: skill_name.clone(),
            skill_level: 1,
            experience: experience_gained,
            last_updated: ctx.timestamp,
        });
        log::info!("Player {} started learning skill {}", player.username, skill_name);
    }
    
    Ok(())
}

/// Helper functions
fn find_inventory_item(
    ctx: &ReducerContext,
    player_identity: &Identity,
    item_id: &str
) -> Option<PlayerInventory> {
    ctx.db.player_inventory().iter()
        .find(|item| item.player_identity == *player_identity && item.item_id == item_id)
        .cloned()
}

fn create_new_inventory_slot(
    ctx: &ReducerContext,
    player_identity: &Identity,
    item_id: &str,
    quantity: u32
) -> Result<(), String> {
    // Find next available slot
    let used_slots: std::collections::HashSet<u32> = ctx.db.player_inventory().iter()
        .filter(|item| item.player_identity == *player_identity)
        .map(|item| item.slot_index)
        .collect();
    
    let mut next_slot = 0;
    while used_slots.contains(&next_slot) {
        next_slot += 1;
        if next_slot > 100 { // Max 100 inventory slots
            return Err("Inventory is full".to_string());
        }
    }
    
    let inventory_id = generate_unique_id(player_identity, ctx.timestamp);
    
    ctx.db.player_inventory().insert(PlayerInventory {
        inventory_id,
        player_identity: *player_identity,
        item_type: "item".to_string(), // TODO: Get from item definition
        item_id: item_id.to_string(),
        quantity,
        slot_index: next_slot,
    });
    
    Ok(())
}

fn find_player_skill(
    ctx: &ReducerContext,
    player_identity: &Identity,
    skill_name: &str
) -> Option<PlayerSkill> {
    ctx.db.player_skills().iter()
        .find(|skill| skill.player_identity == *player_identity && skill.skill_name == skill_name)
        .cloned()
}

fn calculate_required_experience(level: u32) -> u64 {
    // Simple exponential experience curve
    ((level as f64 * 100.0) * (1.2_f64).powi(level as i32)) as u64
}