use serde::{Deserialize, Serialize};

/// Checkout status (per OpenAPI spec)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckoutStatus {
    Pending,
    Paid,
    Failed,
    Cancelled,
    Expired,
}

impl Default for CheckoutStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for CheckoutStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::Paid => write!(f, "PAID"),
            Self::Failed => write!(f, "FAILED"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Expired => write!(f, "EXPIRED"),
        }
    }
}

/// Checkout purpose
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckoutPurpose {
    Checkout,
    SetupRecurringPayment,
}

/// Payment type for processing checkouts (per OpenAPI ProcessCheckout spec)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentType {
    Card,
    Boleto,
    Ideal,
    Blik,
    Bancontact,
    #[serde(other)]
    Other,
}

/// Payout status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayoutStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl std::fmt::Display for PayoutStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::Processing => write!(f, "PROCESSING"),
            Self::Completed => write!(f, "COMPLETED"),
            Self::Failed => write!(f, "FAILED"),
        }
    }
}

/// Card reader status (per OpenAPI spec - lowercase values)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderStatus {
    /// The reader status is unknown
    Unknown,
    /// The reader is created and waits for the physical device to confirm the pairing
    Processing,
    /// The reader is paired with a merchant account and can be used with SumUp APIs
    Paired,
    /// The pairing is expired and no longer usable with the account
    Expired,
}

impl std::fmt::Display for ReaderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Processing => write!(f, "processing"),
            Self::Paired => write!(f, "paired"),
            Self::Expired => write!(f, "expired"),
        }
    }
}

/// Reader device model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReaderDeviceModel {
    Solo,
    VirtualSolo,
}

/// Card type for reader checkout (credit vs debit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReaderCardType {
    Credit,
    Debit,
}

/// Transaction status (per OpenAPI spec)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Successful,
    Cancelled,
    Failed,
    Pending,
    Refunded,
    ChargeBack,
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Successful => write!(f, "SUCCESSFUL"),
            Self::Cancelled => write!(f, "CANCELLED"),
            Self::Failed => write!(f, "FAILED"),
            Self::Pending => write!(f, "PENDING"),
            Self::Refunded => write!(f, "REFUNDED"),
            Self::ChargeBack => write!(f, "CHARGE_BACK"),
        }
    }
}

/// Membership status (per OpenAPI spec - lowercase values)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MembershipStatus {
    Active,
    Pending,
    Declined,
    Revoked,
    #[serde(other)]
    Unknown,
}

impl Default for MembershipStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl std::fmt::Display for MembershipStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Pending => write!(f, "pending"),
            Self::Declined => write!(f, "declined"),
            Self::Revoked => write!(f, "revoked"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// Card type (Visa, Mastercard, etc.) - per OpenAPI spec
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardType {
    Amex,
    #[serde(rename = "CUP")]
    Cup,
    Diners,
    Discover,
    #[serde(rename = "ELO")]
    Elo,
    #[serde(rename = "ELV")]
    Elv,
    Hipercard,
    #[serde(rename = "JCB")]
    Jcb,
    Maestro,
    Mastercard,
    Visa,
    VisaElectron,
    VisaVpay,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for CardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amex => write!(f, "AMEX"),
            Self::Cup => write!(f, "CUP"),
            Self::Diners => write!(f, "DINERS"),
            Self::Discover => write!(f, "DISCOVER"),
            Self::Elo => write!(f, "ELO"),
            Self::Elv => write!(f, "ELV"),
            Self::Hipercard => write!(f, "HIPERCARD"),
            Self::Jcb => write!(f, "JCB"),
            Self::Maestro => write!(f, "MAESTRO"),
            Self::Mastercard => write!(f, "MASTERCARD"),
            Self::Visa => write!(f, "VISA"),
            Self::VisaElectron => write!(f, "VISA_ELECTRON"),
            Self::VisaVpay => write!(f, "VISA_VPAY"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

/// Mandate type for recurring payments (per OpenAPI spec)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateType {
    Recurrent,
    Oneoff,
    #[serde(other)]
    Other,
}

/// Mandate status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
    Cancelled,
}

/// Payment instrument type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentInstrumentType {
    Card,
    BankAccount,
    #[serde(other)]
    Other,
}

impl std::fmt::Display for PaymentInstrumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Card => write!(f, "card"),
            Self::BankAccount => write!(f, "bank_account"),
            Self::Other => write!(f, "other"),
        }
    }
}

