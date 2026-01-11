use super::enums::{Amount, Currency, MerchantCode};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Payout object (fields based on FinancialPayouts schema)
/// Note: Made flexible with Option types since exact schema is not fully documented
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Transaction code (for some response formats)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<MerchantCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Bank account details
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,
    /// Catch any extra fields
    #[serde(flatten)]
    pub extra: Option<std::collections::HashMap<String, serde_json::Value>>,
}

/// Bank account information for payouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
}

/// Response for listing payouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutListResponse {
    /// List of payouts (field name may vary by endpoint)
    #[serde(default, alias = "payouts", alias = "data")]
    pub items: Vec<Payout>,
}
