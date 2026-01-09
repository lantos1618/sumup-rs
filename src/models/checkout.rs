use super::common::EmptyObject;
use super::common::{CardDetails, Mandate, MandateRequest, PaymentInstrumentToken};
use super::customer::PersonalDetails;
use super::enums::{CheckoutPurpose, CheckoutStatus, Currency, PaymentType};
use super::transaction::Transaction;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkout {
    pub id: String,
    pub status: CheckoutStatus,
    pub amount: f64,
    pub currency: Currency,
    pub date: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mandate: Option<Mandate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<Transaction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_instrument: Option<PaymentInstrumentToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCheckoutRequest {
    pub checkout_reference: String,
    pub amount: f64,
    pub currency: Currency,
    pub merchant_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CheckoutPurpose>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

impl CreateCheckoutRequest {
    pub fn new(checkout_reference: impl Into<String>, amount: f64, currency: impl Into<Currency>, merchant_code: impl Into<String>) -> Self {
        Self {
            checkout_reference: checkout_reference.into(),
            amount,
            currency: currency.into(),
            merchant_code: merchant_code.into(),
            description: None,
            return_url: None,
            customer_id: None,
            purpose: None,
            redirect_url: None,
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

    pub fn customer_id(mut self, id: impl Into<String>) -> Self {
        self.customer_id = Some(id.into());
        self
    }

    pub fn purpose(mut self, purpose: CheckoutPurpose) -> Self {
        self.purpose = Some(purpose);
        self
    }

    pub fn redirect_url(mut self, url: impl Into<String>) -> Self {
        self.redirect_url = Some(url.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CheckoutStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

/// Request to process a checkout payment (per OpenAPI spec)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProcessCheckoutRequest {
    pub payment_type: PaymentType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
    /// Mandate for recurrent payments
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<MandateRequest>,
}

impl Default for PaymentType {
    fn default() -> Self {
        Self::Card
    }
}

impl ProcessCheckoutRequest {
    pub fn card(card: CardDetails) -> Self {
        Self {
            payment_type: PaymentType::Card,
            card: Some(card),
            ..Default::default()
        }
    }

    pub fn token(token: impl Into<String>) -> Self {
        Self {
            payment_type: PaymentType::Card,
            token: Some(token.into()),
            ..Default::default()
        }
    }

    pub fn installments(mut self, count: i32) -> Self {
        self.installments = Some(count);
        self
    }

    pub fn customer_id(mut self, id: impl Into<String>) -> Self {
        self.customer_id = Some(id.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedCheckout {
    pub id: String,
    pub status: CheckoutStatus,
    pub amount: f64,
    pub currency: Currency,
    pub date: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<CheckoutPurpose>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<EmptyObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailablePaymentMethod {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailablePaymentMethodsResponse {
    pub available_payment_methods: Vec<AvailablePaymentMethod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProcessCheckoutResponse {
    /// Successful checkout completion (boxed to reduce enum size)
    Success(Box<Checkout>),
    /// Checkout requires additional steps (e.g., 3DS authentication)
    Accepted(CheckoutAccepted),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutAccepted {
    pub next_step: NextStep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextStep {
    pub url: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mechanism: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}