/// Receipt status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReceiptStatus {
    Pending,
    Sent,
    Failed,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for ReceiptStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "PENDING"),
            Self::Sent => write!(f, "SENT"),
            Self::Failed => write!(f, "FAILED"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

/// Currency code (ISO 4217)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Currency(pub String);

impl Currency {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    // Common currencies
    pub const EUR: &'static str = "EUR";
    pub const GBP: &'static str = "GBP";
    pub const USD: &'static str = "USD";
    pub const CHF: &'static str = "CHF";
    pub const PLN: &'static str = "PLN";
    pub const BGN: &'static str = "BGN";
    pub const CZK: &'static str = "CZK";
    pub const HUF: &'static str = "HUF";
    pub const RON: &'static str = "RON";
    pub const SEK: &'static str = "SEK";
    pub const NOK: &'static str = "NOK";
    pub const DKK: &'static str = "DKK";
    pub const BRL: &'static str = "BRL";
    pub const CLP: &'static str = "CLP";
    pub const COP: &'static str = "COP";
}

impl Default for Currency {
    fn default() -> Self {
        Self(Self::EUR.to_string())
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Currency {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Currency {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&String> for Currency {
    fn from(s: &String) -> Self {
        Self(s.clone())
    }
}

impl From<&Currency> for Currency {
    fn from(c: &Currency) -> Self {
        c.clone()
    }
}

/// Monetary amount using exact decimal arithmetic.
///
/// Uses `rust_decimal::Decimal` internally to avoid floating-point precision issues.
/// Serializes as a decimal number (e.g., `10.50`) for API compatibility.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Amount(pub rust_decimal::Decimal);

impl Amount {
    /// Create an amount from a decimal value
    pub fn new(value: rust_decimal::Decimal) -> Self {
        Self(value)
    }

    /// Create an amount from cents (minor units)
    pub fn from_cents(cents: i64) -> Self {
        Self(rust_decimal::Decimal::new(cents, 2))
    }

    /// Create an amount from a major unit value (e.g., dollars/euros)
    pub fn from_major(value: i64) -> Self {
        Self(rust_decimal::Decimal::from(value))
    }

    /// Get the inner decimal value
    pub fn value(&self) -> rust_decimal::Decimal {
        self.0
    }

    /// Convert to cents (minor units), rounding if necessary
    pub fn to_cents(&self) -> i64 {
        use rust_decimal::prelude::ToPrimitive;
        (self.0 * rust_decimal::Decimal::from(100))
            .round()
            .to_i64()
            .unwrap_or(0)
    }

    /// Check if the amount is zero
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Check if the amount is positive
    pub fn is_positive(&self) -> bool {
        self.0.is_sign_positive() && !self.0.is_zero()
    }

    /// Check if the amount is negative
    pub fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for Amount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize as a number for API compatibility
        use rust_decimal::prelude::ToPrimitive;
        self.0.to_f64().unwrap_or(0.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use rust_decimal::prelude::FromPrimitive;
        let value = f64::deserialize(deserializer)?;
        let decimal = rust_decimal::Decimal::from_f64(value)
            .ok_or_else(|| serde::de::Error::custom("invalid decimal value"))?;
        Ok(Amount(decimal))
    }
}

impl From<rust_decimal::Decimal> for Amount {
    fn from(d: rust_decimal::Decimal) -> Self {
        Self(d)
    }
}

impl From<i64> for Amount {
    fn from(v: i64) -> Self {
        Self(rust_decimal::Decimal::from(v))
    }
}

impl From<i32> for Amount {
    fn from(v: i32) -> Self {
        Self(rust_decimal::Decimal::from(v))
    }
}

impl std::ops::Add for Amount {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Amount {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/// Strongly typed checkout ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CheckoutId(pub String);

impl CheckoutId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CheckoutId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CheckoutId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for CheckoutId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for CheckoutId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Strongly typed merchant code
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MerchantCode(pub String);

impl MerchantCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for MerchantCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for MerchantCode {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for MerchantCode {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<&MerchantCode> for MerchantCode {
    fn from(m: &MerchantCode) -> Self {
        m.clone()
    }
}

impl AsRef<str> for MerchantCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Strongly typed customer ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CustomerId(pub String);

impl CustomerId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CustomerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CustomerId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for CustomerId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<&String> for CustomerId {
    fn from(s: &String) -> Self {
        Self(s.clone())
    }
}

impl AsRef<str> for CustomerId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Strongly typed transaction ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TransactionId(pub String);

impl TransactionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TransactionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for TransactionId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for TransactionId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for TransactionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Country code (ISO 3166-1 alpha-2)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode(pub String);

impl CountryCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CountryCode {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for CountryCode {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
