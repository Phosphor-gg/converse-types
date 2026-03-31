use serde::{Deserialize, Serialize};

/// A member currently waiting in the queue (stored in Redis).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueMember {
    pub discord_id: String,
    pub guild_id: String,
    pub gender_preference: Option<String>,
    /// Unix timestamp of when the member joined
    pub joined_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub guild_id: String,
    pub members: Vec<QueueMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueJoinResponse {
    pub member: QueueMember,
    /// Some(discord_id) when an instant match was found.
    pub matched_discord_id: Option<String>,
    /// Some(match_id) when an instant match was found (DB row created).
    pub match_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinQueueRequest {
    pub discord_id: String,
    pub guild_id: String,
    pub gender_preference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveQueueRequest {
    pub discord_id: String,
    pub guild_id: String,
}
