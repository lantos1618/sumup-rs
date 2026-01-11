use crate::Result;
use serde::{Deserialize, Serialize};

const AUTH_URL: &str = "https://api.sumup.com/authorize";
const TOKEN_URL: &str = "https://api.sumup.com/token";

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    Payments,
    TransactionsHistory,
    UserAppSettings,
    UserProfileReadonly,
    UserProfile,
    UserSubaccounts,
    UserPayoutSettings,
    Balance,
    Products,
}

impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Scope::Payments => "payments",
            Scope::TransactionsHistory => "transactions.history",
            Scope::UserAppSettings => "user.app-settings",
            Scope::UserProfileReadonly => "user.profile_readonly",
            Scope::UserProfile => "user.profile",
            Scope::UserSubaccounts => "user.subaccounts",
            Scope::UserPayoutSettings => "user.payout-settings",
            Scope::Balance => "balance",
            Scope::Products => "products",
        }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct OAuthClient {
    config: OAuthConfig,
    http_client: reqwest::Client,
}

impl OAuthClient {
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }

    /// Handle OAuth token response, converting errors to our error type.
    async fn handle_token_response(
        response: reqwest::Response,
        error_title: &str,
    ) -> Result<TokenResponse> {
        if response.status().is_success() {
            Ok(response.json::<TokenResponse>().await?)
        } else {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            Err(crate::Error::ApiError {
                status,
                body: crate::ApiErrorBody {
                    error_type: None,
                    title: Some(error_title.to_string()),
                    status: Some(status),
                    detail: Some(body),
                    error_code: None,
                    message: None,
                    param: None,
                    additional_fields: std::collections::HashMap::new(),
                },
            })
        }
    }

    /// Build the authorization URL to redirect users to.
    pub fn authorization_url(&self, scopes: &[Scope], state: Option<&str>) -> String {
        let scope_str: String = scopes.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");

        let mut url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}",
            AUTH_URL,
            urlencoding::encode(&self.config.client_id),
            urlencoding::encode(&self.config.redirect_uri),
        );

        if !scope_str.is_empty() {
            url.push_str(&format!("&scope={}", urlencoding::encode(&scope_str)));
        }

        if let Some(state) = state {
            url.push_str(&format!("&state={}", urlencoding::encode(state)));
        }

        url
    }

    /// Exchange an authorization code for access and refresh tokens.
    pub async fn exchange_code(&self, code: &str) -> Result<TokenResponse> {
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("redirect_uri", &self.config.redirect_uri),
        ];

        let response = self
            .http_client
            .post(TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        Self::handle_token_response(response, "OAuth token exchange failed").await
    }

    /// Refresh an access token using a refresh token.
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenResponse> {
        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
        ];

        let response = self
            .http_client
            .post(TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        Self::handle_token_response(response, "OAuth token refresh failed").await
    }

    /// Get tokens using client credentials (server-to-server, limited endpoint support).
    pub async fn client_credentials(&self, scopes: &[Scope]) -> Result<TokenResponse> {
        let scope_str: String = scopes.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" ");

        let params = [
            ("grant_type", "client_credentials"),
            ("client_id", self.config.client_id.as_str()),
            ("client_secret", self.config.client_secret.as_str()),
            ("scope", &scope_str),
        ];

        let response = self
            .http_client
            .post(TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        Self::handle_token_response(response, "OAuth client credentials failed").await
    }
}
