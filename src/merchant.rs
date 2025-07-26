use crate::{SumUpClient, Result, Merchant, MerchantProfile};

impl SumUpClient {
    /// Retrieves the authenticated merchant's profile.
    ///
    /// This endpoint returns the profile of the currently authenticated merchant.
    /// The merchant_code is automatically determined from the API key.
    pub async fn get_merchant_profile(&self) -> Result<MerchantProfile> {
        let url = self.build_url("/v0.1/me")?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let profile = response.json::<MerchantProfile>().await?;
            Ok(profile)
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

    /// Lists all merchants accessible to the authenticated user.
    ///
    /// This endpoint returns all merchants that the authenticated user has access to.
    /// This is typically used for users who manage multiple merchant accounts.
    pub async fn list_merchants(&self) -> Result<Vec<Merchant>> {
        let url = self.build_url("/v0.1/me/merchants")?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let merchants = response.json::<Vec<Merchant>>().await?;
            Ok(merchants)
        } else {
            self.handle_error(response).await
        }
    }
} 