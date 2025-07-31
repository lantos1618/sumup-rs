use crate::{SumUpClient, Result, Payout, PayoutListResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PayoutListQuery {
    pub start_date: String, // Required: YYYY-MM-DD format
    pub end_date: String,   // Required: YYYY-MM-DD format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

impl SumUpClient {
    /// Lists all payouts for the authenticated merchant.
    ///
    /// # Arguments
    /// * `query` - Query parameters including required start_date and end_date
    /// 
    /// Note: This endpoint requires a merchant_code. Use list_merchant_payouts instead.
    /// The /v1.0/me/payouts endpoint does not exist in the SumUp API.
    #[deprecated(since = "0.2.0", note = "Use list_merchant_payouts instead. The /me/payouts endpoint does not exist.")]
    pub async fn list_payouts(&self, _query: &PayoutListQuery) -> Result<PayoutListResponse> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v1.0/me/payouts endpoint does not exist in the SumUp API. Use list_merchant_payouts instead.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
    }

    /// Lists payouts for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `query` - Query parameters including required start_date and end_date
    pub async fn list_merchant_payouts(&self, merchant_code: &str, query: &PayoutListQuery) -> Result<PayoutListResponse> {
        let url = self.build_url(&format!("/v1.0/merchants/{}/payouts", merchant_code))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .query(query)
            .send()
            .await?;

        if response.status().is_success() {
            let payouts = response.json::<PayoutListResponse>().await?;
            Ok(payouts)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves an identified payout resource.
    ///
    /// # Arguments
    /// * `payout_id` - The unique payout identifier
    pub async fn retrieve_payout(&self, payout_id: &str) -> Result<Payout> {
        let url = self.build_url(&format!("/v1.0/me/payouts/{}", payout_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let payout = response.json::<Payout>().await?;
            Ok(payout)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves a payout for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `payout_id` - The unique payout identifier
    pub async fn retrieve_merchant_payout(&self, merchant_code: &str, payout_id: &str) -> Result<Payout> {
        let url = self.build_url(&format!("/v1.0/merchants/{}/payouts/{}", merchant_code, payout_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let payout = response.json::<Payout>().await?;
            Ok(payout)
        } else {
            self.handle_error(response).await
        }
    }
} 