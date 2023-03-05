use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    pub sender_id: i64,
    pub receiver_id: i64,
    pub chat_role: openai_api::api::ChatRole,
    pub content: String,
    pub time: i64,
}
