use super::customer::Address;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Merchant {
    pub merchant_code: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Address,
    pub country: String,
    pub currency: String,
    pub timezone: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
