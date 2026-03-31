use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A finalized accepted connection stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMatch {
    pub id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

/// A pending match request stored in Redis with a 24-hour TTL.
/// The key is "converse:match_request:{lo_user_id}:{hi_user_id}".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingMatchRequest {
    /// Normalized: lo_user_id < hi_user_id
    pub lo_user_id: i64,
    pub hi_user_id: i64,
    /// Discord ID of the user who initiated the request
    pub from_discord_id: String,
    /// Discord ID of the target user
    pub to_discord_id: String,
    /// Optional guild context for the bot to use
    pub guild_id: Option<String>,
    /// Unix timestamp of when the request was created
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRequest {
    pub from_discord_id: String,
    pub to_discord_id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SwipeActionType {
    Skip,
    Like,
    Match,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwipeAction {
    pub user_id: i64,
    pub viewed_user_id: i64,
    pub action: SwipeActionType,
}

/// Returned from match-related endpoints. Represents either a pending request or an accepted connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResponse {
    pub match_id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub from_discord_id: String,
    pub to_discord_id: String,
    /// true = accepted connection in DB, false = pending request in Redis
    pub is_accepted: bool,
    pub created_at: NaiveDateTime,
}
