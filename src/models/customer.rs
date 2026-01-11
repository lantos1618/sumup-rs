use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    /// Birth date in ISO 8601 format (YYYY-MM-DD), e.g. "1990-01-15"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,
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
    pub address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
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

/// Payment instrument response (per OpenAPI PaymentInstrumentResponse schema)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument {
    /// Unique token for the saved card
    pub token: String,
    /// Card type (VISA, MASTERCARD, etc.)
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
    /// Last 4 digits of the card
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_4_digits: Option<String>,
    /// Expiry month (MM)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<String>,
    /// Expiry year (YYYY)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<String>,
    /// Cardholder name
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    /// Whether the instrument is active
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Creation timestamp
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
}
