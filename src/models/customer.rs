use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::common::Card;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub customer_id: String,
    #[serde(default)]
    pub personal_details: Option<PersonalDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonalDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>, // YYYY-MM-DD
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

// NOTE: The API requires a customer_id in the body when creating a customer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    pub customer_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument {
    pub token: String,
    pub active: bool,
    #[serde(rename = "type")]
    pub instrument_type: String,
    #[serde(default)]
    pub card: Option<Card>,
    pub created_at: DateTime<Utc>,
} 