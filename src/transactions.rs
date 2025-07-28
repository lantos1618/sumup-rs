use crate::{SumUpClient, Result, Transaction, TransactionHistoryResponse};

impl SumUpClient {
    /// Lists detailed history of all transactions associated with the merchant profile.
    /// Uses the modern v2.1 endpoint.
    ///
    /// # Arguments
    /// * `merchant_code` - The merchant's unique code.
    /// * `limit` - The maximum number of transactions to return.
    /// * `order` - Sort order (e.g., "asc", "desc").
    /// * `newest_time` - The timestamp of the newest transaction to start from.
    pub async fn list_transactions_history(
        &self,
        merchant_code: &str,
        limit: Option<i32>,
        order: Option<&str>,
        newest_time: Option<&str>,
    ) -> Result<TransactionHistoryResponse> {
        let mut url = self.build_url(&format!("/v2.1/merchants/{}/transactions/history", merchant_code))?;

        { // Scoped to release the mutable borrow on url
            let mut query_pairs = url.query_pairs_mut();
            if let Some(l) = limit {
                query_pairs.append_pair("limit", &l.to_string());
            }
            if let Some(o) = order {
                query_pairs.append_pair("order", o);
            }
            if let Some(nt) = newest_time {
                query_pairs.append_pair("newest_time", nt);
            }
            // Add other query parameters here as needed...
        }

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let history = response.json::<TransactionHistoryResponse>().await?;
            Ok(history)
        } else {
            self.handle_error(response).await
        }
    }

    /// Retrieves the full details of an identified transaction.
    /// Uses the modern v2.1 endpoint.
    ///
    /// # Arguments
    /// * `merchant_code` - The merchant's unique code.
    /// * `transaction_id` - The transaction's unique ID.
    pub async fn retrieve_transaction_by_id(
        &self,
        merchant_code: &str,
        transaction_id: &str,
    ) -> Result<Transaction> {
        let mut url = self.build_url(&format!("/v2.1/merchants/{}/transactions", merchant_code))?;
        url.query_pairs_mut().append_pair("id", transaction_id);

        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;

        if response.status().is_success() {
            let transaction = response.json::<Transaction>().await?;
            Ok(transaction)
        } else {
            self.handle_error(response).await
        }
    }

    /// Refunds a transaction.
    ///
    /// # Arguments
    /// * `merchant_code` - The merchant's unique code.
    /// * `transaction_id` - The transaction's unique ID.
    /// * `amount` - The amount to refund (optional, defaults to full amount).
    /// * `reason` - The reason for the refund.
    pub async fn refund_transaction(
        &self,
        merchant_code: &str,
        transaction_id: &str,
        amount: Option<f64>,
        reason: &str,
    ) -> Result<Transaction> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/transactions/{}/refunds", merchant_code, transaction_id))?;

        let mut body = serde_json::Map::new();
        body.insert("reason".to_string(), serde_json::Value::String(reason.to_string()));
        if let Some(amt) = amount {
            body.insert("amount".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(amt).unwrap()));
        }

        let response = self
            .http_client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            let transaction = response.json::<Transaction>().await?;
            Ok(transaction)
        } else {
            self.handle_error(response).await
        }
    }
} 