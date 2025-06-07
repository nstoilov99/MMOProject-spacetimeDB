//! Database cleanup utilities

use spacetimedb::ReducerContext;
use shared_module::*;
use crate::tables::*;

/// Clean up old chat messages to prevent database bloat
pub fn cleanup_old_chat_messages(ctx: &ReducerContext) {
    ChatMessage::cleanup_old_messages(ctx, MAX_CHAT_HISTORY);
}

/// Clean up sessions for users who haven't been active
pub fn cleanup_stale_sessions(ctx: &ReducerContext) {
    let timeout_duration = std::time::Duration::from_secs(INACTIVITY_TIMEOUT_SECONDS);
    let cutoff_time = ctx.timestamp - timeout_duration;
    
    let inactive_sessions = GameSession::get_inactive_sessions(ctx, cutoff_time);
    
    for session in inactive_sessions {
        // Mark associated player as offline
        Player::set_online_status(ctx, &session.identity, false, ctx.timestamp);
        
        // Remove the expired session
        GameSession::remove_session(ctx, &session.identity);
    }
}

/// Clean up players who have been offline for an extended period
pub fn cleanup_offline_players(ctx: &ReducerContext, offline_threshold_days: u64) {
    let threshold_duration = std::time::Duration::from_secs(offline_threshold_days * 24 * 60 * 60);
    let cutoff_time = ctx.timestamp - threshold_duration;
    
    let offline_players: Vec<Player> = ctx.db.game_players().iter()
        .filter(|p| !p.is_online && p.last_seen < cutoff_time)
        .cloned()
        .collect();
    
    let mut cleaned_count = 0;
    
    for player in offline_players {
        // In a real implementation, you might archive player data instead of deleting
        // For now, we'll just mark them for potential cleanup
        log::info!("Player {} has been offline for {} days", 
                  player.username, offline_threshold_days);
        cleaned_count += 1;
    }
    
    if cleaned_count > 0 {
        log::info!("Found {} players offline for more than {} days", 
                  cleaned_count, offline_threshold_days);
    }
}

/// Perform comprehensive database maintenance
pub fn perform_database_maintenance(ctx: &ReducerContext) {
    log::info!("Starting database maintenance...");
    
    // Clean up old chat messages
    cleanup_old_chat_messages(ctx);
    
    // Clean up stale sessions
    cleanup_stale_sessions(ctx);
    
    // Clean up long-offline players (30 days threshold)
    cleanup_offline_players(ctx, 30);
    
    log::info!("Database maintenance completed");
}

/// Get database statistics for monitoring
pub fn get_database_stats(ctx: &ReducerContext) -> DatabaseStats {
    let total_users = ctx.db.user().iter().count();
    let active_users = ctx.db.user().iter().filter(|u| u.is_active).count();
    let online_players = Player::get_online_players(ctx).len();
    let total_sessions = GameSession::get_all_sessions(ctx).len();
    let total_messages = ctx.db.chatmessage().iter().count();
    
    DatabaseStats {
        total_users,
        active_users,
        online_players,
        total_sessions,
        total_messages,
    }
}

/// Database statistics structure
#[derive(Clone, Debug)]
pub struct DatabaseStats {
    pub total_users: usize,
    pub active_users: usize,
    pub online_players: usize,
    pub total_sessions: usize,
    pub total_messages: usize,
}

/// Archive old data instead of deleting (for data retention policies)
pub fn archive_old_data(ctx: &ReducerContext, archive_threshold_days: u64) {
    let threshold_duration = std::time::Duration::from_secs(archive_threshold_days * 24 * 60 * 60);
    let cutoff_time = ctx.timestamp - threshold_duration;
    
    // Find old chat messages
    let old_messages: Vec<ChatMessage> = ctx.db.chatmessage().iter()
        .filter(|msg| msg.timestamp < cutoff_time)
        .cloned()
        .collect();
    
    if !old_messages.is_empty() {
        log::info!("Found {} old messages to archive", old_messages.len());
        
        // In a real implementation, you would:
        // 1. Export the messages to an archive system
        // 2. Delete them from the active database
        // For now, we'll just log the count
    }
}