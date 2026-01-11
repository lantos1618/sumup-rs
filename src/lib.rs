#![allow(clippy::result_large_err)]

use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use std::time::Duration;
use url::Url;

// Re-export models for easier access
pub mod models;
pub use models::*;

// Declare modules for API endpoints
pub mod checkouts;
pub mod customers;
pub mod members;
pub mod memberships;
pub mod merchant;
pub mod oauth;
pub mod payouts;
pub mod readers;
pub mod receipts;
pub mod roles;
pub mod subaccounts;
pub mod transactions;
pub mod webhooks;

// Re-export OAuth types
pub use oauth::{OAuthClient, OAuthConfig, Scope, TokenResponse};

// Re-export Subaccount types
pub use subaccounts::{CreateOperatorRequest, Operator, UpdateOperatorRequest};

// Re-export Webhook types
pub use webhooks::{WebhookEvent, WebhookEventType, WebhookResponse};

// Re-export query types for convenience
pub use transactions::TransactionHistoryQuery;

// --- Custom Error Type ---
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("API error {status}: {}", .body.detail.as_ref().or(.body.message.as_ref()).unwrap_or(&"Unknown error".to_string()))]
    ApiError { status: u16, body: ApiErrorBody },

    #[error("Rate limited. Retry after {retry_after} seconds")]
    RateLimit { retry_after: u64 },

    #[error("Authentication failed: {0}")]
    Unauthorized(String),
}

/// Structured representation of SumUp API error responses
#[derive(Debug, serde::Deserialize)]
pub struct ApiErrorBody {
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub title: Option<String>,
    pub status: Option<u16>,
    pub detail: Option<String>,
    pub error_code: Option<String>,
    pub message: Option<String>,
    pub param: Option<String>,
    #[serde(flatten)]
    pub additional_fields: std::collections::HashMap<String, serde_json::Value>,
}

pub type Result<T> = std::result::Result<T, Error>;

// --- Client Builder ---

/// Builder for configuring a SumUpClient
#[derive(Default)]
pub struct SumUpClientBuilder {
    api_key: Option<SecretString>,
    base_url: Option<String>,
    timeout: Option<Duration>,
}

impl SumUpClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the API key (required)
    pub fn api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(SecretString::new(key.into()));
        self
    }

    /// Set a custom base URL (default: https://api.sumup.com)
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set request timeout (default: 30 seconds)
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Build the client
    pub fn build(self) -> Result<SumUpClient> {
        let api_key = self
            .api_key
            .ok_or_else(|| Error::Config("API key is required".into()))?;

        let base_url = self
            .base_url
            .unwrap_or_else(|| SumUpClient::BASE_URL.to_string());

        let timeout = self.timeout.unwrap_or(Duration::from_secs(30));

        let http_client = Client::builder()
            .timeout(timeout)
            .build()
            .map_err(Error::Http)?;

        Ok(SumUpClient {
            http_client,
            api_key,
            base_url: Url::parse(&base_url)?,
        })
    }
}

// --- The Main Client ---
pub struct SumUpClient {
    pub(crate) http_client: Client,
    pub(crate) api_key: SecretString,
    pub(crate) base_url: Url,
}

impl SumUpClient {
    const BASE_URL: &'static str = "https://api.sumup.com";

    /// Create a new builder for SumUpClient
    pub fn builder() -> SumUpClientBuilder {
        SumUpClientBuilder::new()
    }

    /// Creates a new client for the SumUp API.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your SumUp API key (or OAuth token).
    /// * `_use_sandbox` - Ignored. SumUp uses the same URL for sandbox/production;
    ///   the environment is determined by your API key type.
    ///
    /// # Deprecated
    /// Prefer using `SumUpClient::builder().api_key(key).build()` instead.
    pub fn new(api_key: impl Into<String>, _use_sandbox: bool) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Creates a new client with a custom base URL.
    #[deprecated(note = "Use SumUpClient::builder().api_key(key).base_url(url).build() instead")]
    pub fn with_custom_url(api_key: impl Into<String>, base_url: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).base_url(base_url).build()
    }

    pub(crate) fn build_url(&self, path: &str) -> Result<Url> {
        Ok(self.base_url.join(path)?)
    }

    /// Get the API key for use in requests (internal use only)
    pub(crate) fn api_key_str(&self) -> &str {
        self.api_key.expose_secret()
    }

    /// Handle response - parse JSON on success, error on failure.
    pub(crate) async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T> {
        if response.status().is_success() {
            Ok(response.json::<T>().await?)
        } else {
            self.handle_error(response).await
        }
    }

    /// Handle response that returns no body (204 No Content, etc).
    pub(crate) async fn handle_empty_response(&self, response: reqwest::Response) -> Result<()> {
        if response.status().is_success() {
            Ok(())
        } else {
            self.handle_error(response).await
        }
    }

    /// Helper function to handle API error responses.
    pub(crate) async fn handle_error<T>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status().as_u16();

        // Check for rate limiting
        if status == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok())
                .unwrap_or(60);
            return Err(Error::RateLimit { retry_after });
        }

        // Check for auth errors
        if status == 401 {
            let body = response.text().await.unwrap_or_default();
            return Err(Error::Unauthorized(body));
        }

        let response_text = response.text().await.unwrap_or_default();

        let body = serde_json::from_str::<ApiErrorBody>(&response_text).unwrap_or_else(|_| {
            ApiErrorBody {
                error_type: None,
                title: None,
                status: Some(status),
                detail: Some(response_text),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        });

        Err(Error::ApiError { status, body })
    }

    /// Get the base URL being used by the client.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}

// Implement Debug manually to avoid exposing the API key
impl std::fmt::Debug for SumUpClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SumUpClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}
