use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reader {
    pub id: String,
    pub serial_number: String,
    pub status: String, // ACTIVE, INACTIVE, LOST, STOLEN
    pub merchant_code: String,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderListResponse {
    pub readers: Vec<Reader>,
} 