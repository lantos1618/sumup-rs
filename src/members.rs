use crate::{
    CreateMemberRequest, Member, MemberListResponse, Result, SumUpClient, UpdateMemberRequest,
};

impl SumUpClient {
    /// Lists all members for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    pub async fn list_members(&self, merchant_code: &str) -> Result<MemberListResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/members", merchant_code))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let members = response.json::<MemberListResponse>().await?;
            Ok(members)
        } else {
            self.handle_error(response).await
        }
    }

    /// Creates a new member resource for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    /// * `body` - The member details to create.
    pub async fn create_member(
        &self,
        merchant_code: &str,
        body: &CreateMemberRequest,
    ) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/members", merchant_code))?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let member = response.json::<Member>().await?;
            Ok(member)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves an identified member resource for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    /// * `member_id` - The unique member identifier.
    pub async fn retrieve_member(&self, merchant_code: &str, member_id: &str) -> Result<Member> {
        let url = self.build_url(&format!(
            "/v0.1/merchants/{}/members/{}",
            merchant_code, member_id
        ))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let member = response.json::<Member>().await?;
            Ok(member)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates an identified member resource for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    /// * `member_id` - The unique member identifier.
    /// * `body` - The member details to update.
    pub async fn update_member(
        &self,
        merchant_code: &str,
        member_id: &str,
        body: &UpdateMemberRequest,
    ) -> Result<Member> {
        let url = self.build_url(&format!(
            "/v0.1/merchants/{}/members/{}",
            merchant_code, member_id
        ))?;

        let response = self
            .http_client
            .put(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let member = response.json::<Member>().await?;
            Ok(member)
        } else {
            self.handle_error(response).await
        }
    }

    /// Deletes an identified member resource for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    /// * `member_id` - The unique member identifier.
    pub async fn delete_member(&self, merchant_code: &str, member_id: &str) -> Result<()> {
        let url = self.build_url(&format!(
            "/v0.1/merchants/{}/members/{}",
            merchant_code, member_id
        ))?;

        let response = self
            .http_client
            .delete(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            self.handle_error(response).await
        }
    }
}
