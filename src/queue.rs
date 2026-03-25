use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueMember {
    pub id: i64,
    pub discord_id: String,
    pub joined_at: NaiveDateTime,
    pub gender_preference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueStatus {
    pub is_active: bool,
    pub members: Vec<QueueMember>,
    pub started_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueJoinResponse {
    pub member: QueueMember,
    /// Some(discord_id) when an instant match was found.
    pub matched_discord_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinQueueRequest {
    pub discord_id: String,
    pub guild_id: Option<i64>,
    pub gender_preference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveQueueRequest {
    pub discord_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartQueueRequest {
    pub guild_id: i64,
    pub channel_id: i64,
    pub started_by: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopQueueRequest {
    pub guild_id: i64,
    pub stopped_by: String,
}
