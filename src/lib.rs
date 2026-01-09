#![allow(clippy::result_large_err)]
use reqwest::Client;
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
#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Url(url::ParseError),
    // Structured API error with parsed response body
    ApiError { status: u16, body: ApiErrorBody },
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
    // Sometimes the API returns additional context
    #[serde(flatten)]
    pub additional_fields: std::collections::HashMap<String, serde_json::Value>,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::Url(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {}", e),
            Error::Json(e) => write!(f, "JSON error: {}", e),
            Error::Url(e) => write!(f, "URL error: {}", e),
            Error::ApiError { status, body } => {
                // Try to provide the most useful error message
                let status_str = status.to_string();
                let message = body
                    .detail
                    .as_ref()
                    .or(body.message.as_ref())
                    .or(body.title.as_ref())
                    .unwrap_or(&status_str);
                write!(f, "API error {}: {}", status, message)
            }
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// --- The Main Client ---
pub struct SumUpClient {
    pub(crate) http_client: Client,
    pub(crate) api_key: String,
    pub(crate) base_url: Url,
}

impl SumUpClient {
    const BASE_URL: &'static str = "https://api.sumup.com";

    /// Creates a new client for the SumUp API.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your SumUp API key (or OAuth token).
    /// * `_use_sandbox` - Ignored. SumUp uses the same URL for sandbox/production;
    ///   the environment is determined by your API key type.
    pub fn new(api_key: String, _use_sandbox: bool) -> Result<Self> {
        Ok(Self {
            http_client: Client::new(),
            api_key,
            base_url: Url::parse(Self::BASE_URL)?,
        })
    }

    /// Creates a new client with a custom base URL.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your SumUp API key (or OAuth token).
    /// * `base_url` - Custom base URL for the API.
    pub fn with_custom_url(api_key: String, base_url: String) -> Result<Self> {
        Ok(Self {
            http_client: Client::new(),
            api_key,
            base_url: Url::parse(&base_url)?,
        })
    }

    pub(crate) fn build_url(&self, path: &str) -> Result<Url> {
        Ok(self.base_url.join(path)?)
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

    /// Get the current API key being used by the client.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get the base URL being used by the client.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }
}
