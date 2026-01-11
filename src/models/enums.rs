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

/// Monetary amount in minor units (cents)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AmountMinor(pub i64);

impl AmountMinor {
    pub fn from_cents(cents: i64) -> Self {
        Self(cents)
    }

    pub fn from_major(major: f64) -> Self {
        Self((major * 100.0).round() as i64)
    }

    pub fn cents(&self) -> i64 {
        self.0
    }

    pub fn to_major(&self) -> f64 {
        self.0 as f64 / 100.0
    }
}

impl std::fmt::Display for AmountMinor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}", self.0 / 100, (self.0 % 100).abs())
    }
}
