use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::customer::Address;

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
    pub merchant_code: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: Address,
    pub country: String,
    pub currency: String,
    pub timezone: String,
} 