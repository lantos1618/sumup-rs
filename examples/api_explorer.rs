#![allow(clippy::type_complexity)]
#![allow(deprecated)]
use sumup_rs::SumUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("Please set SUMUP_API_SECRET_KEY environment variable");

    // Create a client (use sandbox for testing)
    let client = SumUpClient::new(api_key, true)?;

    println!("=== SumUp API Explorer ===\n");

    // Test 1: Get merchant profile
    println!("1. Testing merchant profile...");
    match client.get_merchant_profile().await {
        Ok(profile) => {
            println!("✅ Merchant profile retrieved successfully!");
            println!("   Merchant Code: {}", profile.merchant_code);
            println!("   Name: {}", profile.name);
            println!(
                "   Email: {}",
                profile
                    .doing_business_as
                    .as_ref()
                    .map(|dba| &dba.email)
                    .unwrap_or(&"No email".to_string())
            );
            println!("   Country: {}", profile.country);
            println!("   Currency: {}", profile.currency);
        }
        Err(e) => {
            println!("❌ Failed to get merchant profile: {}", e);
        }
    }

    // Test 2: List merchants
    println!("\n2. Testing list merchants...");
    match client.list_merchants().await {
        Ok(merchants) => {
            println!("✅ Found {} merchants:", merchants.len());
            for merchant in &merchants {
                println!("   - {}", merchant.id);
            }
        }
        Err(e) => {
            println!("❌ Failed to list merchants: {}", e);
        }
    }

    // Test 3: Retrieve a specific customer (using the one we know exists)
    println!("\n3. Testing retrieve customer...");
    match client.retrieve_customer("cust_12345").await {
        Ok(customer) => {
            println!("✅ Customer retrieved successfully!");
            println!("   Customer ID: {}", customer.customer_id);
            if let Some(pd) = &customer.personal_details {
                println!(
                    "   Name: {} {}",
                    pd.first_name.as_deref().unwrap_or(""),
                    pd.last_name.as_deref().unwrap_or("")
                );
                println!("   Email: {}", pd.email.as_deref().unwrap_or("No email"));
            }
        }
        Err(e) => {
            println!("❌ Failed to retrieve customer: {}", e);
        }
    }

    // Test 4: List payouts (with date range)
    println!("\n4. Testing list payouts...");
    let payout_query = sumup_rs::payouts::PayoutListQuery {
        start_date: "2023-01-01".to_string(),
        end_date: "2024-12-31".to_string(),
        limit: Some(5),
        offset: Some(0),
    };

    match client.list_payouts(&payout_query).await {
        Ok(payouts) => {
            println!("✅ Found {} payouts:", payouts.payouts.len());
            for payout in &payouts.payouts {
                println!(
                    "   - {}: {} {} ({})",
                    payout.id, payout.amount, payout.currency, payout.status
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to list payouts: {}", e);
        }
    }

    // Test 5: List readers
    println!("\n5. Testing list readers...");
    match client.list_readers().await {
        Ok(readers) => {
            println!("✅ Found {} readers:", readers.readers.len());
            for reader in &readers.readers {
                println!(
                    "   - {} ({}) - Status: {}",
                    reader.serial_number, reader.id, reader.status
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to list readers: {}", e);
        }
    }

    // Test 6: List checkouts
    println!("\n6. Testing list checkouts...");
    match client.list_checkouts(None).await {
        Ok(checkouts) => {
            println!("✅ Found {} checkouts:", checkouts.len());
            for checkout in &checkouts {
                println!(
                    "   - {}: {} {} ({})",
                    checkout.id, checkout.amount, checkout.currency, checkout.status
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to list checkouts: {}", e);
        }
    }

    // Test 7: List customer payment instruments
    println!("\n7. Testing list customer payment instruments...");
    match client.list_customer_payment_instruments("cust_12345").await {
        Ok(instruments) => {
            println!("✅ Found {} payment instruments:", instruments.len());
            for instrument in &instruments {
                println!("   - {} ({})", instrument.token, instrument.instrument_type);
            }
        }
        Err(e) => {
            println!("❌ Failed to list payment instruments: {}", e);
        }
    }

    // Test 8: List receipts
    println!("\n8. Testing list receipts...");
    let receipt_query = sumup_rs::receipts::ReceiptListQuery {
        mid: "test-merchant".to_string(),
        limit: Some(5),
        offset: Some(0),
    };

    match client.list_receipts(&receipt_query).await {
        Ok(receipts) => {
            println!("✅ Found {} receipts:", receipts.receipts.len());
            for receipt in &receipts.receipts {
                println!(
                    "   - {}: {} {} ({})",
                    receipt.id, receipt.amount, receipt.currency, receipt.status
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to list receipts: {}", e);
        }
    }

    println!("\n=== API Explorer Complete ===");
    Ok(())
}
