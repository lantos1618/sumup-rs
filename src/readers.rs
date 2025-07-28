use crate::{SumUpClient, Result, Reader, ReaderListResponse, CreateReaderRequest, UpdateReaderRequest, CreateReaderCheckoutRequest, ReaderCheckoutResponse};

impl SumUpClient {
    /// Lists all readers for the authenticated merchant.
    pub async fn list_readers(&self) -> Result<ReaderListResponse> {
        let url = self.build_url("/v0.1/me/readers")?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let readers = response.json::<ReaderListResponse>().await?;
            Ok(readers)
        } else {
            self.handle_error(response).await
        }
    }

    /// Lists readers for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    pub async fn list_merchant_readers(&self, merchant_code: &str) -> Result<ReaderListResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers", merchant_code))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let readers = response.json::<ReaderListResponse>().await?;
            Ok(readers)
        } else {
            self.handle_error(response).await
        }
    }

    /// Creates a new reader resource.
    ///
    /// # Arguments
    /// * `body` - The reader details to create
    pub async fn create_reader(&self, body: &CreateReaderRequest) -> Result<Reader> {
        let url = self.build_url("/v0.1/me/readers")?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let reader = response.json::<Reader>().await?;
            Ok(reader)
        } else {
            self.handle_error(response).await
        }
    }

    /// Creates a reader for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `body` - The reader details to create
    pub async fn create_merchant_reader(&self, merchant_code: &str, body: &CreateReaderRequest) -> Result<Reader> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers", merchant_code))?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let reader = response.json::<Reader>().await?;
            Ok(reader)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves an identified reader resource.
    ///
    /// # Arguments
    /// * `reader_id` - The unique reader identifier
    pub async fn retrieve_reader(&self, reader_id: &str) -> Result<Reader> {
        let url = self.build_url(&format!("/v0.1/me/readers/{}", reader_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let reader = response.json::<Reader>().await?;
            Ok(reader)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves a reader for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    pub async fn retrieve_merchant_reader(&self, merchant_code: &str, reader_id: &str) -> Result<Reader> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}", merchant_code, reader_id))?;

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let reader = response.json::<Reader>().await?;
            Ok(reader)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates an identified reader resource.
    ///
    /// # Arguments
    /// * `reader_id` - The unique reader identifier
    /// * `body` - The reader details to update
    pub async fn update_reader(&self, reader_id: &str, body: &UpdateReaderRequest) -> Result<Reader> {
        let url = self.build_url(&format!("/v0.1/me/readers/{}", reader_id))?;

        let response = self
            .http_client
            .put(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let reader = response.json::<Reader>().await?;
            Ok(reader)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates a reader for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    /// * `body` - The reader details to update
    pub async fn update_merchant_reader(&self, merchant_code: &str, reader_id: &str, body: &UpdateReaderRequest) -> Result<Reader> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}", merchant_code, reader_id))?;

        let response = self
            .http_client
            .put(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let reader = response.json::<Reader>().await?;
            Ok(reader)
        } else {
            self.handle_error(response).await
        }
    }

    /// Deletes an identified reader resource.
    ///
    /// # Arguments
    /// * `reader_id` - The unique reader identifier
    pub async fn delete_reader(&self, reader_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/me/readers/{}", reader_id))?;

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

    /// Deletes a reader for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    pub async fn delete_merchant_reader(&self, merchant_code: &str, reader_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}", merchant_code, reader_id))?;

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

    /// Creates a checkout for a specific reader (in-person payment).
    ///
    /// # Arguments
    /// * `reader_id` - The unique reader identifier
    /// * `body` - The checkout request details
    pub async fn create_reader_checkout(&self, reader_id: &str, body: &CreateReaderCheckoutRequest) -> Result<ReaderCheckoutResponse> {
        let url = self.build_url(&format!("/v0.1/me/readers/{}/checkout", reader_id))?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let checkout = response.json::<ReaderCheckoutResponse>().await?;
            Ok(checkout)
        } else {
            self.handle_error(response).await
        }
    }

    /// Creates a checkout for a specific reader and merchant (in-person payment).
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    /// * `body` - The checkout request details
    pub async fn create_merchant_reader_checkout(&self, merchant_code: &str, reader_id: &str, body: &CreateReaderCheckoutRequest) -> Result<ReaderCheckoutResponse> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}/checkout", merchant_code, reader_id))?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let checkout = response.json::<ReaderCheckoutResponse>().await?;
            Ok(checkout)
        } else {
            self.handle_error(response).await
        }
    }

    /// Terminates a reader checkout.
    ///
    /// # Arguments
    /// * `reader_id` - The unique reader identifier
    /// * `checkout_id` - The unique checkout identifier
    pub async fn terminate_reader_checkout(&self, reader_id: &str, checkout_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/me/readers/{}/checkout/{}", reader_id, checkout_id))?;

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

    /// Terminates a reader checkout for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    /// * `checkout_id` - The unique checkout identifier
    pub async fn terminate_merchant_reader_checkout(&self, merchant_code: &str, reader_id: &str, checkout_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/readers/{}/checkout/{}", merchant_code, reader_id, checkout_id))?;

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