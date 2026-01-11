use crate::{
    CreateCustomerRequest, Customer, PaymentInstrument, Result, SumUpClient, UpdateCustomerRequest,
};

impl SumUpClient {
    /// Creates a new saved customer resource.
    pub async fn create_customer(&self, body: &CreateCustomerRequest) -> Result<Customer> {
        let url = self.build_url("/v0.1/customers")?;
        let response = self.http_client.post(url).bearer_auth(self.api_key_str()).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves an identified saved customer resource.
    pub async fn retrieve_customer(&self, customer_id: impl AsRef<str>) -> Result<Customer> {
        let url = self.build_url(&format!("/v0.1/customers/{}", customer_id.as_ref()))?;
        let response = self.http_client.get(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_response(response).await
    }

    /// Updates an identified saved customer resource's personal details.
    pub async fn update_customer(&self, customer_id: impl AsRef<str>, body: &UpdateCustomerRequest) -> Result<Customer> {
        let url = self.build_url(&format!("/v0.1/customers/{}", customer_id.as_ref()))?;
        let response = self.http_client.put(url).bearer_auth(self.api_key_str()).json(body).send().await?;
        self.handle_response(response).await
    }

    /// Lists all payment instruments for a customer.
    pub async fn list_customer_payment_instruments(&self, customer_id: impl AsRef<str>) -> Result<Vec<PaymentInstrument>> {
        let url = self.build_url(&format!("/v0.1/customers/{}/payment-instruments", customer_id.as_ref()))?;
        let response = self.http_client.get(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_response(response).await
    }

    /// Deactivates a payment instrument for a customer.
    pub async fn deactivate_customer_payment_instrument(&self, customer_id: impl AsRef<str>, token: impl AsRef<str>) -> Result<()> {
        let url = self.build_url(&format!("/v0.1/customers/{}/payment-instruments/{}", customer_id.as_ref(), token.as_ref()))?;
        let response = self.http_client.delete(url).bearer_auth(self.api_key_str()).send().await?;
        self.handle_empty_response(response).await
    }
}
