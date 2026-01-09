use crate::{
    CreateReaderCheckoutRequest, CreateReaderRequest, Reader, ReaderCheckoutResponse,
    ReaderListResponse, Result, SumUpClient, UpdateReaderRequest,
};

impl SumUpClient {
    /// Lists readers for a merchant.
    pub async fn list_merchant_readers(&self, merchant_code: &str) -> Result<ReaderListResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers", merchant_code))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }

    /// Creates a reader for a merchant.
    pub async fn create_merchant_reader(&self, merchant_code: &str, body: &CreateReaderRequest) -> Result<Reader> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers", merchant_code))?;
        let response = self.http_client.post(url).bearer_auth(&self.api_key).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves a reader for a merchant.
    pub async fn retrieve_merchant_reader(&self, merchant_code: &str, reader_id: &str) -> Result<Reader> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}", merchant_code, reader_id))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }

    /// Updates a reader for a merchant.
    pub async fn update_merchant_reader(&self, merchant_code: &str, reader_id: &str, body: &UpdateReaderRequest) -> Result<Reader> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}", merchant_code, reader_id))?;
        let response = self.http_client.put(url).bearer_auth(&self.api_key).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Deletes a reader for a merchant.
    pub async fn delete_merchant_reader(&self, merchant_code: &str, reader_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}", merchant_code, reader_id))?;
        let response = self.http_client.delete(url).bearer_auth(&self.api_key).send().await?;
        self.handle_empty_response(response).await
    }

    /// Creates a checkout for a reader (in-person payment).
    pub async fn create_merchant_reader_checkout(&self, merchant_code: &str, reader_id: &str, body: &CreateReaderCheckoutRequest) -> Result<ReaderCheckoutResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}/checkout", merchant_code, reader_id))?;
        let response = self.http_client.post(url).bearer_auth(&self.api_key).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Terminates a reader checkout.
    pub async fn terminate_merchant_reader_checkout(&self, merchant_code: &str, reader_id: &str, checkout_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}/checkout/{}", merchant_code, reader_id, checkout_id))?;
        let response = self.http_client.delete(url).bearer_auth(&self.api_key).send().await?;
        self.handle_empty_response(response).await
    }
}
