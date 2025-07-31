use crate::{SumUpClient, Result, Role, CreateRoleRequest, UpdateRoleRequest, RoleListResponse};

impl SumUpClient {
    /// Lists all roles for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    pub async fn list_roles(&self, merchant_code: &str) -> Result<RoleListResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles", merchant_code))?;

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

    /// Creates a new role resource for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    /// * `body` - The role details to create.
    pub async fn create_role(&self, merchant_code: &str, body: &CreateRoleRequest) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles", merchant_code))?;

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

    /// Retrieves an identified role resource for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    /// * `role_id` - The unique role identifier.
    pub async fn retrieve_role(&self, merchant_code: &str, role_id: &str) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles/{}", merchant_code, role_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let role = response.json::<Role>().await?;
            Ok(role)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates an identified role resource for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    /// * `role_id` - The unique role identifier.
    /// * `body` - The role details to update.
    pub async fn update_role(&self, merchant_code: &str, role_id: &str, body: &UpdateRoleRequest) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles/{}", merchant_code, role_id))?;

        let response = self
            .http_client
            .patch(url) // Note: API spec uses PATCH for updates here
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

    /// Deletes an identified role resource for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier.
    /// * `role_id` - The unique role identifier.
    pub async fn delete_role(&self, merchant_code: &str, role_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles/{}", merchant_code, role_id))?;

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