use crate::{Membership, MembershipListResponse, Result, SumUpClient};

impl SumUpClient {
    /// Lists all memberships for the authenticated user.
    pub async fn list_memberships(&self) -> Result<Vec<Membership>> {
        let url = self.build_url("/v0.1/memberships")?;
        let response = self.http_client.get(url).bearer_auth(self.api_key_str()).send().await?;
        let resp: MembershipListResponse = self.handle_response(response).await?;
        Ok(resp.items)
    }
}
