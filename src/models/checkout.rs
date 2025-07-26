use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::common::{CardDetails, Mandate, PaymentInstrumentToken};
use super::customer::PersonalDetails;
use super::transaction::Transaction;
use super::common::EmptyObject;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkout {
    pub id: String,
    pub status: String, // PENDING, FAILED, PAID
    pub amount: f64,
    pub currency: String,
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
    pub currency: String,
    pub merchant_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>, // CHECKOUT, SETUP_RECURRING_PAYMENT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessCheckoutRequest {
    pub payment_type: String, // card, boleto, ideal, etc.
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedCheckout {
    pub id: String,
    pub status: String, // EXPIRED
    pub amount: f64,
    pub currency: String,
    pub date: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
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