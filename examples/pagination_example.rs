#![allow(clippy::type_complexity)]
use sumup_rs::{CheckoutListQuery, SumUpClient, TransactionHistoryQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("Please set SUMUP_API_SECRET_KEY environment variable");

    // Create a client (use sandbox for testing)
    let client = SumUpClient::new(api_key, true)?;

    println!("=== SumUp Pagination and Query Examples ===\n");

    // Example 1: Manual pagination through transaction history
    println!("1. Manual pagination through transaction history...");
    let merchant_code = "your-merchant-code"; // Replace with actual merchant code

    let mut page_count = 0;
    let mut newest_time: Option<String> = None;

    loop {
        println!("  Fetching page {}...", page_count + 1);

        let query = TransactionHistoryQuery {
            limit: Some(10), // Small page size for demonstration
            order: Some("desc"),
            newest_time: newest_time.as_deref(),
            oldest_time: None,
        };
        let history = client
            .list_transactions_history(merchant_code, &query)
            .await?;

        println!(
            "    Found {} transactions on this page",
            history.items.len()
        );

        // Process transactions on this page
        for transaction in &history.items {
            println!(
                "    - {}: {} {} ({})",
                transaction.id,
                transaction.amount,
                transaction.currency,
                transaction.status.as_deref().unwrap_or("Unknown")
            );
        }

        // Check if there are more pages
        if !SumUpClient::has_next_page_from_history(&history) {
            println!("    No more pages available");
            break;
        }

        // Get the newest time from the last transaction for the next page
        if let Some(last_transaction) = history.items.last() {
            newest_time = Some(last_transaction.timestamp.clone());
        } else {
            break;
        }

        page_count += 1;

        // Limit to 3 pages for demonstration
        if page_count >= 3 {
            println!("    Reached page limit for demonstration");
            break;
        }
    }

    // Example 2: Automatic pagination with convenience method
    println!("\n2. Automatic pagination with convenience method...");
    let all_transactions = client
        .list_all_transactions_history(
            merchant_code,
            Some("desc"),
            Some(5), // Limit to 5 pages
        )
        .await?;

    println!(
        "  Fetched {} transactions across all pages",
        all_transactions.len()
    );

    // Example 3: Advanced checkout queries
    println!("\n3. Advanced checkout queries...");

    // Query for paid checkouts with specific reference
    let query = CheckoutListQuery {
        checkout_reference: Some("order-123".to_string()),
        status: Some("PAID".to_string()),
        merchant_code: Some(merchant_code.to_string()),
        customer_id: None,
        limit: Some(5),
        offset: Some(0),
    };

    match client.list_checkouts_with_query(&query).await {
        Ok(checkouts) => {
            println!(
                "  Found {} paid checkouts with reference 'order-123'",
                checkouts.len()
            );
            for checkout in &checkouts {
                println!(
                    "    - {}: {} {} ({})",
                    checkout.id, checkout.amount, checkout.currency, checkout.status
                );
            }
        }
        Err(e) => {
            println!("  Error querying checkouts: {}", e);
        }
    }

    // Example 4: Pagination helper functions
    println!("\n4. Using pagination helper functions...");
    let query = TransactionHistoryQuery {
        limit: Some(5),
        order: Some("desc"),
        newest_time: None,
        oldest_time: None,
    };
    let history = client
        .list_transactions_history(merchant_code, &query)
        .await?;

    // Check for next page
    if let Some(next_url) = SumUpClient::get_next_page_url_from_history(&history) {
        println!("  Next page available at: {}", next_url);
    } else {
        println!("  No next page available");
    }

    // Check for previous page
    if let Some(prev_url) = SumUpClient::get_previous_page_url_from_history(&history) {
        println!("  Previous page available at: {}", prev_url);
    } else {
        println!("  No previous page available");
    }

    // Check if there are more pages
    if SumUpClient::has_next_page_from_history(&history) {
        println!("  More pages are available");
    } else {
        println!("  This is the last page");
    }

    println!("\n=== Pagination Examples Complete ===");
    println!("Key features demonstrated:");
    println!("  - Manual pagination with cursor-based navigation");
    println!("  - Automatic pagination with convenience methods");
    println!("  - Advanced query filtering for checkouts");
    println!("  - Pagination helper functions for navigation");

    Ok(())
}
