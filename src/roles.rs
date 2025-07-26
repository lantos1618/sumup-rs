use crate::{SumUpClient, Result, Role, CreateRoleRequest, UpdateRoleRequest, RoleListResponse};

impl SumUpClient {
    /// Lists all roles for a specific membership.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    pub async fn list_roles(&self, membership_id: &str) -> Result<RoleListResponse> {
        let url = self.build_url(&format!("/v0.1/me/memberships/{}/roles", membership_id))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let roles = response.json::<RoleListResponse>().await?;
            Ok(roles)
        } else {
            self.handle_error(response).await
        }
    }

    /// Lists roles for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    pub async fn list_merchant_roles(&self, merchant_code: &str, membership_id: &str) -> Result<RoleListResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/roles", merchant_code, membership_id))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let roles = response.json::<RoleListResponse>().await?;
            Ok(roles)
        } else {
            self.handle_error(response).await
        }
    }

    /// Creates a new role resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    /// * `body` - The role details to create
    pub async fn create_role(&self, membership_id: &str, body: &CreateRoleRequest) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/me/memberships/{}/roles", membership_id))?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let role = response.json::<Role>().await?;
            Ok(role)
        } else {
            self.handle_error(response).await
        }
    }

    /// Creates a role for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    /// * `body` - The role details to create
    pub async fn create_merchant_role(&self, merchant_code: &str, membership_id: &str, body: &CreateRoleRequest) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/roles", merchant_code, membership_id))?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let role = response.json::<Role>().await?;
            Ok(role)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves an identified role resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    /// * `role_id` - The unique role identifier
    pub async fn retrieve_role(&self, membership_id: &str, role_id: &str) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/me/memberships/{}/roles/{}", membership_id, role_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let role = response.json::<Role>().await?;
            Ok(role)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves a role for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    /// * `role_id` - The unique role identifier
    pub async fn retrieve_merchant_role(&self, merchant_code: &str, membership_id: &str, role_id: &str) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/roles/{}", merchant_code, membership_id, role_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let role = response.json::<Role>().await?;
            Ok(role)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates an identified role resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    /// * `role_id` - The unique role identifier
    /// * `body` - The role details to update
    pub async fn update_role(&self, membership_id: &str, role_id: &str, body: &UpdateRoleRequest) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/me/memberships/{}/roles/{}", membership_id, role_id))?;

        let response = self
            .http_client
            .put(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let role = response.json::<Role>().await?;
            Ok(role)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates a role for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    /// * `role_id` - The unique role identifier
    /// * `body` - The role details to update
    pub async fn update_merchant_role(&self, merchant_code: &str, membership_id: &str, role_id: &str, body: &UpdateRoleRequest) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/roles/{}", merchant_code, membership_id, role_id))?;

        let response = self
            .http_client
            .put(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let role = response.json::<Role>().await?;
            Ok(role)
        } else {
            self.handle_error(response).await
        }
    }

    /// Deletes an identified role resource.
    ///
    /// # Arguments
    /// * `membership_id` - The unique membership identifier
    /// * `role_id` - The unique role identifier
    pub async fn delete_role(&self, membership_id: &str, role_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/me/memberships/{}/roles/{}", membership_id, role_id))?;

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

    /// Deletes a role for a specific merchant membership.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `membership_id` - The unique membership identifier
    /// * `role_id` - The unique role identifier
    pub async fn delete_merchant_role(&self, merchant_code: &str, membership_id: &str, role_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/memberships/{}/roles/{}", merchant_code, membership_id, role_id))?;

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