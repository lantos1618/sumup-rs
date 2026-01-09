use sumup_rs::{OAuthClient, OAuthConfig, Scope, SumUpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Setup OAuth client
    let config = OAuthConfig {
        client_id: std::env::var("SUMUP_CLIENT_ID")
            .expect("Set SUMUP_CLIENT_ID"),
        client_secret: std::env::var("SUMUP_CLIENT_SECRET")
            .expect("Set SUMUP_CLIENT_SECRET"),
        redirect_uri: "https://your-app.com/callback".to_string(),
    };

    let oauth = OAuthClient::new(config);

    // 2. Generate authorization URL (redirect user here)
    let auth_url = oauth.authorization_url(
        &[Scope::Payments, Scope::TransactionsHistory],
        Some("random-state-string"),
    );
    println!("Redirect user to:\n{}\n", auth_url);

    // 3. After user authorizes, exchange the code for tokens
    // (In real app, you'd get this from the callback URL query params)
    let code = std::env::var("SUMUP_AUTH_CODE").ok();

    if let Some(code) = code {
        println!("Exchanging code for tokens...");
        let tokens = oauth.exchange_code(&code).await?;

        println!("Access token: {}...", &tokens.access_token[..20]);
        println!("Expires in: {}s", tokens.expires_in);

        if let Some(refresh) = &tokens.refresh_token {
            println!("Refresh token: {}...", &refresh[..20.min(refresh.len())]);
        }

        // 4. Use the access token with SumUpClient
        let client = SumUpClient::new(tokens.access_token, false)?;
        let profile = client.get_merchant_profile().await?;
        println!("\nMerchant: {} ({})", profile.name, profile.merchant_code);

        // 5. Later, refresh the token when it expires
        if let Some(refresh_token) = tokens.refresh_token {
            println!("\nRefreshing token...");
            let new_tokens = oauth.refresh_token(&refresh_token).await?;
            println!("New access token: {}...", &new_tokens.access_token[..20]);
        }
    } else {
        println!("Set SUMUP_AUTH_CODE to exchange for tokens");
    }

    Ok(())
}
