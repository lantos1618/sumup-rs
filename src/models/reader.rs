use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReaderRequest {
    pub serial_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReaderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

// Reader Checkout specific models (different from online checkout)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalAmount {
    pub value: f64,
    pub currency: String,
    pub minor_unit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReaderCheckoutRequest {
    pub total_amount: TotalAmount,
    pub description: String,
    pub return_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderCheckoutResponse {
    pub id: String,
    pub status: String, // PENDING, COMPLETED, FAILED, CANCELLED
    pub total_amount: TotalAmount,
    pub description: String,
    pub return_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}
