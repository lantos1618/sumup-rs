# SumUp API Client Implementation Guide

This document provides a step-by-step guide for implementing the actual HTTP logic in the SumUp API client scaffold.

## Current Status

✅ **Completed:**
- Complete project structure with all modules
- Comprehensive data models for all API endpoints
- Function signatures for all API operations
- Error handling infrastructure
- Client initialization and configuration
- Documentation and examples

🔄 **Next Steps:**
- Implement HTTP request logic in each module
- Add comprehensive tests
- Add CI/CD pipeline
- Publish to crates.io

## Implementation Strategy

### 1. Understanding the Pattern

Each API function follows this pattern:

```rust
pub async fn function_name(&self, params...) -> Result<ResponseType> {
    // 1. Build URL
    let url = self.build_url("/v0.1/endpoint")?;
    
    // 2. Make HTTP request
    let response = self.http_client
        .method(url)  // GET, POST, PUT, DELETE
        .bearer_auth(&self.api_key)
        .json(body)  // For POST/PUT requests
        .send()
        .await?;
    
    // 3. Handle response
    if response.status().is_success() {
        let data = response.json::<ResponseType>().await?;
        Ok(data)
    } else {
        let status = response.status().as_u16();
        let error_text = response.text().await.unwrap_or_default();
        
        Err(Error::ApiError {
            status,
            message: error_text,
        })
    }
}
```

### 2. Implementation Order

Start with the most commonly used endpoints:

1. **Checkouts** (`src/checkouts.rs`) - Core payment functionality
2. **Customers** (`src/customers.rs`) - Customer management
3. **Transactions** (`src/transactions.rs`) - Transaction history
4. **Merchant** (`src/merchant.rs`) - Merchant profile
5. **Payouts** (`src/payouts.rs`) - Payout management
6. **Receipts** (`src/receipts.rs`) - Receipt generation
7. **Readers** (`src/readers.rs`) - Hardware management
8. **Memberships** (`src/memberships.rs`) - Team management
9. **Members** (`src/members.rs`) - Team member management
10. **Roles** (`src/roles.rs`) - Permission management

### 3. Example Implementation

Here's how to implement the `create_checkout` function in `src/checkouts.rs`:

```rust
// Replace the unimplemented!() with:
pub async fn create_checkout(&self, body: &CreateCheckoutRequest) -> Result<Checkout> {
    let url = self.build_url("/v0.1/checkouts")?;
    
    let response = self.http_client
        .post(url)
        .bearer_auth(&self.api_key)
        .json(body)
        .send()
        .await?;
    
    if response.status().is_success() {
        let checkout = response.json::<Checkout>().await?;
        Ok(checkout)
    } else {
        let status = response.status().as_u16();
        let error_text = response.text().await.unwrap_or_default();
        
        Err(Error::ApiError {
            status,
            message: error_text,
        })
    }
}
```

### 4. Query Parameters

For endpoints that accept query parameters:

```rust
pub async fn list_checkouts(&self, checkout_reference: Option<&str>) -> Result<Vec<Checkout>> {
    let mut url = self.build_url("/v0.1/checkouts")?;
    
    if let Some(ref_) = checkout_reference {
        url.query_pairs_mut().append_pair("checkout_reference", ref_);
    }
    
    let response = self.http_client
        .get(url)
        .bearer_auth(&self.api_key)
        .send()
        .await?;
    
    if response.status().is_success() {
        let checkouts = response.json::<Vec<Checkout>>().await?;
        Ok(checkouts)
    } else {
        let status = response.status().as_u16();
        let error_text = response.text().await.unwrap_or_default();
        
        Err(Error::ApiError {
            status,
            message: error_text,
        })
    }
}
```

### 5. Path Parameters

For endpoints with path parameters:

```rust
pub async fn retrieve_checkout(&self, checkout_id: &str) -> Result<Checkout> {
    let url = self.build_url(&format!("/v0.1/checkouts/{}", checkout_id))?;
    
    let response = self.http_client
        .get(url)
        .bearer_auth(&self.api_key)
        .send()
        .await?;
    
    if response.status().is_success() {
        let checkout = response.json::<Checkout>().await?;
        Ok(checkout)
    } else {
        let status = response.status().as_u16();
        let error_text = response.text().await.unwrap_or_default();
        
        Err(Error::ApiError {
            status,
            message: error_text,
        })
    }
}
```

## Testing Strategy

### 1. Unit Tests

Create tests for each module:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_create_checkout() {
        // Test implementation
    }
}
```

### 2. Integration Tests

Create integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use sumup_rs::{SumUpClient, CreateCheckoutRequest};

#[tokio::test]
async fn test_checkout_workflow() {
    let client = SumUpClient::new("test-api-key".to_string(), true).unwrap();
    
    // Test full workflow
}
```

