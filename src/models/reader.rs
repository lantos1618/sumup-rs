use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderDevice {
    pub identifier: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reader {
    pub id: String,
    pub name: String,
    pub status: String, // unknown, processing, paired, expired
    pub device: ReaderDevice,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderListResponse {
    pub items: Vec<Reader>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReaderRequest {
    pub pairing_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReaderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// Reader Checkout specific models (different from online checkout)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalAmount {
    pub value: u32,
    pub currency: String,
    pub minor_unit: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReaderCheckoutRequest {
    pub total_amount: TotalAmount,
    pub description: String,
    pub return_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderCheckoutResponse {
    pub data: ReaderCheckoutResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderCheckoutResponseData {
    pub client_transaction_id: String,
}
