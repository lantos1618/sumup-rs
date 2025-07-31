# SumUp API Client for Rust

A comprehensive, type-safe Rust client for the SumUp API. This library provides a complete interface to all SumUp API endpoints with full async/await support.

## Features

- **Complete API Coverage**: All SumUp API endpoints are supported, including payments, customers, transactions, and team management.
- **Type Safety**: Full Rust type definitions for all request/response models ensure correctness at compile time.
- **Async/Await**: Built on Tokio for high-performance, non-blocking I/O.
- **Robust Error Handling**: Comprehensive error types for handling API, network, and validation errors gracefully.
- **3DS Support**: Correctly handles 3DS payment flows by differentiating between immediate success and required authentication steps.
- **Ergonomic Queries**: Type-safe query builders for endpoints with complex filtering and pagination.
- **Sandbox Support**: Easy switching between production and sandbox environments for safe development and testing.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sumup-rs = "0.2.0"
```

## Quick Start

```rust
use sumup_rs::{SumUpClient, CreateCheckoutRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("Please set SUMUP_API_SECRET_KEY environment variable");
    
    // Create a client (use sandbox for testing)
    let client = SumUpClient::new(api_key, true)?;

    // Get your merchant profile to use the correct merchant code
    let merchant_profile = client.get_merchant_profile().await?;
    
    // Create a checkout
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: "order-123".to_string(),
        amount: 29.99,
        currency: merchant_profile.currency.clone(),
        merchant_code: merchant_profile.merchant_code.clone(),
        description: Some("Coffee and pastry".to_string()),
        return_url: Some("https://your-site.com/return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };
    
    let checkout = client.create_checkout(&checkout_request).await?;
    println!("Created checkout: {:?}", checkout.id);
    
    Ok(())
}
```

## API Coverage

This client supports all SumUp API endpoints:

### Checkouts
- Create, retrieve, process, and deactivate checkouts.
- **Correctly handles 3DS payment flows.**
- List checkouts with advanced filtering.
- Get available payment methods.

### Customers
- Full CRUD operations for customer management.
- Manage customer payment instruments.

### Transactions
- List transaction history with powerful, type-safe query parameters.
- Retrieve full details for any transaction.
- Refund transactions.

### Merchants
- Retrieve profiles for the authenticated user or specific merchants.
- List all merchants accessible to the authenticated user.

### Team Management (Memberships, Roles, Members)
- **Fully functional and corrected implementation.**
- Full CRUD operations for memberships, custom roles, and team members.
- Assign roles and permissions to team members.

### Payouts, Receipts & Readers
- List and retrieve financial payouts and transaction receipts.
- Manage physical card readers and initiate in-person payments.

## Error Handling

The client uses a custom `Error` type that provides structured information about API errors:

```rust
use sumup_rs::{Error, Result};

// ... inside an async function
match client.create_checkout(&request).await {
    Ok(checkout) => println!("Success: {:?}", checkout),
    Err(Error::Http(e)) => eprintln!("HTTP error: {}", e),
    Err(Error::Json(e)) => eprintln!("JSON error: {}", e),
    Err(Error::ApiError { status, body }) => {
        eprintln!("API error (Status {}): {}", status, body.message.as_deref().unwrap_or("No details"));
    }
    Err(e) => eprintln!("An unexpected error occurred: {}", e),
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This is not an official SumUp product. This library is provided as-is without any warranty. 