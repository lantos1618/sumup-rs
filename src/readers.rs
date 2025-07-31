use crate::{
    CreateReaderCheckoutRequest, CreateReaderRequest, Reader, ReaderCheckoutResponse,
    ReaderListResponse, Result, SumUpClient, UpdateReaderRequest,
};

impl SumUpClient {
    /// Lists all readers for the authenticated merchant.
    ///
    /// Note: The /v0.1/me/readers endpoint does not exist in the SumUp API.
    /// Use list_merchant_readers with a merchant_code instead.
    #[deprecated(
        since = "0.2.0",
        note = "The /me/readers endpoint does not exist. Use list_merchant_readers instead."
    )]
    pub async fn list_readers(&self) -> Result<ReaderListResponse> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v0.1/me/readers endpoint does not exist in the SumUp API. Use list_merchant_readers with a merchant_code instead.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
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
    ///
    /// Note: The /v0.1/me/readers endpoint does not exist in the SumUp API.
    /// Use create_merchant_reader with a merchant_code instead.
    #[deprecated(
        since = "0.2.0",
        note = "The /me/readers endpoint does not exist. Use create_merchant_reader instead."
    )]
    pub async fn create_reader(&self, _body: &CreateReaderRequest) -> Result<Reader> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v0.1/me/readers endpoint does not exist in the SumUp API. Use create_merchant_reader with a merchant_code instead.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
    }

    /// Creates a reader for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `body` - The reader details to create
    pub async fn create_merchant_reader(
        &self,
        merchant_code: &str,
        body: &CreateReaderRequest,
    ) -> Result<Reader> {
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
    ///
    /// Note: The /v0.1/me/readers/{reader_id} endpoint does not exist in the SumUp API.
    /// Use retrieve_merchant_reader with a merchant_code instead.
    #[deprecated(
        since = "0.2.0",
        note = "The /me/readers/{reader_id} endpoint does not exist. Use retrieve_merchant_reader instead."
    )]
    pub async fn retrieve_reader(&self, _reader_id: &str) -> Result<Reader> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v0.1/me/readers/{reader_id} endpoint does not exist in the SumUp API. Use retrieve_merchant_reader with a merchant_code instead.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
    }

    /// Retrieves a reader for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    pub async fn retrieve_merchant_reader(
        &self,
        merchant_code: &str,
        reader_id: &str,
    ) -> Result<Reader> {
        let url = self.build_url(&format!(
            "/v0.1/merchants/{}/readers/{}",
            merchant_code, reader_id
        ))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

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
    ///
    /// Note: The /v0.1/me/readers/{reader_id} endpoint does not exist in the SumUp API.
    /// Use update_merchant_reader with a merchant_code instead.
    #[deprecated(
        since = "0.2.0",
        note = "The /me/readers/{reader_id} endpoint does not exist. Use update_merchant_reader instead."
    )]
    pub async fn update_reader(
        &self,
        _reader_id: &str,
        _body: &UpdateReaderRequest,
    ) -> Result<Reader> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v0.1/me/readers/{reader_id} endpoint does not exist in the SumUp API. Use update_merchant_reader with a merchant_code instead.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
    }

    /// Updates a reader for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    /// * `body` - The reader details to update
    pub async fn update_merchant_reader(
        &self,
        merchant_code: &str,
        reader_id: &str,
        body: &UpdateReaderRequest,
    ) -> Result<Reader> {
        let url = self.build_url(&format!(
            "/v0.1/merchants/{}/readers/{}",
            merchant_code, reader_id
        ))?;

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
    ///
    /// Note: The /v0.1/me/readers/{reader_id} endpoint does not exist in the SumUp API.
    /// Use delete_merchant_reader with a merchant_code instead.
    #[deprecated(
        since = "0.2.0",
        note = "The /me/readers/{reader_id} endpoint does not exist. Use delete_merchant_reader instead."
    )]
    pub async fn delete_reader(&self, _reader_id: &str) -> Result<()> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v0.1/me/readers/{reader_id} endpoint does not exist in the SumUp API. Use delete_merchant_reader with a merchant_code instead.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
    }

    /// Deletes a reader for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    pub async fn delete_merchant_reader(&self, merchant_code: &str, reader_id: &str) -> Result<()> {
        let url = self.build_url(&format!(
            "/v0.1/merchants/{}/readers/{}",
            merchant_code, reader_id
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

    /// Creates a checkout for a specific reader (in-person payment).
    ///
    /// # Arguments
    /// * `reader_id` - The unique reader identifier
    /// * `body` - The checkout request details
    ///
    /// Note: The /v0.1/me/readers/{reader_id}/checkout endpoint does not exist in the SumUp API.
    /// Use create_merchant_reader_checkout with a merchant_code instead.
    #[deprecated(
        since = "0.2.0",
        note = "The /me/readers/{reader_id}/checkout endpoint does not exist. Use create_merchant_reader_checkout instead."
    )]
    pub async fn create_reader_checkout(
        &self,
        _reader_id: &str,
        _body: &CreateReaderCheckoutRequest,
    ) -> Result<ReaderCheckoutResponse> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v0.1/me/readers/{reader_id}/checkout endpoint does not exist in the SumUp API. Use create_merchant_reader_checkout with a merchant_code instead.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
    }

    /// Creates a checkout for a specific reader and merchant (in-person payment).
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    /// * `body` - The checkout request details
    pub async fn create_merchant_reader_checkout(
        &self,
        merchant_code: &str,
        reader_id: &str,
        body: &CreateReaderCheckoutRequest,
    ) -> Result<ReaderCheckoutResponse> {
        let url = self.build_url(&format!(
            "/v0.1/merchants/{}/readers/{}/checkout",
            merchant_code, reader_id
        ))?;

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
    ///
    /// Note: The /v0.1/me/readers/{reader_id}/checkout/{checkout_id} endpoint does not exist in the SumUp API.
    /// Use terminate_merchant_reader_checkout with a merchant_code instead.
    #[deprecated(
        since = "0.2.0",
        note = "The /me/readers/{reader_id}/checkout/{checkout_id} endpoint does not exist. Use terminate_merchant_reader_checkout instead."
    )]
    pub async fn terminate_reader_checkout(
        &self,
        _reader_id: &str,
        _checkout_id: &str,
    ) -> Result<()> {
        Err(crate::Error::ApiError {
            status: 404,
            body: crate::ApiErrorBody {
                error_type: None,
                title: Some("Endpoint not implemented".to_string()),
                status: Some(404),
                detail: Some("The /v0.1/me/readers/{reader_id}/checkout/{checkout_id} endpoint does not exist in the SumUp API. Use terminate_merchant_reader_checkout with a merchant_code instead.".to_string()),
                error_code: None,
                message: None,
                param: None,
                additional_fields: std::collections::HashMap::new(),
            }
        })
    }

    /// Terminates a reader checkout for a specific merchant.
    ///
    /// # Arguments
    /// * `merchant_code` - The unique merchant code identifier
    /// * `reader_id` - The unique reader identifier
    /// * `checkout_id` - The unique checkout identifier
    pub async fn terminate_merchant_reader_checkout(
        &self,
        merchant_code: &str,
        reader_id: &str,
        checkout_id: &str,
    ) -> Result<()> {
        let url = self.build_url(&format!(
            "/v0.1/merchants/{}/readers/{}/checkout/{}",
            merchant_code, reader_id, checkout_id
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
