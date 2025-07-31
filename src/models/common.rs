use serde::{Deserialize, Serialize};

// A helper for empty objects {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyObject {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    pub number: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub cvv: String, // Corrected from `cvc` to `cvv` to match SumUp API specification
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub last_4_digits: String,
    #[serde(rename = "type")]
    pub card_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mandate {
    #[serde(rename = "type")]
    pub mandate_type: String,
    pub status: String,
    pub merchant_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInstrumentToken {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub rel: String,
    pub href: String,
    #[serde(rename = "type")]
    pub link_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
}
