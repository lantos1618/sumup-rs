use sumup_rs::{SumUpClient, TransactionHistoryQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")?;
    let client = SumUpClient::new(api_key, true)?;

    let merchant_code = std::env::var("SUMUP_MERCHANT_CODE")?;

    // Manual pagination
    println!("Manual pagination:");
    let mut newest_time: Option<String> = None;

    for page in 0..3 {
        let query = TransactionHistoryQuery {
            limit: Some(10),
            order: Some("desc".to_string()),
            newest_time: newest_time.clone(),
            oldest_time: None,
            status: None,
            payment_type: None,
        };

        let history = client.list_transactions_history(&merchant_code, &query).await?;
        println!("Page {}: {} transactions", page + 1, history.items.len());

        if !SumUpClient::has_next_page_from_history(&history) {
            break;
        }

        newest_time = history.items.last().map(|t| t.timestamp.to_rfc3339());
    }

    // Auto pagination (up to 3 pages)
    println!("\nAuto pagination:");
    let all = client
        .list_all_transactions_history(&merchant_code, Some("desc"), Some(3))
        .await?;
    println!("Total: {} transactions", all.len());

    Ok(())
}
