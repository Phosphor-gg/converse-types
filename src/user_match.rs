use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A finalized accepted connection stored in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMatch {
    pub id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub thread_channel_id: Option<i64>,
    pub created_at: NaiveDateTime,
}

/// A pending match request stored in Redis with a 24-hour TTL.
/// Keyed by `token` (UUID v4). A reverse lookup from (lo_id, hi_id) → token is
/// also stored so mutual matches can be detected without scanning Redis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingMatchRequest {
    /// Opaque UUID identifying this request.
    pub token: String,
    /// Normalized: lo_user_id < hi_user_id (internal DB user IDs).
    pub lo_user_id: i64,
    pub hi_user_id: i64,
    /// Discord ID of the user who initiated the request.
    pub from_discord_id: String,
    /// Discord ID of the target user.
    pub to_discord_id: String,
    /// Unix timestamp of when the request was created.
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

/// Returned from match-related endpoints.
///
/// When `match_id` is `Some`: the match exists in the DB (accepted connection).
/// When `pending_token` is `Some`: the request is pending in Redis (awaiting the other user).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResponse {
    /// Database row ID — `Some` only when the match is accepted.
    pub match_id: Option<i64>,
    /// Opaque UUID token — `Some` only when the request is pending in Redis.
    pub pending_token: Option<String>,
    pub from_discord_id: String,
    pub to_discord_id: String,
    pub created_at: NaiveDateTime,
}
