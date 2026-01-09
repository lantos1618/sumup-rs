use crate::{Result, SumUpClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub id: String,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateOperatorRequest {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpdateOperatorRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl SumUpClient {
    /// Lists all operators (subaccounts). [DEPRECATED per OpenAPI spec]
    pub async fn list_operators(&self) -> Result<Vec<Operator>> {
        let url = self.build_url("/v0.1/me/accounts")?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }

    /// Creates a new operator (subaccount). [DEPRECATED per OpenAPI spec]
    pub async fn create_operator(&self, body: &CreateOperatorRequest) -> Result<Operator> {
        let url = self.build_url("/v0.1/me/accounts")?;
        let response = self.http_client.post(url).bearer_auth(&self.api_key).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves an operator by ID. [DEPRECATED per OpenAPI spec]
    pub async fn retrieve_operator(&self, operator_id: &str) -> Result<Operator> {
        let url = self.build_url(&format!("/v0.1/me/accounts/{}", operator_id))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }

    /// Updates an operator. [DEPRECATED per OpenAPI spec]
    pub async fn update_operator(&self, operator_id: &str, body: &UpdateOperatorRequest) -> Result<Operator> {
        let url = self.build_url(&format!("/v0.1/me/accounts/{}", operator_id))?;
        let response = self.http_client.put(url).bearer_auth(&self.api_key).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Disables an operator. [DEPRECATED per OpenAPI spec]
    pub async fn disable_operator(&self, operator_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/me/accounts/{}", operator_id))?;
        let response = self.http_client.delete(url).bearer_auth(&self.api_key).send().await?;
        self.handle_empty_response(response).await
    }
}
