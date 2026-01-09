use crate::{Payout, PayoutListResponse, Result, SumUpClient};
use serde::Serialize;

/// Query parameters for listing payouts (per OpenAPI spec)
#[derive(Debug, Clone, Serialize, Default)]
pub struct PayoutListQuery {
    /// Start date (ISO8601 format, required)
    pub start_date: String,
    /// End date (ISO8601 format, required)
    pub end_date: String,
    /// Response format (json or csv)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Maximum number of results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Sort order (desc or asc)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

impl PayoutListQuery {
    pub fn new(start_date: impl Into<String>, end_date: impl Into<String>) -> Self {
        Self {
            start_date: start_date.into(),
            end_date: end_date.into(),
            format: None,
            limit: None,
            order: None,
        }
    }

    pub fn format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn order_desc(mut self) -> Self {
        self.order = Some("desc".to_string());
        self
    }

    pub fn order_asc(mut self) -> Self {
        self.order = Some("asc".to_string());
        self
    }
}

impl SumUpClient {
    /// Lists payouts for a merchant.
    pub async fn list_merchant_payouts(&self, merchant_code: &str, query: &PayoutListQuery) -> Result<PayoutListResponse> {
        let url = self.build_url(&format!("/v1.0/merchants/{}/payouts", merchant_code))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).query(query).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves a payout by ID.
    pub async fn retrieve_payout(&self, payout_id: &str) -> Result<Payout> {
        let url = self.build_url(&format!("/v1.0/me/payouts/{}", payout_id))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves a payout for a specific merchant.
    pub async fn retrieve_merchant_payout(&self, merchant_code: &str, payout_id: &str) -> Result<Payout> {
        let url = self.build_url(&format!("/v1.0/merchants/{}/payouts/{}", merchant_code, payout_id))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }
}
