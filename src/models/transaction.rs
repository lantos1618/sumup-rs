use super::common::Link;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionHistoryResponse {
    pub items: Vec<Transaction>,
    #[serde(default)]
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub transaction_code: String,
    pub amount: f64,
    pub currency: String,
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<i64>,
    // ... add other fields from the transaction object as needed
}
