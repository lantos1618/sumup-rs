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
}

impl SumUpClient {
    /// Lists transaction history for a merchant.
    pub async fn list_transactions_history(&self, merchant_code: &str, query: &TransactionHistoryQuery<'_>) -> Result<TransactionHistoryResponse> {
        let url = self.build_url(&format!("/v2.1/merchants/{}/transactions/history", merchant_code))?;
        let response = self.http_client.get(url).bearer_auth(&self.api_key).query(query).send().await?;
        self.handle_response(response).await
    }

    /// Retrieves a transaction by ID.
    pub async fn retrieve_transaction_by_id(&self, merchant_code: &str, transaction_id: &str) -> Result<Transaction> {
        let mut url = self.build_url(&format!("/v2.1/merchants/{}/transactions", merchant_code))?;
        url.query_pairs_mut().append_pair("id", transaction_id);
        let response = self.http_client.get(url).bearer_auth(&self.api_key).send().await?;
        self.handle_response(response).await
    }

    /// Refunds a transaction.
    pub async fn refund_transaction(&self, merchant_code: &str, transaction_id: &str, amount: Option<f64>, reason: &str) -> Result<Transaction> {
        let url = self.build_url(&format!("/v0.1/merchants/{}/transactions/{}/refunds", merchant_code, transaction_id))?;

        let mut body = serde_json::Map::new();
        body.insert("reason".to_string(), serde_json::Value::String(reason.to_string()));
        if let Some(amt) = amount {
            body.insert("amount".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(amt).unwrap()));
        }

        let response = self.http_client.post(url).bearer_auth(&self.api_key).json(&body).send().await?;
        self.handle_response(response).await
    }

    /// Extracts the next page URL from a transaction history response.
    pub fn get_next_page_url_from_history(history: &TransactionHistoryResponse) -> Option<String> {
        history.links.iter().find(|link| link.rel == "next").map(|link| link.href.clone())
    }

    /// Extracts the previous page URL from a transaction history response.
    pub fn get_previous_page_url_from_history(history: &TransactionHistoryResponse) -> Option<String> {
        history.links.iter().find(|link| link.rel == "prev").map(|link| link.href.clone())
    }

    /// Checks if there are more pages available.
    pub fn has_next_page_from_history(history: &TransactionHistoryResponse) -> bool {
        history.links.iter().any(|link| link.rel == "next")
    }

    /// Fetches all transactions by automatically handling pagination.
    pub async fn list_all_transactions_history(&self, merchant_code: &str, order: Option<&str>, max_pages: Option<usize>) -> Result<Vec<Transaction>> {
        let mut all_transactions = Vec::new();
        let mut page_count = 0;
        let mut newest_time: Option<String> = None;

        loop {
            if let Some(max) = max_pages {
                if page_count >= max { break; }
            }

            let history = self.list_transactions_history(merchant_code, &TransactionHistoryQuery {
                limit: Some(100),
                order,
                newest_time: newest_time.as_deref(),
                oldest_time: None,
            }).await?;

            let has_next = Self::has_next_page_from_history(&history);
            let last_ts = history.items.last().map(|t| t.timestamp.clone());

            all_transactions.extend(history.items);

            match last_ts {
                Some(ts) => newest_time = Some(ts),
                None => break,
            }

            if !has_next { break; }
            page_count += 1;
        }

        Ok(all_transactions)
    }
}
