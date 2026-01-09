use super::enums::{Currency, ReaderCardType, ReaderDeviceModel, ReaderStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Information about the underlying physical device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderDevice {
    /// A unique identifier of the physical device (e.g., serial number)
    pub identifier: String,
    /// Identifier of the model of the device
    pub model: ReaderDeviceModel,
}

/// A physical card reader device that can accept in-person payments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reader {
    /// Unique identifier of the reader (30 chars, e.g., "rdr_3MSAFM23CK82VSTT4BN6RWSQ65")
    pub id: String,
    /// Custom human-readable, user-defined name for easier identification
    pub name: String,
    /// The current status of the reader
    pub status: ReaderStatus,
    /// Information about the underlying physical device
    pub device: ReaderDevice,
    /// Optional metadata associated with the reader
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The timestamp of when the reader was created
    pub created_at: DateTime<Utc>,
    /// The timestamp of when the reader was last updated
    pub updated_at: DateTime<Utc>,
}

/// Response for listing readers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderListResponse {
    pub items: Vec<Reader>,
}

/// Request to create a new reader
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReaderRequest {
    /// The pairing code displayed on the SumUp device (8-9 alphanumeric chars)
    pub pairing_code: String,
    /// Custom name for the reader
    pub name: String,
    /// Optional metadata to associate with the reader
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl CreateReaderRequest {
    pub fn new(pairing_code: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            pairing_code: pairing_code.into(),
            name: name.into(),
            metadata: None,
        }
    }

    pub fn metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Request to update a reader
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateReaderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl UpdateReaderRequest {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Amount structure for reader checkouts.
/// The amount is represented as an integer value in minor units (e.g., cents).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalAmount {
    /// Integer value of the amount in minor units (e.g., 1000 = EUR 10.00)
    pub value: u64,
    /// Currency ISO 4217 code
    pub currency: Currency,
    /// The minor units of the currency (number of decimal places, usually 2)
    pub minor_unit: u8,
}

impl TotalAmount {
    /// Create a new TotalAmount from major units (e.g., euros/dollars)
    pub fn from_major(amount: f64, currency: impl Into<Currency>) -> Self {
        Self {
            value: (amount * 100.0).round() as u64,
            currency: currency.into(),
            minor_unit: 2,
        }
    }

    /// Create a new TotalAmount from minor units (e.g., cents)
    pub fn from_minor(value: u64, currency: impl Into<Currency>, minor_unit: u8) -> Self {
        Self {
            value,
            currency: currency.into(),
            minor_unit,
        }
    }

    /// Get the value in major units
    pub fn to_major(&self) -> f64 {
        self.value as f64 / 10f64.powi(self.minor_unit as i32)
    }
}

/// Affiliate metadata for tracking transaction sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affiliate {
    /// Application ID of the affiliate
    pub app_id: String,
    /// Affiliate key
    pub key: String,
    /// Foreign transaction ID for the affiliate
    pub foreign_transaction_id: String,
    /// Additional metadata tags
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
}

/// Request to create a reader checkout (in-person payment)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReaderCheckoutRequest {
    /// The total amount for the checkout
    pub total_amount: TotalAmount,
    /// Description of the checkout (shown in Merchant Sales)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Webhook URL for payment result notification (must be HTTPS)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// Number of installments (country-dependent, e.g., max 12 for Brazil)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
    /// Card type (required for some countries like Brazil)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<ReaderCardType>,
    /// Affiliate metadata for tracking
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<Affiliate>,
    /// List of tipping rates (0.01 to 0.99, sorted ascending)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_rates: Option<Vec<f32>>,
    /// Time in seconds for tip selection (30-120, default 30)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip_timeout: Option<i32>,
}

impl CreateReaderCheckoutRequest {
    pub fn new(total_amount: TotalAmount) -> Self {
        Self {
            total_amount,
            description: None,
            return_url: None,
            installments: None,
            card_type: None,
            affiliate: None,
            tip_rates: None,
            tip_timeout: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn return_url(mut self, url: impl Into<String>) -> Self {
        self.return_url = Some(url.into());
        self
    }

    pub fn installments(mut self, count: i32) -> Self {
        self.installments = Some(count);
        self
    }

    pub fn card_type(mut self, card_type: ReaderCardType) -> Self {
        self.card_type = Some(card_type);
        self
    }

    pub fn affiliate(mut self, affiliate: Affiliate) -> Self {
        self.affiliate = Some(affiliate);
        self
    }

    pub fn tip_rates(mut self, rates: Vec<f32>) -> Self {
        self.tip_rates = Some(rates);
        self
    }

    pub fn tip_timeout(mut self, timeout: i32) -> Self {
        self.tip_timeout = Some(timeout);
        self
    }
}

/// Response data from creating a reader checkout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderCheckoutResponseData {
    /// The client transaction ID (UUID) for fetching transaction details later
    pub client_transaction_id: String,
}

/// Response from creating a reader checkout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderCheckoutResponse {
    pub data: ReaderCheckoutResponseData,
}

/// Callback payload for reader checkout status changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderCheckoutStatusChange {
    /// Type of event (e.g., "solo.transaction.updated")
    pub event_type: String,
    /// Unique identifier for the event
    pub id: String,
    /// The event payload
    pub payload: ReaderCheckoutStatusPayload,
}

/// Payload for reader checkout status change events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderCheckoutStatusPayload {
    /// The unique client transaction ID
    pub client_transaction_id: String,
    /// The merchant code associated with the transaction
    pub merchant_code: String,
    /// The current status of the transaction
    pub status: ReaderCheckoutTransactionStatus,
    /// The transaction ID (deprecated, use client_transaction_id)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
}

/// Status of a reader checkout transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderCheckoutTransactionStatus {
    Successful,
    Failed,
}
