use super::enums::{CardType, MandateStatus, MandateType};
use serde::{Deserialize, Serialize};

// A helper for empty objects {}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmptyObject {}

/// Card details for ProcessCheckout request (per OpenAPI spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    /// Card number
    pub number: String,
    /// Expiry month (MM format)
    pub expiry_month: String,
    /// Expiry year (YYYY format)
    pub expiry_year: String,
    /// CVV security code
    pub cvv: String,
    /// Cardholder name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Card type (VISA, MASTERCARD, etc.)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub card_type: Option<CardType>,
    /// Zip/postal code for AVS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

impl CardDetails {
    /// Create new card details with required fields
    pub fn new(
        number: impl Into<String>,
        expiry_month: impl Into<String>,
        expiry_year: impl Into<String>,
        cvv: impl Into<String>,
    ) -> Self {
        Self {
            number: number.into(),
            expiry_month: expiry_month.into(),
            expiry_year: expiry_year.into(),
            cvv: cvv.into(),
            name: None,
            card_type: None,
            zip_code: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn card_type(mut self, card_type: CardType) -> Self {
        self.card_type = Some(card_type);
        self
    }

    pub fn zip_code(mut self, zip_code: impl Into<String>) -> Self {
        self.zip_code = Some(zip_code.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub last_4_digits: String,
    #[serde(rename = "type")]
    pub card_type: CardType,
}

/// Mandate response (returned from API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mandate {
    #[serde(rename = "type")]
    pub mandate_type: MandateType,
    pub status: MandateStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
}

/// Mandate request for ProcessCheckout (per OpenAPI spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandateRequest {
    #[serde(rename = "type")]
    pub mandate_type: MandateType,
    pub user_agent: String,
    pub user_ip: String,
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
