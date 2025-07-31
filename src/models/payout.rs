use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub id: String,
    pub merchant_code: String,
    pub amount: f64,
    pub currency: String,
    pub status: String, // PENDING, PROCESSING, COMPLETED, FAILED
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub iban: String,
    pub bic: String,
    pub account_holder_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutListResponse {
    pub payouts: Vec<Payout>,
}
