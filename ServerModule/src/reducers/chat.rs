//! Chat system reducers

use spacetimedb::{reducer, ReducerContext};
use shared_module::*;
use crate::tables::*;

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
    
    // Sanitize and validate message content using shared utility
    let sanitized_message = sanitize_chat_message(&message)?;
    
    // Validate channel
    let valid_channels = ["global", "zone", "guild", "party", "whisper"];
    if !valid_channels.contains(&channel.as_str()) {
        return Err("Invalid chat channel".to_string());
    }
    
    // Generate unique message ID
    let message_id = generate_unique_id(&ctx.sender, ctx.timestamp);
    
    // Create the message
    ChatMessage::create_message(
        ctx,
        message_id,
        ctx.sender,
        player.username,
        sanitized_message,
        channel,
        ctx.timestamp
    );
    
    // Update session activity
    GameSession::update_activity(ctx, &ctx.sender, ctx.timestamp);
    
    Ok(())
}

/// Get recent messages from a channel
#[reducer]
pub fn get_recent_messages(
    ctx: &ReducerContext,
    channel: String,
    limit: u32
) -> Result<Vec<ChatMessage>, String> {
    // Verify player exists
    Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Player not found")?;
    
    let limit = (limit as usize).min(MAX_CHAT_HISTORY); // Cap at max history
    
    let messages = ChatMessage::get_recent_messages(ctx, &channel, limit);
    Ok(messages)
}

/// Send a whisper (private message) to another player
#[reducer]
pub fn send_whisper(
    ctx: &ReducerContext,
    target_username: String,
    message: String
) -> Result<(), String> {
    // Verify sender is online
    let sender = Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Must be in game to send messages")?;
    
    if !sender.is_online {
        return Err("Must be online to send messages".to_string());
    }
    
    // Find target player
    let _target = ctx.db.game_players().iter()
        .find(|p| p.username == target_username && p.is_online)
        .ok_or("Target player not found or offline")?;
    
    // Sanitize message
    let sanitized_message = sanitize_chat_message(&message)?;
    
    // Create the whisper message with special channel format
    let whisper_channel = format!("whisper:{}:{}", sender.username, target_username);
    let message_id = generate_unique_id(&ctx.sender, ctx.timestamp);
    
    ChatMessage::create_message(
        ctx,
        message_id,
        ctx.sender,
        sender.username,
        sanitized_message,
        whisper_channel,
        ctx.timestamp
    );
    
    // Update session activity
    GameSession::update_activity(ctx, &ctx.sender, ctx.timestamp);
    
    Ok(())
}

/// Clean up old chat messages
#[reducer]
pub fn cleanup_chat_messages(ctx: &ReducerContext) -> Result<(), String> {
    // Only allow this to be called by system/admin
    // In a real implementation, you'd check for admin privileges
    
    ChatMessage::cleanup_old_messages(ctx, MAX_CHAT_HISTORY);
    
    log::info!("Chat message cleanup completed");
    Ok(())
}

/// Get whisper conversation between two players
#[reducer]
pub fn get_whisper_conversation(
    ctx: &ReducerContext,
    other_username: String,
    limit: u32
) -> Result<Vec<ChatMessage>, String> {
    // Verify requesting player exists
    let player = Player::filter_by_identity(ctx, &ctx.sender)
        .ok_or("Player not found")?;
    
    let limit = (limit as usize).min(50); // Cap whisper history
    
    // Find messages in both directions
    let channel1 = format!("whisper:{}:{}", player.username, other_username);
    let channel2 = format!("whisper:{}:{}", other_username, player.username);
    
    let mut messages: Vec<ChatMessage> = ctx.db.chatmessage().iter()
        .filter(|msg| msg.channel == channel1 || msg.channel == channel2)
        .cloned()
        .collect();
    
    // Sort by timestamp (newest first)
    messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    messages.truncate(limit);
    
    Ok(messages)
}