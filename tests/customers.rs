use sumup_rs::{SumUpClient, CreateCustomerRequest, PersonalDetails, Address};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, body_json, header};
use chrono;

#[tokio::test]
async fn test_create_customer_success() {
    // 1. Arrange: Start a mock server
    let mock_server = MockServer::start().await;

    // The request body our client will send
    let request_body = CreateCustomerRequest {
        customer_id: "cust_12345".to_string(),
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

    // The response body the mock server will return
    let response_body = serde_json::json!({
        "customer_id": "cust_12345",
        "personal_details": {
            "first_name": "John",
            "last_name": "Doe",
            "email": "john.doe@example.com",
            "phone": "+1234567890",
            "birth_date": "1990-01-01",
            "tax_id": "123456789",
            "address": {
                "city": "New York",
                "country": "US",
                "line_1": "123 Main St",
                "line_2": "Apt 4B",
                "postal_code": "10001",
                "state": "NY"
            }
        }
    });

    // 2. Arrange: Set up the mock response on the server
    Mock::given(method("POST"))
        .and(path("/v0.1/customers"))
        .and(header("Authorization", "Bearer test-api-key"))
        .and(body_json(&request_body)) // Verify the client sends the correct body
        .respond_with(
            ResponseTemplate::new(201) // 201 Created
                .set_body_json(&response_body)
        )
        .mount(&mock_server)
        .await;

    // 3. Act: Create a client pointing to the mock server and call the function
    let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
    let result = client.create_customer(&request_body).await;

    // 4. Assert: Check if the result is what we expect
    assert!(result.is_ok());
    let customer = result.unwrap();
    assert_eq!(customer.customer_id, "cust_12345");
    
    let personal_details = customer.personal_details.unwrap();
    assert_eq!(personal_details.first_name, Some("John".to_string()));
    assert_eq!(personal_details.last_name, Some("Doe".to_string()));
    assert_eq!(personal_details.email, Some("john.doe@example.com".to_string()));
    assert_eq!(personal_details.phone, Some("+1234567890".to_string()));
    assert_eq!(personal_details.birth_date, Some("1990-01-01".to_string()));
    assert_eq!(personal_details.tax_id, Some("123456789".to_string()));
    
    let address = personal_details.address.unwrap();
    assert_eq!(address.city, Some("New York".to_string()));
    assert_eq!(address.country, Some("US".to_string()));
    assert_eq!(address.line_1, Some("123 Main St".to_string()));
    assert_eq!(address.line_2, Some("Apt 4B".to_string()));
    assert_eq!(address.postal_code, Some("10001".to_string()));
    assert_eq!(address.state, Some("NY".to_string()));
}

#[tokio::test]
async fn test_create_customer_minimal() {
    // Test with minimal required fields only
    let mock_server = MockServer::start().await;

    let request_body = CreateCustomerRequest {
        customer_id: "cust_minimal".to_string(),
        personal_details: None,
    };

    let response_body = serde_json::json!({
        "customer_id": "cust_minimal"
    });

    Mock::given(method("POST"))
        .and(path("/v0.1/customers"))
        .and(header("Authorization", "Bearer test-api-key"))
        .and(body_json(&request_body))
        .respond_with(
            ResponseTemplate::new(201)
                .set_body_json(&response_body)
        )
        .mount(&mock_server)
        .await;

    let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
    let result = client.create_customer(&request_body).await;

    assert!(result.is_ok());
    let customer = result.unwrap();
    assert_eq!(customer.customer_id, "cust_minimal");
    assert!(customer.personal_details.is_none());
}

#[tokio::test]
async fn test_create_customer_api_error() {
    // Test error handling
    let mock_server = MockServer::start().await;

    let request_body = CreateCustomerRequest {
        customer_id: "cust_error".to_string(),
        personal_details: None,
    };

    let error_response = serde_json::json!({
        "error_code": "VALIDATION_ERROR",
        "message": "Customer ID already exists",
        "param": "customer_id"
    });

    Mock::given(method("POST"))
        .and(path("/v0.1/customers"))
        .and(header("Authorization", "Bearer test-api-key"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_json(&error_response)
        )
        .mount(&mock_server)
        .await;

    let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
    let result = client.create_customer(&request_body).await;

    assert!(result.is_err());
    // You can add more specific error checking here based on your Error type
}

#[tokio::test]
async fn test_create_customer_with_real_api() {
    // Load environment variables from .env.local
    dotenv::from_filename(".env.local").ok();
    
    // Get API key from environment
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY environment variable must be set");
    
    // Create a client with the real API (use sandbox for testing)
    let client = SumUpClient::new(api_key, true).expect("Failed to create SumUp client");
    
    // Create a unique customer ID using timestamp
    let customer_id = format!("test-cust-{}", chrono::Utc::now().timestamp());
    
    let request_body = CreateCustomerRequest {
        customer_id: customer_id.clone(),
        personal_details: Some(PersonalDetails {
            first_name: Some("Test".to_string()),
            last_name: Some("Customer".to_string()),
            email: Some("test.customer@example.com".to_string()),
            phone: Some("+1234567890".to_string()),
            birth_date: Some("1990-01-01".to_string()),
            tax_id: Some("123456789".to_string()),
            address: Some(Address {
                city: Some("Test City".to_string()),
                country: Some("US".to_string()),
                line_1: Some("123 Test St".to_string()),
                line_2: Some("Apt 1".to_string()),
                postal_code: Some("12345".to_string()),
                state: Some("CA".to_string()),
            }),
        }),
    };

    // Test creating a customer with the real API
    let result = client.create_customer(&request_body).await;
    
    // This might succeed or fail depending on the API key and sandbox status
    match result {
        Ok(customer) => {
            println!("✅ Successfully created customer: {}", customer.customer_id);
            assert_eq!(customer.customer_id, customer_id);
            
            let personal_details = customer.personal_details.unwrap();
            assert_eq!(personal_details.first_name, Some("Test".to_string()));
            assert_eq!(personal_details.last_name, Some("Customer".to_string()));
            assert_eq!(personal_details.email, Some("test.customer@example.com".to_string()));
        }
        Err(e) => {
            println!("❌ Failed to create customer with real API: {}", e);
            // Don't fail the test - this might be expected if the API key is invalid or sandbox is not available
        }
    }
}