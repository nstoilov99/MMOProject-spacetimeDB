// server/src/modules/chat.rs
use crate::*;
use spacetimedb::{reducer, ReducerContext};

/// Send a chat message
#[reducer]
pub fn send_chat_message(
    ctx: &ReducerContext,
    message: String,
    channel: String
) -> Result<(), String> {
    // Verify player is online
    let player = Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Must be in game to send messages")?;
    
    if !player.is_online {
        return Err("Must be online to send messages".to_string());
    }
    
    // Validate message content
    let trimmed_message = message.trim();
    if trimmed_message.is_empty() {
        return Err("Message cannot be empty".to_string());
    }
    
    if trimmed_message.len() > 500 {
        return Err("Message too long (max 500 characters)".to_string());
    }
    
    // Validate channel
    let valid_channels = ["global", "zone", "guild", "party"];
    if !valid_channels.contains(&channel.as_str()) {
        return Err("Invalid chat channel".to_string());
    }
    
    // Generate unique message ID using hash of multiple factors
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    format!("{:?}", ctx.sender).hash(&mut hasher);
    format!("{:?}", ctx.timestamp).hash(&mut hasher);
    trimmed_message.hash(&mut hasher);
    channel.hash(&mut hasher);
    let message_id = hasher.finish();
    
    // Insert the message
    ctx.db.chatmessage().insert(ChatMessage {
        message_id,
        sender_identity: ctx.sender,
        sender_username: player.username,
        message: trimmed_message.to_string(),
        channel,
        timestamp: ctx.timestamp,
    });
    
    // Update session activity to show the player is still active
    if let Some(session) = GameSession::filter_by_identity(ctx, &ctx.sender) {
        let mut updated_session = session;
        updated_session.last_activity = ctx.timestamp;
        ctx.db.gamesession().identity().update(updated_session);
    }
    
    Ok(())
}

/// Helper function to get recent messages from a channel
/// Note: This is not a reducer - clients should query the database directly using:
/// SELECT * FROM chatmessage WHERE channel = ? ORDER BY timestamp DESC LIMIT ?
pub fn get_recent_messages_helper(
    ctx: &ReducerContext,
    channel: String,
    limit: u32
) -> Result<Vec<ChatMessage>, String> {
    // Verify player exists
    Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Player not found")?;
    
    let limit = limit.min(50); // Cap at 50 messages
    
    // Get messages from the specified channel
    let mut messages: Vec<ChatMessage> = ctx.db.chatmessage().iter()
        .filter(|msg| msg.channel == channel)
        .collect();
    
    // Sort by timestamp (newest first)
    messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    
    // Take the requested number of messages
    messages.truncate(limit as usize);
    
    Ok(messages)
}