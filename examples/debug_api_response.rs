use sumup_rs::SumUpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_KEY")
        .expect("SUMUP_API_KEY environment variable must be set");
    
    println!("Testing API key: {}...", &api_key[..10]);
    
    // Create client
    let client = SumUpClient::new(api_key.clone(), true)?;
    
    // Make a raw HTTP request to see the actual response
    let http_client = reqwest::Client::new();
    let response = http_client
        .get("https://api.sumup.com/v0.1/me")
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;
    
    println!("Status: {}", response.status());
    println!("Headers: {:?}", response.headers());
    
    let body = response.text().await?;
    println!("Raw response body:");
    println!("{}", body);
    
    // Try to parse as JSON
    match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(json) => {
            println!("✅ Valid JSON response:");
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        Err(e) => {
            println!("❌ Invalid JSON: {}", e);
        }
    }
    
    Ok(())
} 