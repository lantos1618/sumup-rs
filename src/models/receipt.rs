use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub id: String,
    pub transaction_id: String,
    pub merchant_code: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptListResponse {
    pub receipts: Vec<Receipt>,
}
