use crate::{SumUpClient, Result, Membership, CreateMembershipRequest, UpdateMembershipRequest, MembershipListResponse};

impl SumUpClient {
    /// Lists all memberships for the authenticated merchant.
    pub async fn list_memberships(&self) -> Result<MembershipListResponse> {
        let url = self.build_url("/v0.1/me/memberships")?;

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

    /// Lists memberships for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    pub async fn list_merchant_memberships(&self, merchant_code: &str) -> Result<MembershipListResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships", merchant_code))?;

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

    /// Creates a new membership resource.
    ///
    /// # Arguments
    /// * `body` - The membership details to create
    pub async fn create_membership(&self, body: &CreateMembershipRequest) -> Result<Membership> {
        let url = self.build_url("/v0.1/me/memberships")?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let membership = response.json::<Membership>().await?;
            Ok(membership)
        } else {
            self.handle_error(response).await
        }
    }

    /// Creates a membership for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `body` - The membership details to create
    pub async fn create_merchant_membership(&self, merchant_code: &str, body: &CreateMembershipRequest) -> Result<Membership> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships", merchant_code))?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let membership = response.json::<Membership>().await?;
            Ok(membership)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves an identified membership resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    pub async fn retrieve_membership(&self, membership_id: &str) -> Result<Membership> {
        let url = self.build_url(&format!("/v0.1/me/memberships/{}", membership_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let membership = response.json::<Membership>().await?;
            Ok(membership)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves a membership for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    pub async fn retrieve_merchant_membership(&self, merchant_code: &str, membership_id: &str) -> Result<Membership> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}", merchant_code, membership_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let membership = response.json::<Membership>().await?;
            Ok(membership)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates an identified membership resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    /// * `body` - The membership details to update
    pub async fn update_membership(&self, membership_id: &str, body: &UpdateMembershipRequest) -> Result<Membership> {
        let url = self.build_url(&format!("/v0.1/me/memberships/{}", membership_id))?;

        let response = self
            .http_client
            .put(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let membership = response.json::<Membership>().await?;
            Ok(membership)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates a membership for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    /// * `body` - The membership details to update
    pub async fn update_merchant_membership(&self, merchant_code: &str, membership_id: &str, body: &UpdateMembershipRequest) -> Result<Membership> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}", merchant_code, membership_id))?;

        let response = self
            .http_client
            .put(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let membership = response.json::<Membership>().await?;
            Ok(membership)
        } else {
            self.handle_error(response).await
        }
    }

    /// Deletes an identified membership resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    pub async fn delete_membership(&self, membership_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/me/memberships/{}", membership_id))?;

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

    /// Deletes a membership for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    pub async fn delete_merchant_membership(&self, merchant_code: &str, membership_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}", merchant_code, membership_id))?;

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