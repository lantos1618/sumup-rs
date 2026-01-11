# SumUp API Client for Rust

A type-safe Rust client for the [SumUp API](https://developer.sumup.com/), built from the official [OpenAPI specification](https://github.com/sumup/sumup-openapi).

> **Note**: This library was created with assistance from Claude (Anthropic).

## Installation

```toml
[dependencies]
sumup-rs = "0.2.0"
```

## Quick Start

```rust
use sumup_rs::{SumUpClient, CreateCheckoutRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")?;
    let client = SumUpClient::new(api_key, true)?;

    // Create a checkout
    let checkout = client.create_checkout(
        &CreateCheckoutRequest::new("order-123", 29.99, "EUR", "MERCHANT_CODE")
            .description("Coffee and pastry")
    ).await?;

    println!("Created checkout: {}", checkout.id);
    Ok(())
}
```

## API Coverage

| API | Status |
|-----|--------|
| Checkouts (+ 3DS) | Complete |
| Customers | Complete |
| Transactions | Complete |
| Merchants | Complete |
| Memberships | Complete |
| Members & Roles | Complete |
| Payouts | Complete |
| Receipts | Complete |
| Readers | Complete |
| Subaccounts | Deprecated (per OpenAPI spec) |
| OAuth | Complete |

## Examples

```bash
export SUMUP_API_SECRET_KEY="your-api-key"
cargo run --example basic_usage
cargo run --example checkout_example
```

## OpenAPI Spec Reference

This library tracks the [official SumUp OpenAPI specification](https://github.com/sumup/sumup-openapi/blob/main/openapi.yaml). Endpoints marked as deprecated in the spec have corresponding `#[deprecated]` attributes in this library.

## License

MIT
