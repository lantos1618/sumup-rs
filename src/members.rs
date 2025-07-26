use crate::{SumUpClient, Result, Member, CreateMemberRequest, UpdateMemberRequest, MemberListResponse};

impl SumUpClient {
    /// Lists all members for a specific membership.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    pub async fn list_members(&self, membership_id: &str) -> Result<MemberListResponse> {
        let url = self.build_url(&format!("/v0.1/memberships/{}/members", membership_id))?;

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

    /// Lists members for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    pub async fn list_merchant_members(&self, merchant_code: &str, membership_id: &str) -> Result<MemberListResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/members", merchant_code, membership_id))?;

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

    /// Creates a new member resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    /// * `body` - The member details to create
    pub async fn create_member(&self, membership_id: &str, body: &CreateMemberRequest) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/memberships/{}/members", membership_id))?;

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

    /// Creates a member for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    /// * `body` - The member details to create
    pub async fn create_merchant_member(&self, merchant_code: &str, membership_id: &str, body: &CreateMemberRequest) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/members", merchant_code, membership_id))?;

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

    /// Retrieves an identified member resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    /// * `member_id` - The unique member identifier
    pub async fn retrieve_member(&self, membership_id: &str, member_id: &str) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/memberships/{}/members/{}", membership_id, member_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let member = response.json::<Member>().await?;
            Ok(member)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves a member for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    /// * `member_id` - The unique member identifier
    pub async fn retrieve_merchant_member(&self, merchant_code: &str, membership_id: &str, member_id: &str) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/members/{}", merchant_code, membership_id, member_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let member = response.json::<Member>().await?;
            Ok(member)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates an identified member resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    /// * `member_id` - The unique member identifier
    /// * `body` - The member details to update
    pub async fn update_member(&self, membership_id: &str, member_id: &str, body: &UpdateMemberRequest) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/memberships/{}/members/{}", membership_id, member_id))?;

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

    /// Updates a member for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    /// * `member_id` - The unique member identifier
    /// * `body` - The member details to update
    pub async fn update_merchant_member(&self, merchant_code: &str, membership_id: &str, member_id: &str, body: &UpdateMemberRequest) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/members/{}", merchant_code, membership_id, member_id))?;

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

    /// Deletes an identified member resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    /// * `member_id` - The unique member identifier
    pub async fn delete_member(&self, membership_id: &str, member_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/memberships/{}/members/{}", membership_id, member_id))?;

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

    /// Deletes a member for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    /// * `member_id` - The unique member identifier
    pub async fn delete_merchant_member(&self, merchant_code: &str, membership_id: &str, member_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/members/{}", merchant_code, membership_id, member_id))?;

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