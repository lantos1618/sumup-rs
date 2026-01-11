use crate::{CreateMemberRequest, Member, MemberListResponse, Result, SumUpClient, UpdateMemberRequest};

impl SumUpClient {
    /// Lists all members for a merchant.
    pub async fn list_members(&self, merchant_code: impl AsRef<str>) -> Result<MemberListResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/members", merchant_code.as_ref()))?;
        let response = self.http_client.get(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_response(response).await
    }

    /// Creates a new member for a merchant.
    pub async fn create_member(&self, merchant_code: impl AsRef<str>, body: &CreateMemberRequest) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/members", merchant_code.as_ref()))?;
        let response = self.http_client.post(url).bearer_auth(self.api_key_str()).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves a member by ID.
    pub async fn retrieve_member(&self, merchant_code: impl AsRef<str>, member_id: impl AsRef<str>) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/members/{}", merchant_code.as_ref(), member_id.as_ref()))?;
        let response = self.http_client.get(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_response(response).await
    }

    /// Updates a member.
    pub async fn update_member(&self, merchant_code: impl AsRef<str>, member_id: impl AsRef<str>, body: &UpdateMemberRequest) -> Result<Member> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/members/{}", merchant_code.as_ref(), member_id.as_ref()))?;
        let response = self.http_client.put(url).bearer_auth(self.api_key_str()).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Deletes a member.
    pub async fn delete_member(&self, merchant_code: impl AsRef<str>, member_id: impl AsRef<str>) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/members/{}", merchant_code.as_ref(), member_id.as_ref()))?;
        let response = self.http_client.delete(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_empty_response(response).await
    }
}
