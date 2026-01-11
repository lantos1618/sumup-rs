use sumup_rs::{OAuthClient, OAuthConfig, Scope, SumUpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = OAuthConfig {
        client_id: std::env::var("SUMUP_CLIENT_ID")?,
        client_secret: std::env::var("SUMUP_CLIENT_SECRET")?,
        redirect_uri: "https://your-app.com/callback".to_string(),
    };

    let oauth = OAuthClient::new(config);

    // Generate authorization URL
    let auth_url = oauth.authorization_url(
        &[Scope::Payments, Scope::TransactionsHistory],
        Some("random-state"),
    );
    println!("Auth URL: {}", auth_url);

    // Exchange code for tokens (if provided)
    if let Ok(code) = std::env::var("SUMUP_AUTH_CODE") {
        let tokens = oauth.exchange_code(&code).await?;
        println!("Access token: {}...", &tokens.access_token[..20]);

        // Use with client
        #[allow(deprecated)]
        let client = SumUpClient::new(tokens.access_token, false)?;
        let profile = client.get_merchant_profile().await?;
        println!("Merchant: {}", profile.merchant_code);

        // Refresh token
        if let Some(refresh) = tokens.refresh_token {
            let new_tokens = oauth.refresh_token(&refresh).await?;
            println!("Refreshed: {}...", &new_tokens.access_token[..20]);
        }
    }

    Ok(())
}
