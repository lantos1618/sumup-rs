use sumup_rs::{SumUpClient, CreateCheckoutRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client (use sandbox for testing)
    let client = SumUpClient::new("your-api-key-here".to_string(), true)?;
    
    // Create a checkout request
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: "order-123".to_string(),
        amount: 25.50,
        currency: "EUR".to_string(),
        merchant_code: "M123456".to_string(),
        description: Some("Coffee and pastry".to_string()),
        return_url: Some("https://your-app.com/return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };
    
    println!("Creating checkout...");
    
    // Create the checkout
    match client.create_checkout(&checkout_request).await {
        Ok(checkout) => {
            println!("✅ Checkout created successfully!");
            println!("   ID: {}", checkout.id);
            println!("   Status: {}", checkout.status);
            println!("   Amount: {} {}", checkout.amount, checkout.currency);
            println!("   Reference: {:?}", checkout.checkout_reference);
            
            // Now let's retrieve the checkout
            println!("\nRetrieving checkout...");
            match client.retrieve_checkout(&checkout.id).await {
                Ok(retrieved_checkout) => {
                    println!("✅ Checkout retrieved successfully!");
                    println!("   ID: {}", retrieved_checkout.id);
                    println!("   Status: {}", retrieved_checkout.status);
                }
                Err(e) => {
                    println!("❌ Failed to retrieve checkout: {}", e);
                }
            }
            
            // List checkouts with the same reference
            println!("\nListing checkouts with reference 'order-123'...");
            match client.list_checkouts(Some("order-123")).await {
                Ok(checkouts) => {
                    println!("✅ Found {} checkout(s)", checkouts.len());
                    for (i, checkout) in checkouts.iter().enumerate() {
                        println!("   {}. ID: {}, Status: {}", i + 1, checkout.id, checkout.status);
                    }
                }
                Err(e) => {
                    println!("❌ Failed to list checkouts: {}", e);
                }
            }
            
            // Get available payment methods
            println!("\nGetting available payment methods...");
            match client.get_available_payment_methods("M123456", Some(25.50), Some("EUR")).await {
                Ok(methods) => {
                    println!("✅ Found {} available payment method(s)", methods.available_payment_methods.len());
                    for method in &methods.available_payment_methods {
                        println!("   - {}", method.id);
                    }
                }
                Err(e) => {
                    println!("❌ Failed to get payment methods: {}", e);
                }
            }
            
            // Example of processing a checkout (commented out as it requires real payment details)
            /*
            let process_request = ProcessCheckoutRequest {
                payment_type: "card".to_string(),
                installments: None,
                card: Some(CardDetails {
                    number: "4111111111111111".to_string(),
                    expiry_month: "12".to_string(),
                    expiry_year: "2025".to_string(),
                    cvc: "123".to_string(),
                    name: Some("John Doe".to_string()),
                }),
                token: None,
                customer_id: None,
            };
            
            match client.process_checkout(&checkout.id, &process_request).await {
                Ok(processed_checkout) => {
                    println!("✅ Checkout processed successfully!");
                    println!("   Status: {}", processed_checkout.status);
                }
                Err(e) => {
                    println!("❌ Failed to process checkout: {}", e);
                }
            }
            */
            
            // Deactivate the checkout
            println!("\nDeactivating checkout...");
            match client.deactivate_checkout(&checkout.id).await {
                Ok(deleted_checkout) => {
                    println!("✅ Checkout deactivated successfully!");
                    println!("   Status: {}", deleted_checkout.status);
                }
                Err(e) => {
                    println!("❌ Failed to deactivate checkout: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create checkout: {}", e);
        }
    }
    
    Ok(())
} 