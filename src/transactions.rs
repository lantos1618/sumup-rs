use crate::{Result, SumUpClient, Transaction, TransactionHistoryResponse};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Default)]
pub struct TransactionHistoryQuery<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newest_time: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oldest_time: Option<&'a str>,
    // Add other query parameters as needed
}

impl SumUpClient {
    /// Lists detailed history of all transactions associated with the merchant profile.
    /// Uses the modern v2.1 endpoint.
    ///
    /// # Arguments
    /// * `merchant_code` - The merchant's unique code.
    /// * `query` - A struct with query parameters for filtering and pagination.
    pub async fn list_transactions_history(
        &self,
        merchant_code: &str,
        query: &TransactionHistoryQuery<'_>,
    ) -> Result<TransactionHistoryResponse> {
        let url = self.build_url(&format!(
            "/v2.1/merchants/{}/transactions/history",
            merchant_code
        ))?;

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .query(query)
            .send()
            .await?;

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

        let response = self
            .http_client
            .get(url)
            .bearer_auth(&self.api_key)
            .send()
            .await?;

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
        let url = self.build_url(&format!(
            "/v0.1/merchants/{}/transactions/{}/refunds",
            merchant_code, transaction_id
        ))?;

        let mut body = serde_json::Map::new();
        body.insert(
            "reason".to_string(),
            serde_json::Value::String(reason.to_string()),
        );
        if let Some(amt) = amount {
            body.insert(
                "amount".to_string(),
                serde_json::Value::Number(serde_json::Number::from_f64(amt).unwrap()),
            );
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

// --- Pagination Helpers ---

impl SumUpClient {
    /// Extracts the next page URL from a transaction history response.
    /// Returns `None` if there are no more pages.
    ///
    /// # Arguments
    /// * `history` - The transaction history response to extract from
    ///
    /// # Returns
    /// * `Some(url)` - The URL for the next page, if available
    /// * `None` - If there are no more pages
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sumup_rs::{SumUpClient, TransactionHistoryQuery};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SumUpClient::new("your-api-key".to_string(), true)?;
    /// let query = TransactionHistoryQuery {
    ///     limit: Some(10),
    ///     order: Some("desc"),
    ///     newest_time: None,
    ///     oldest_time: None,
    /// };
    /// let history = client.list_transactions_history("merchant123", &query).await?;
    ///
    /// if let Some(next_url) = SumUpClient::get_next_page_url_from_history(&history) {
    ///     println!("Next page available at: {}", next_url);
    /// } else {
    ///     println!("No more pages available");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_next_page_url_from_history(history: &TransactionHistoryResponse) -> Option<String> {
        history
            .links
            .iter()
            .find(|link| link.rel == "next")
            .map(|link| link.href.clone())
    }

    /// Extracts the previous page URL from a transaction history response.
    /// Returns `None` if there is no previous page.
    ///
    /// # Arguments
    /// * `history` - The transaction history response to extract from
    ///
    /// # Returns
    /// * `Some(url)` - The URL for the previous page, if available
    /// * `None` - If there is no previous page
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sumup_rs::{SumUpClient, TransactionHistoryQuery};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SumUpClient::new("your-api-key".to_string(), true)?;
    /// let query = TransactionHistoryQuery {
    ///     limit: Some(10),
    ///     order: Some("desc"),
    ///     newest_time: None,
    ///     oldest_time: None,
    /// };
    /// let history = client.list_transactions_history("merchant123", &query).await?;
    ///
    /// if let Some(prev_url) = SumUpClient::get_previous_page_url_from_history(&history) {
    ///     println!("Previous page available at: {}", prev_url);
    /// } else {
    ///     println!("No previous page available");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_previous_page_url_from_history(
        history: &TransactionHistoryResponse,
    ) -> Option<String> {
        history
            .links
            .iter()
            .find(|link| link.rel == "prev")
            .map(|link| link.href.clone())
    }

    /// Checks if there are more pages available in a transaction history response.
    ///
    /// # Arguments
    /// * `history` - The transaction history response to check
    ///
    /// # Returns
    /// * `true` - If there are more pages available
    /// * `false` - If this is the last page
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sumup_rs::{SumUpClient, TransactionHistoryQuery};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SumUpClient::new("your-api-key".to_string(), true)?;
    /// let query = TransactionHistoryQuery {
    ///     limit: Some(10),
    ///     order: Some("desc"),
    ///     newest_time: None,
    ///     oldest_time: None,
    /// };
    /// let history = client.list_transactions_history("merchant123", &query).await?;
    ///
    /// if SumUpClient::has_next_page_from_history(&history) {
    ///     println!("More pages available");
    /// } else {
    ///     println!("This is the last page");
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn has_next_page_from_history(history: &TransactionHistoryResponse) -> bool {
        history.links.iter().any(|link| link.rel == "next")
    }

    /// Fetches all transactions for a merchant by automatically handling pagination.
    /// This is a convenience method that fetches all pages and combines the results.
    ///
    /// # Arguments
    /// * `merchant_code` - The merchant's unique code
    /// * `order` - Sort order (e.g., "asc", "desc")
    /// * `max_pages` - Maximum number of pages to fetch (None for unlimited)
    ///
    /// # Returns
    /// * `Vec<Transaction>` - All transactions from all pages
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use sumup_rs::SumUpClient;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = SumUpClient::new("your-api-key".to_string(), true)?;
    ///
    /// // Fetch all transactions (up to 5 pages to avoid overwhelming the API)
    /// let all_transactions = client.list_all_transactions_history("merchant123", Some("desc"), Some(5)).await?;
    /// println!("Fetched {} transactions", all_transactions.len());
    ///
    /// // Fetch all transactions without page limit (use with caution)
    /// let all_transactions = client.list_all_transactions_history("merchant123", Some("desc"), None).await?;
    /// println!("Fetched {} transactions", all_transactions.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_all_transactions_history(
        &self,
        merchant_code: &str,
        order: Option<&str>,
        max_pages: Option<usize>,
    ) -> Result<Vec<Transaction>> {
        let mut all_transactions = Vec::new();
        let mut page_count = 0;
        let mut newest_time: Option<String> = None;

        loop {
            // Check if we've reached the maximum number of pages
            if let Some(max) = max_pages {
                if page_count >= max {
                    break;
                }
            }

            // Fetch the current page
            let history = self
                .list_transactions_history(
                    merchant_code,
                    &TransactionHistoryQuery {
                        limit: Some(100), // Use a reasonable page size
                        order,
                        newest_time: newest_time.as_deref(),
                        oldest_time: None, // No oldest_time for this convenience method
                    },
                )
                .await?;

            // Check if there are more pages before moving data
            let has_next_page = Self::has_next_page_from_history(&history);

            // Get the newest time from the last transaction for the next page
            let last_transaction = history.items.last().map(|t| t.timestamp.clone());

            // Add transactions from this page
            all_transactions.extend(history.items);

            // Update newest_time for next iteration
            if let Some(timestamp) = last_transaction {
                newest_time = Some(timestamp);
            } else {
                break;
            }

            // Check if we should continue
            if !has_next_page {
                break;
            }

            page_count += 1;
        }

        Ok(all_transactions)
    }
}
