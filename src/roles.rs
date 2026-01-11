use crate::{CreateRoleRequest, Result, Role, RoleListResponse, SumUpClient, UpdateRoleRequest};

impl SumUpClient {
    /// Lists all roles for a merchant.
    pub async fn list_roles(&self, merchant_code: impl AsRef<str>) -> Result<RoleListResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles", merchant_code.as_ref()))?;
        let response = self.http_client.get(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_response(response).await
    }

    /// Creates a new role for a merchant.
    pub async fn create_role(&self, merchant_code: impl AsRef<str>, body: &CreateRoleRequest) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles", merchant_code.as_ref()))?;
        let response = self.http_client.post(url).bearer_auth(self.api_key_str()).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves a role by ID.
    pub async fn retrieve_role(&self, merchant_code: impl AsRef<str>, role_id: impl AsRef<str>) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles/{}", merchant_code.as_ref(), role_id.as_ref()))?;
        let response = self.http_client.get(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_response(response).await
    }

    /// Updates a role.
    pub async fn update_role(&self, merchant_code: impl AsRef<str>, role_id: impl AsRef<str>, body: &UpdateRoleRequest) -> Result<Role> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles/{}", merchant_code.as_ref(), role_id.as_ref()))?;
        let response = self.http_client.patch(url).bearer_auth(self.api_key_str()).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Deletes a role.
    pub async fn delete_role(&self, merchant_code: impl AsRef<str>, role_id: impl AsRef<str>) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/roles/{}", merchant_code.as_ref(), role_id.as_ref()))?;
        let response = self.http_client.delete(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_empty_response(response).await
    }
}
