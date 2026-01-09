use super::enums::{Currency, ReaderCheckoutStatus, ReaderStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reader {
    pub id: String,
    pub serial_number: String,
    pub status: ReaderStatus,
    pub merchant_code: String,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderListResponse {
    pub readers: Vec<Reader>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReaderRequest {
    pub serial_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateReaderRequest {
    pub fn new(serial_number: impl Into<String>) -> Self {
        Self {
            serial_number: serial_number.into(),
            name: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateReaderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ReaderStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalAmount {
    pub value: f64,
    pub currency: Currency,
    pub minor_unit: i32,
}

impl TotalAmount {
    pub fn new(value: f64, currency: impl Into<Currency>) -> Self {
        Self {
            value,
            currency: currency.into(),
            minor_unit: 2,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReaderCheckoutRequest {
    pub total_amount: TotalAmount,
    pub description: String,
    pub return_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
}

impl CreateReaderCheckoutRequest {
    pub fn new(total_amount: TotalAmount, description: impl Into<String>, return_url: impl Into<String>) -> Self {
        Self {
            total_amount,
            description: description.into(),
            return_url: return_url.into(),
            installments: None,
            customer_id: None,
            external_reference: None,
        }
    }

    pub fn external_reference(mut self, reference: impl Into<String>) -> Self {
        self.external_reference = Some(reference.into());
        self
    }

    pub fn customer_id(mut self, id: impl Into<String>) -> Self {
        self.customer_id = Some(id.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderCheckoutResponse {
    pub id: String,
    pub status: ReaderCheckoutStatus,
    pub total_amount: TotalAmount,
    pub description: String,
    pub return_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
}
