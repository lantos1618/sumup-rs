use crate::{Result, SumUpClient};
use serde::{Deserialize, Serialize};

/// Webhook event types from SumUp.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WebhookEventType {
    CheckoutCompleted,
    CheckoutFailed,
    CheckoutExpired,
    PayoutCompleted,
    PayoutFailed,
    #[serde(other)]
    Unknown,
}

/// A webhook event from SumUp.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event_type: WebhookEventType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// Raw payload for fields not explicitly defined.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl WebhookEvent {
    /// Parse a webhook event from JSON bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(serde_json::from_slice(bytes)?)
    }

    /// Parse a webhook event from a JSON string.
    ///
    /// This is a convenience method that wraps the JSON parsing.
    /// You can also use the `FromStr` trait via `.parse()`.
    pub fn parse_json(s: &str) -> Result<Self> {
        Ok(serde_json::from_str(s)?)
    }
}

impl std::str::FromStr for WebhookEvent {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl SumUpClient {
    /// Verify a webhook event by fetching the resource from the API.
    ///
    /// SumUp recommends verifying webhooks by calling back to the API
    /// rather than using signature verification.
    ///
    /// Returns the verified checkout if valid.
    pub async fn verify_checkout_webhook(&self, checkout_id: &str) -> Result<crate::Checkout> {
        self.retrieve_checkout(checkout_id).await
    }
}

/// Helper to construct webhook responses.
pub struct WebhookResponse;

impl WebhookResponse {
    /// Returns the status code for a successful webhook receipt.
    /// SumUp expects any 2xx response.
    pub fn success_status() -> u16 {
        200
    }

    /// Returns an empty body for webhook responses.
    pub fn empty_body() -> &'static str {
        ""
    }
}
