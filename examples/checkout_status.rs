#![allow(clippy::type_complexity)]
use sumup_rs::SumUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();

    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");

    // Get checkout ID from command line args
    let checkout_id = std::env::args()
        .nth(1)
        .expect("Please provide checkout ID as argument");

    println!("Checking status for checkout: {}", checkout_id);

    // Create a client
    let client = SumUpClient::new(api_key, true)?;

    // Retrieve the checkout
    match client.retrieve_checkout(&checkout_id).await {
        Ok(checkout) => {
            println!("✅ Checkout Status: {}", checkout.status);
            println!("   ID: {}", checkout.id);
            println!("   Amount: {} {}", checkout.amount, checkout.currency);
            println!("   Reference: {:?}", checkout.checkout_reference);

            if !checkout.transactions.is_empty() {
                println!("\n📊 Transactions:");
                for transaction in &checkout.transactions {
                    println!("   - ID: {}", transaction.id);
                    println!(
                        "     Status: {}",
                        transaction.status.as_ref().map(|s| s.to_string()).unwrap_or_else(|| "Unknown".to_string())
                    );
                    println!(
                        "     Amount: {} {}",
                        transaction.amount, transaction.currency
                    );
                    println!("     Timestamp: {}", transaction.timestamp);
                }
            } else {
                println!("\n📊 No transactions found");
            }
        }
        Err(e) => {
            println!("❌ Failed to retrieve checkout: {}", e);
        }
    }

    Ok(())
}