### 3. Mock Testing

Use `wiremock` or similar for HTTP mocking:

```rust
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_create_checkout_mock() {
    let mock_server = MockServer::start().await;
    
    Mock::given(method("POST"))
        .and(path("/v0.1/checkouts"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;
    
    // Test with mock server
}
```

## Error Handling Improvements

### 1. Add More Specific Error Types

```rust
#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Json(serde_json::Error),
    Url(url::ParseError),
    ApiError { status: u16, message: String },
    // Add more specific errors
    AuthenticationError,
    RateLimitError,
    ValidationError { field: String, message: String },
}
```

### 2. Add Retry Logic

```rust
use tokio::time::{sleep, Duration};

pub async fn create_checkout_with_retry(&self, body: &CreateCheckoutRequest) -> Result<Checkout> {
    let mut attempts = 0;
    let max_attempts = 3;
    
    loop {
        match self.create_checkout(body).await {
            Ok(checkout) => return Ok(checkout),
            Err(Error::ApiError { status, .. }) if status == 429 && attempts < max_attempts => {
                attempts += 1;
                sleep(Duration::from_secs(2u64.pow(attempts))).await;
                continue;
            }
            Err(e) => return Err(e),
        }
    }
}
```

## Performance Optimizations

### 1. Connection Pooling

The `reqwest::Client` already handles connection pooling, but you can configure it:

```rust
use reqwest::Client;

let client = Client::builder()
    .pool_max_idle_per_host(10)
    .timeout(Duration::from_secs(30))
    .build()?;
```

### 2. Request Batching

For bulk operations, implement batching:

```rust
pub async fn create_multiple_checkouts(&self, requests: Vec<CreateCheckoutRequest>) -> Result<Vec<Checkout>> {
    let mut checkouts = Vec::new();
    
    for request in requests {
        let checkout = self.create_checkout(&request).await?;
        checkouts.push(checkout);
    }
    
    Ok(checkouts)
}
```

## Documentation

### 1. Add Examples

Add examples to each function:

```rust
/// Creates a new payment checkout resource.
///
/// # Examples
///
/// ```rust
/// use sumup_rs::{SumUpClient, CreateCheckoutRequest};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = SumUpClient::new("your-api-key".to_string(), true)?;
///     
///     let request = CreateCheckoutRequest {
///         checkout_reference: "order-123".to_string(),
///         amount: 29.99,
///         currency: "EUR".to_string(),
///         merchant_code: "your-merchant-code".to_string(),
///         description: Some("Coffee and pastry".to_string()),
///         return_url: Some("https://your-site.com/return".to_string()),
///         customer_id: None,
///         purpose: None,
///         redirect_url: None,
///     };
///     
///     let checkout = client.create_checkout(&request).await?;
///     println!("Created checkout: {:?}", checkout);
///     
///     Ok(())
/// }
/// ```
pub async fn create_checkout(&self, body: &CreateCheckoutRequest) -> Result<Checkout> {
    // Implementation
}
```

### 2. Generate Documentation

```bash
cargo doc --no-deps --open
```

## Publishing

### 1. Version Management

Update version in `Cargo.toml`:

```toml
[package]
name = "sumup-rs"
version = "0.1.0"  # Increment as needed
```

### 2. Publish to crates.io

```bash
cargo publish
```

## Next Steps Checklist

- [ ] Implement HTTP logic in `src/checkouts.rs`
- [ ] Implement HTTP logic in `src/customers.rs`
- [ ] Implement HTTP logic in `src/transactions.rs`
- [ ] Implement HTTP logic in `src/merchant.rs`
- [ ] Implement HTTP logic in `src/payouts.rs`
- [ ] Implement HTTP logic in `src/receipts.rs`
- [ ] Implement HTTP logic in `src/readers.rs`
- [ ] Implement HTTP logic in `src/memberships.rs`
- [ ] Implement HTTP logic in `src/members.rs`
- [ ] Implement HTTP logic in `src/roles.rs`
- [ ] Add comprehensive tests
- [ ] Add CI/CD pipeline
- [ ] Add more examples
- [ ] Improve error handling
- [ ] Add performance optimizations
- [ ] Generate and publish documentation
- [ ] Publish to crates.io

## Resources

- [SumUp API Documentation](https://developer.sumup.com/)
- [reqwest Documentation](https://docs.rs/reqwest/)
- [serde Documentation](https://serde.rs/)
- [tokio Documentation](https://tokio.rs/)
- [Rust Async Book](https://rust-lang.github.io/async-book/) 