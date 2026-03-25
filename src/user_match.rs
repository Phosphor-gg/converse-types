use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMatch {
    pub id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub is_accepted: bool,
    pub is_active: bool,
    pub thread_channel_id: Option<i64>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRequest {
    pub from_discord_id: String,
    pub to_discord_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMatchRequest {
    pub from_user_id: i64,
    pub to_user_id: i64,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResponse {
    pub match_id: i64,
    pub from_user_id: i64,
    pub to_user_id: i64,
    pub from_discord_id: String,
    pub to_discord_id: String,
    pub is_accepted: bool,
    pub created_at: NaiveDateTime,
}
