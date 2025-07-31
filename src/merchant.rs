use crate::{SumUpClient, Result, Merchant, MerchantProfile, MerchantProfileDetails};

impl SumUpClient {
    /// Retrieves the authenticated merchant's profile.
    ///
    /// This endpoint returns the profile of the currently authenticated merchant.
    /// The merchant_code is automatically determined from the API key.
    pub async fn get_merchant_profile(&self) -> Result<MerchantProfileDetails> {
        let url = self.build_url("/v0.1/me")?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let full_profile = response.json::<MerchantProfile>().await?;
            Ok(full_profile.merchant_profile)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves a specific merchant's profile.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    pub async fn get_merchant(&self, merchant_code: &str) -> Result<Merchant> {
        let url = self.build_url(&format!("/v0.1/merchants/{}", merchant_code))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let merchant = response.json::<Merchant>().await?;
            Ok(merchant)
        } else {
            self.handle_error(response).await
        }
    }

    /// Lists all merchant accounts the authenticated user is a member of.
    /// This is the correct way to "list merchants" according to the API spec.
    /// 
    /// Note: This uses the memberships endpoint as the API doesn't have a direct
    /// "list merchants" endpoint. The memberships contain merchant information.
    pub async fn list_merchants(&self) -> Result<Vec<crate::Membership>> {
        self.list_memberships().await
    }
} 