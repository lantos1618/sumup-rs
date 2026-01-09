use crate::{Payout, PayoutListResponse, Result, SumUpClient};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PayoutListQuery {
    pub start_date: String,
    pub end_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
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
