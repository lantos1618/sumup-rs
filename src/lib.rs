use reqwest::Client;
use url::Url;

// Re-export models for easier access
pub mod models;
pub use models::*;

// Declare modules for API endpoints
pub mod checkouts;
pub mod customers;
pub mod transactions;
pub mod merchant;
pub mod payouts;
pub mod receipts;
pub mod readers;
pub mod memberships;
pub mod members;
pub mod roles;

// --- Custom Error Type ---
#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Url(url::ParseError),
    // You can add more specific API errors here
    ApiError { status: u16, message: String },
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
            Error::ApiError { status, message } => write!(f, "API error {}: {}", status, message),
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
    /// Creates a new client for the SumUp API.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your SumUp API key (or OAuth token).
    /// * `use_sandbox` - Set to `true` to use the sandbox environment.
    pub fn new(api_key: String, use_sandbox: bool) -> Result<Self> {
        let base_url_str = if use_sandbox {
            "https://api.sumup.com" // NOTE: The docs state the same URL for sandbox but to use a sandbox key.
        } else {
            "https://api.sumup.com"
        };

        Ok(Self {
            http_client: Client::new(),
            api_key,
            base_url: Url::parse(base_url_str)?,
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

    /// Helper function to handle API error responses.
    pub(crate) async fn handle_error<T>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status().as_u16();
        let message = response.text().await.unwrap_or_default();
        Err(Error::ApiError { status, message })
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
