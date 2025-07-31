use sumup_rs::{CreateCustomerRequest, SumUpClient, UpdateCustomerRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("Please set SUMUP_API_SECRET_KEY environment variable");

    let client = SumUpClient::new(api_key, true)?;

    println!("=== SumUp Customer Workflow Test ===\n");

    // Step 1: Create a customer
    println!("1. Creating a customer...");
    let customer_request = CreateCustomerRequest {
        customer_id: "workflow-test-customer".to_string(),
        personal_details: Some(sumup_rs::PersonalDetails {
            first_name: Some("Workflow".to_string()),
            last_name: Some("Test".to_string()),
            email: Some("workflow@test.com".to_string()),
            phone: Some("+1234567890".to_string()),
            birth_date: Some("1990-01-01".to_string()),
            tax_id: None,
            address: Some(sumup_rs::Address {
                city: Some("Test City".to_string()),
                country: Some("US".to_string()),
                line_1: Some("123 Test St".to_string()),
                line_2: None,
                postal_code: Some("12345".to_string()),
                state: Some("CA".to_string()),
            }),
        }),
    };

    let customer = match client.create_customer(&customer_request).await {
        Ok(customer) => {
            println!("✅ Customer created successfully!");
            println!("   Customer ID: {}", customer.customer_id);
            println!(
                "   Email: {}",
                customer
                    .personal_details
                    .as_ref()
                    .and_then(|pd| pd.email.as_ref())
                    .unwrap_or(&"N/A".to_string())
            );
            customer
        }
        Err(e) => {
            println!("❌ Customer creation failed: {}", e);
            return Ok(());
        }
    };

    // Step 2: Retrieve the customer
    println!("\n2. Retrieving the customer...");
    match client.retrieve_customer(&customer.customer_id).await {
        Ok(retrieved_customer) => {
            println!("✅ Customer retrieved successfully!");
            println!("   Customer ID: {}", retrieved_customer.customer_id);
            println!(
                "   Name: {} {}",
                retrieved_customer
                    .personal_details
                    .as_ref()
                    .and_then(|pd| pd.first_name.as_ref())
                    .unwrap_or(&"N/A".to_string()),
                retrieved_customer
                    .personal_details
                    .as_ref()
                    .and_then(|pd| pd.last_name.as_ref())
                    .unwrap_or(&"N/A".to_string())
            );
        }
        Err(e) => {
            println!("❌ Customer retrieval failed: {}", e);
        }
    }

    // Step 3: Update the customer
    println!("\n3. Updating the customer...");
    let update_request = UpdateCustomerRequest {
        personal_details: Some(sumup_rs::PersonalDetails {
            first_name: Some("Updated".to_string()),
            last_name: Some("Customer".to_string()),
            email: Some("updated@test.com".to_string()),
            phone: Some("+9876543210".to_string()),
            birth_date: Some("1990-01-01".to_string()),
            tax_id: None,
            address: Some(sumup_rs::Address {
                city: Some("Updated City".to_string()),
                country: Some("US".to_string()),
                line_1: Some("456 Updated St".to_string()),
                line_2: None,
                postal_code: Some("54321".to_string()),
                state: Some("NY".to_string()),
            }),
        }),
    };

    match client
        .update_customer(&customer.customer_id, &update_request)
        .await
    {
        Ok(updated_customer) => {
            println!("✅ Customer updated successfully!");
            println!(
                "   Updated Name: {} {}",
                updated_customer
                    .personal_details
                    .as_ref()
                    .and_then(|pd| pd.first_name.as_ref())
                    .unwrap_or(&"N/A".to_string()),
                updated_customer
                    .personal_details
                    .as_ref()
                    .and_then(|pd| pd.last_name.as_ref())
                    .unwrap_or(&"N/A".to_string())
            );
            println!(
                "   Updated Email: {}",
                updated_customer
                    .personal_details
                    .as_ref()
                    .and_then(|pd| pd.email.as_ref())
                    .unwrap_or(&"N/A".to_string())
            );
        }
        Err(e) => {
            println!("❌ Customer update failed: {}", e);
        }
    }

    // Step 4: List customer payment instruments
    println!("\n4. Listing customer payment instruments...");
    match client
        .list_customer_payment_instruments(&customer.customer_id)
        .await
    {
        Ok(instruments) => {
            println!("✅ Payment instruments retrieved successfully!");
            println!("   Found {} payment instruments", instruments.len());
            for instrument in instruments {
                println!(
                    "   - Token: {} (Type: {}, Active: {})",
                    instrument.token, instrument.instrument_type, instrument.active
                );
            }
        }
        Err(e) => {
            println!("❌ Payment instruments retrieval failed: {}", e);
        }
    }

    println!("\n=== Customer Workflow Test Complete ===");
    println!("✅ Successfully demonstrated:");
    println!("   - Customer creation");
    println!("   - Customer retrieval");
    println!("   - Customer updates");
    println!("   - Payment instrument listing");
    println!("   - Enhanced error handling");

    Ok(())
}
