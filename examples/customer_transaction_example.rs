#![allow(clippy::type_complexity)]
use sumup_rs::{
    Address, CreateCustomerRequest, PersonalDetails, SumUpClient, TransactionHistoryQuery,
    UpdateCustomerRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your API key
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");

    let client = SumUpClient::new(api_key, false)?;

    // Example 1: Create a new customer with full details
    println!("=== Creating a new customer ===");

    let customer_request = CreateCustomerRequest {
        customer_id: format!("cust_{}", chrono::Utc::now().timestamp()),
        personal_details: Some(PersonalDetails {
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: Some("john.doe@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            birth_date: Some("1990-01-01".to_string()),
            tax_id: Some("123456789".to_string()),
            address: Some(Address {
                city: Some("New York".to_string()),
                country: Some("US".to_string()),
                line_1: Some("123 Main St".to_string()),
                line_2: Some("Apt 4B".to_string()),
                postal_code: Some("10001".to_string()),
                state: Some("NY".to_string()),
            }),
        }),
    };

    match client.create_customer(&customer_request).await {
        Ok(customer) => {
            println!("✅ Customer created successfully!");
            println!("   Customer ID: {}", customer.customer_id);
            if let Some(details) = &customer.personal_details {
                println!(
                    "   Name: {} {}",
                    details.first_name.as_deref().unwrap_or("N/A"),
                    details.last_name.as_deref().unwrap_or("N/A")
                );
                println!("   Email: {}", details.email.as_deref().unwrap_or("N/A"));
            }
        }
        Err(e) => {
            println!("❌ Failed to create customer: {:?}", e);
            return Err(e.into());
        }
    }

    // Example 2: Retrieve the customer we just created
    println!("\n=== Retrieving customer ===");

    match client
        .retrieve_customer(&customer_request.customer_id)
        .await
    {
        Ok(customer) => {
            println!("✅ Customer retrieved successfully!");
            println!("   Customer ID: {}", customer.customer_id);
        }
        Err(e) => {
            println!("❌ Failed to retrieve customer: {:?}", e);
            return Err(e.into());
        }
    }

    // Example 3: Update customer information
    println!("\n=== Updating customer ===");

    let update_request = UpdateCustomerRequest {
        personal_details: Some(PersonalDetails {
            first_name: Some("John".to_string()),
            last_name: Some("Smith".to_string()), // Changed last name
            email: Some("john.smith@example.com".to_string()), // Changed email
            phone: Some("+1234567890".to_string()),
            birth_date: Some("1990-01-01".to_string()),
            tax_id: Some("123456789".to_string()),
            address: Some(Address {
                city: Some("Los Angeles".to_string()), // Changed city
                country: Some("US".to_string()),
                line_1: Some("456 Oak Ave".to_string()), // Changed address
                line_2: Some("Suite 100".to_string()),
                postal_code: Some("90210".to_string()),
                state: Some("CA".to_string()), // Changed state
            }),
        }),
    };

    match client
        .update_customer(&customer_request.customer_id, &update_request)
        .await
    {
        Ok(customer) => {
            println!("✅ Customer updated successfully!");
            if let Some(details) = &customer.personal_details {
                println!(
                    "   Updated Name: {} {}",
                    details.first_name.as_deref().unwrap_or("N/A"),
                    details.last_name.as_deref().unwrap_or("N/A")
                );
                println!(
                    "   Updated Email: {}",
                    details.email.as_deref().unwrap_or("N/A")
                );
                if let Some(addr) = &details.address {
                    println!("   Updated City: {}", addr.city.as_deref().unwrap_or("N/A"));
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to update customer: {:?}", e);
            return Err(e.into());
        }
    }

    // Example 4: List customer payment instruments
    println!("\n=== Listing customer payment instruments ===");

    match client
        .list_customer_payment_instruments(&customer_request.customer_id)
        .await
    {
        Ok(instruments) => {
            println!("✅ Found {} payment instruments", instruments.len());
            for (i, instrument) in instruments.iter().enumerate() {
                println!(
                    "   {}. Token: {}, Type: {}, Active: {}",
                    i + 1,
                    instrument.token,
                    instrument.instrument_type,
                    instrument.active
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to list payment instruments: {:?}", e);
            // Don't return error here as this might be expected for new customers
        }
    }

    // Example 5: List transaction history (requires a merchant code)
    println!("\n=== Listing transaction history ===");

    // Note: You'll need to replace "your_merchant_code" with an actual merchant code
    let merchant_code =
        std::env::var("SUMUP_MERCHANT_CODE").unwrap_or_else(|_| "your_merchant_code".to_string());

    if merchant_code != "your_merchant_code" {
        let query = TransactionHistoryQuery {
            limit: Some(10),     // Limit to 10 transactions
            order: Some("desc"), // Sort by newest first
            newest_time: None,   // No specific newest_time filter
            oldest_time: None,
        };
        match client
            .list_transactions_history(&merchant_code, &query)
            .await
        {
            Ok(history) => {
                println!("✅ Found {} transactions", history.items.len());
                for (i, transaction) in history.items.iter().enumerate() {
                    println!(
                        "   {}. ID: {}, Amount: {} {}, Status: {}",
                        i + 1,
                        transaction.id,
                        transaction.amount,
                        transaction.currency,
                        transaction.status.as_deref().unwrap_or("N/A")
                    );
                }
            }
            Err(e) => {
                println!("❌ Failed to list transactions: {:?}", e);
                // Don't return error here as this might be expected
            }
        }
    } else {
        println!("⚠️  Set SUMUP_MERCHANT_CODE environment variable to test transaction history");
    }

    // Example 6: Retrieve a specific transaction (if we have a transaction ID)
    println!("\n=== Retrieving specific transaction ===");

    let transaction_id =
        std::env::var("SUMUP_TRANSACTION_ID").unwrap_or_else(|_| "your_transaction_id".to_string());

    if transaction_id != "your_transaction_id" && merchant_code != "your_merchant_code" {
        match client
            .retrieve_transaction_by_id(&merchant_code, &transaction_id)
            .await
        {
            Ok(transaction) => {
                println!("✅ Transaction retrieved successfully!");
                println!("   ID: {}", transaction.id);
                println!("   Amount: {} {}", transaction.amount, transaction.currency);
                println!(
                    "   Status: {}",
                    transaction.status.as_deref().unwrap_or("N/A")
                );
            }
            Err(e) => {
                println!("❌ Failed to retrieve transaction: {:?}", e);
            }
        }
    } else {
        println!("⚠️  Set SUMUP_TRANSACTION_ID environment variable to test transaction retrieval");
    }

    println!("\n=== Example completed successfully! ===");
    Ok(())
}
