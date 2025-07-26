use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// A helper for empty objects {}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyObject {}

//================================================================================
// Checkout Models
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkout {
    pub id: String,
    pub status: String, // PENDING, FAILED, PAID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    pub amount: f64,
    pub currency: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_code: Option<String>,
    pub date: DateTime<Utc>,
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
    // Fields from retrieve/list responses
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletedCheckout {
    pub id: String,
    pub status: String, // EXPIRED
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkout_reference: Option<String>,
    pub amount: f64,
    pub currency: String,
    pub merchant_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,
    pub date: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailablePaymentMethod {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailablePaymentMethodsResponse {
    pub available_payment_methods: Vec<AvailablePaymentMethod>,
}

//================================================================================
// Customer Models
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub customer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<PersonalDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>, // YYYY-MM-DD
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerRequest {
    pub personal_details: PersonalDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerRequest {
    pub personal_details: PersonalDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument {
    pub token: String,
    pub active: bool,
    #[serde(rename = "type")]
    pub instrument_type: String,
    pub card: Card,
    pub created_at: DateTime<Utc>,
}

//================================================================================
// Transaction Models
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub transaction_code: String,
    pub amount: f64,
    pub currency: String,
    pub timestamp: String, // Can be parsed to DateTime<Utc> if needed
    pub status: String,
    pub payment_type: String,
    pub merchant_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub installments_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tip_amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vat_amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionListResponse {
    pub transactions: Vec<Transaction>,
}

//================================================================================
// Merchant Models
//================================================================================

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

//================================================================================
// Payout Models
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payout {
    pub id: String,
    pub merchant_code: String,
    pub amount: f64,
    pub currency: String,
    pub status: String, // PENDING, PROCESSING, COMPLETED, FAILED
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub iban: String,
    pub bic: String,
    pub account_holder_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayoutListResponse {
    pub payouts: Vec<Payout>,
}

//================================================================================
// Receipt Models
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Receipt {
    pub id: String,
    pub transaction_id: String,
    pub merchant_code: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptListResponse {
    pub receipts: Vec<Receipt>,
}

//================================================================================
// Reader Models
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reader {
    pub id: String,
    pub serial_number: String,
    pub status: String, // ACTIVE, INACTIVE, LOST, STOLEN
    pub merchant_code: String,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReaderListResponse {
    pub readers: Vec<Reader>,
}

//================================================================================
// Membership Models
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub id: String,
    pub name: String,
    pub merchant_code: String,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMembershipRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMembershipRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipListResponse {
    pub memberships: Vec<Membership>,
}

//================================================================================
// Member Models
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: String,
    pub membership_id: String,
    pub email: String,
    pub status: String, // ACTIVE, INACTIVE, PENDING
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMemberRequest {
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberListResponse {
    pub members: Vec<Member>,
}

//================================================================================
// Role Models
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub membership_id: String,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleListResponse {
    pub roles: Vec<Role>,
}

//================================================================================
// Common Models (Card, Mandate, etc.)
//================================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    pub number: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub cvc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub last_4_digits: String,
    #[serde(rename = "type")]
    pub card_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mandate {
    #[serde(rename = "type")]
    pub mandate_type: String,
    pub status: String,
    pub merchant_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentInstrumentToken {
    pub token: String,
}

//================================================================================
// Pagination Models
//================================================================================

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