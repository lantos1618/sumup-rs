use super::customer::Address;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Merchant account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merchant {
    pub merchant_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    /// Catch any extra fields
    #[serde(flatten)]
    pub extra: Option<std::collections::HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantProfile {
    #[serde(rename = "merchant_profile")]
    pub merchant_profile: MerchantProfileDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerchantProfileDetails {
    #[serde(rename = "merchant_code")]
    pub merchant_code: String,
    #[serde(rename = "company_name")]
    pub name: String,
    #[serde(rename = "default_currency")]
    pub currency: String,
    pub country: String,
    #[serde(rename = "mobile_phone")]
    pub phone: Option<String>,
    pub address: Address,
    pub website: Option<String>,
    #[serde(rename = "doing_business_as")]
    pub doing_business_as: Option<DoingBusinessAs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoingBusinessAs {
    pub email: String,
    #[serde(rename = "business_name")]
    pub name: String,
    pub website: Option<String>,
    pub address: Address,
}
