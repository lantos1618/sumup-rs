use crate::{SumUpClient, Result, MembershipListResponse};

impl SumUpClient {
    /// Lists all memberships for the authenticated user.
    pub async fn list_memberships(&self) -> Result<MembershipListResponse> {
        let url = self.build_url("/v0.1/memberships")?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let memberships = response.json::<MembershipListResponse>().await?;
            Ok(memberships)
        } else {
            self.handle_error(response).await
        }
    }
    // Note: The SumUp API does not currently have endpoints for creating, updating, or deleting memberships directly.
    // These actions are typically handled within the SumUp dashboard.
    // The placeholder functions below are retained in case the API is extended in the future.
} 