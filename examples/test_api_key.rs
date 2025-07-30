use sumup_rs::SumUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");
    
    println!("Testing API key: {}...", &api_key[..10]);
    
    // Try to create a client
    let client = match SumUpClient::new(api_key, true) {
        Ok(client) => {
            println!("✅ Client created successfully");
            client
        }
        Err(e) => {
            println!("❌ Failed to create client: {}", e);
            return Err(e.into());
        }
    };
    
    // Try to get merchant profile (this should work with a valid key)
    println!("Testing merchant profile retrieval...");
    match client.get_merchant_profile().await {
        Ok(profile) => {
            println!("✅ API key is valid!");
            println!("   Merchant Code: {}", profile.merchant_code);
            println!("   Name: {}", profile.name);
            println!("   Country: {}", profile.country);
            println!("   Currency: {}", profile.currency);
            println!("   Phone: {}", profile.phone.as_deref().unwrap_or("Not provided"));
            if let Some(ref dba) = profile.doing_business_as {
                println!("   Email: {}", dba.email);
            }
        }
        Err(e) => {
            println!("❌ API key test failed: {}", e);
            println!("   This suggests the API key is invalid, expired, or lacks permissions");
        }
    }
    
    Ok(())
} 