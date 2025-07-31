use crate::{
    CreateCustomerRequest, Customer, PaymentInstrument, Result, SumUpClient, UpdateCustomerRequest,
};

impl SumUpClient {
    /// Creates a new saved customer resource.
    ///
    /// # Arguments
    /// * `body` - The request body containing the customer_id and personal details.
    pub async fn create_customer(&self, body: &CreateCustomerRequest) -> Result<Customer> {
        let url = self.build_url("/v0.1/customers")?;

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let customer = response.json::<Customer>().await?;
            Ok(customer)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves an identified saved customer resource.
    ///
    /// # Arguments
    /// * `customer_id` - The unique ID of the customer.
    pub async fn retrieve_customer(&self, customer_id: &str) -> Result<Customer> {
        let url = self.build_url(&format!("/v0.1/customers/{}", customer_id))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let customer = response.json::<Customer>().await?;
            Ok(customer)
        } else {
            self.handle_error(response).await
        }
    }

    /// Updates an identified saved customer resource's personal details.
    ///
    /// # Arguments
    /// * `customer_id` - The unique ID of the customer.
    /// * `body` - The customer details to update.
    pub async fn update_customer(
        &self,
        customer_id: &str,
        body: &UpdateCustomerRequest,
    ) -> Result<Customer> {
        let url = self.build_url(&format!("/v0.1/customers/{}", customer_id))?;

        let response = self
            .http_client
            .put(url)
            .bearer_auth(&self.api_key)
            .json(body)
            .send()
            .await?;

        if response.status().is_success() {
            let customer = response.json::<Customer>().await?;
            Ok(customer)
        } else {
            self.handle_error(response).await
        }
    }

    /// Lists all payment instrument resources that are saved for an identified customer.
    ///
    /// # Arguments
    /// * `customer_id` - The unique ID of the customer.
    pub async fn list_customer_payment_instruments(
        &self,
        customer_id: &str,
    ) -> Result<Vec<PaymentInstrument>> {
        let url = self.build_url(&format!(
            "/v0.1/customers/{}/payment-instruments",
            customer_id
        ))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            let instruments = response.json::<Vec<PaymentInstrument>>().await?;
            Ok(instruments)
        } else {
            self.handle_error(response).await
        }
    }

    /// Deactivates an identified card payment instrument resource for a customer.
    /// A successful deactivation returns a 204 No Content response.
    ///
    /// # Arguments
    /// * `customer_id` - The unique ID of the customer.
    /// * `token` - The token of the payment instrument to deactivate.
    pub async fn deactivate_customer_payment_instrument(
        &self,
        customer_id: &str,
        token: &str,
    ) -> Result<()> {
        let url = self.build_url(&format!(
            "/v0.1/customers/{}/payment-instruments/{}",
            customer_id, token
        ))?;

        let response = self
            .http_client
            .delete(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(()) // Successful deletion often returns 204 No Content
        } else {
            self.handle_error(response).await
        }
    }
}
