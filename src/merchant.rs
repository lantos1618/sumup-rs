use crate::{Merchant, MerchantProfile, MerchantProfileDetails, Result, SumUpClient};

impl SumUpClient {
    /// Retrieves the authenticated merchant's profile.
    #[deprecated(since = "0.1.0", note = "The /v0.1/me endpoint is deprecated in the SumUp OpenAPI spec. Use list_memberships() to get merchant codes, then get_merchant() for details.")]
    pub async fn get_merchant_profile(&self) -> Result<MerchantProfileDetails> {
        let url = self.build_url("/v0.1/me")?;
        let response = self.http_client.get(url).bearer_auth(self.api_key_str()).send().await?;
        let full_profile: MerchantProfile = self.handle_response(response).await?;
        Ok(full_profile.merchant_profile)
    }

    /// Retrieves a specific merchant's profile.
    pub async fn get_merchant(&self, merchant_code: impl AsRef<str>) -> Result<Merchant> {
        let url = self.build_url(&format!("/v0.1/merchants/{}", merchant_code.as_ref()))?;
        let response = self.http_client.get(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_response(response).await
    }

    /// Lists all merchants the authenticated user is a member of.
    pub async fn list_merchants(&self) -> Result<Vec<crate::Membership>> {
        self.list_memberships().await
    }
}
