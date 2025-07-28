use sumup_rs::{SumUpClient, Merchant, MerchantProfile};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_get_merchant_profile_success() {
    // Arrange
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let expected_profile = MerchantProfile {
        merchant_code: "merchant_123".to_string(),
        name: "Test Merchant".to_string(),
        email: "merchant@test.com".to_string(),
        phone: "+1234567890".to_string(),
        address: sumup_rs::Address {
            line_1: Some("123 Test St".to_string()),
            line_2: None,
            city: Some("Test City".to_string()),
            state: None,
            postal_code: Some("12345".to_string()),
            country: Some("US".to_string()),
        },
        country: "US".to_string(),
        currency: "USD".to_string(),
        timezone: "America/New_York".to_string(),
    };

    let response_body = serde_json::json!({
        "merchant_code": "merchant_123",
        "name": "Test Merchant",
        "email": "merchant@test.com",
        "phone": "+1234567890",
        "address": {
            "line_1": "123 Test St",
            "line_2": null,
            "city": "Test City",
            "state": null,
            "postal_code": "12345",
            "country": "US"
        },
        "country": "US",
        "currency": "USD",
        "timezone": "America/New_York"
    });

    Mock::given(method("GET"))
        .and(path("/v0.1/me"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    // Act
    let result = client.get_merchant_profile().await;

    // Assert
    assert!(result.is_ok());
    let profile = result.unwrap();
    assert_eq!(profile.merchant_code, "merchant_123");
    assert_eq!(profile.name, "Test Merchant");
    assert_eq!(profile.email, "merchant@test.com");
    assert_eq!(profile.currency, "USD");
}

#[tokio::test]
async fn test_get_merchant_success() {
    // Arrange
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let response_body = serde_json::json!({
        "merchant_code": "merchant_456",
        "name": "Another Merchant",
        "email": "another@test.com",
        "phone": "+0987654321",
        "address": {
            "line_1": "456 Another St",
            "line_2": null,
            "city": "Another City",
            "state": null,
            "postal_code": "54321",
            "country": "CA"
        },
        "country": "CA",
        "currency": "CAD",
        "timezone": "America/Toronto",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z"
    });

    Mock::given(method("GET"))
        .and(path("/v0.1/merchants/merchant_456"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    // Act
    let result = client.get_merchant("merchant_456").await;

    // Assert
    assert!(result.is_ok());
    let merchant = result.unwrap();
    assert_eq!(merchant.merchant_code, "merchant_456");
    assert_eq!(merchant.name, "Another Merchant");
    assert_eq!(merchant.email, "another@test.com");
    assert_eq!(merchant.currency, "CAD");
}

#[tokio::test]
async fn test_list_merchants_success() {
    // Arrange
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let response_body = serde_json::json!([
        {
            "merchant_code": "merchant_1",
            "name": "First Merchant",
            "email": "first@test.com",
            "phone": "+1111111111",
            "address": {
                "line_1": "111 First St",
                "line_2": null,
                "city": "First City",
                "state": null,
                "postal_code": "11111",
                "country": "US"
            },
            "country": "US",
            "currency": "USD",
            "timezone": "America/New_York",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        },
        {
            "merchant_code": "merchant_2",
            "name": "Second Merchant",
            "email": "second@test.com",
            "phone": "+2222222222",
            "address": {
                "line_1": "222 Second St",
                "line_2": null,
                "city": "Second City",
                "state": null,
                "postal_code": "22222",
                "country": "CA"
            },
            "country": "CA",
            "currency": "CAD",
            "timezone": "America/Toronto",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/v0.1/me/merchants"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    // Act
    let result = client.list_merchants().await;

    // Assert
    assert!(result.is_ok());
    let merchants = result.unwrap();
    assert_eq!(merchants.len(), 2);
    assert_eq!(merchants[0].merchant_code, "merchant_1");
    assert_eq!(merchants[0].name, "First Merchant");
    assert_eq!(merchants[1].merchant_code, "merchant_2");
    assert_eq!(merchants[1].name, "Second Merchant");
}

#[tokio::test]
async fn test_get_merchant_profile_error() {
    // Arrange
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let error_response = serde_json::json!({
        "error_code": "unauthorized",
        "message": "Invalid API key"
    });

    Mock::given(method("GET"))
        .and(path("/v0.1/me"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    // Act
    let result = client.get_merchant_profile().await;

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err();
    match error {
        sumup_rs::Error::ApiError { status, body } => {
            assert_eq!(status, 401);
            assert!(body.error_code.as_ref().unwrap() == "unauthorized");
        }
        _ => panic!("Expected ApiError, got {:?}", error),
    }
}

#[tokio::test]
async fn test_get_merchant_not_found() {
    // Arrange
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let error_response = serde_json::json!({
        "error_code": "not_found",
        "message": "Merchant not found"
    });

    Mock::given(method("GET"))
        .and(path("/v0.1/merchants/nonexistent"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
        .mount(&mock_server)
        .await;

    // Act
    let result = client.get_merchant("nonexistent").await;

    // Assert
    assert!(result.is_err());
    let error = result.unwrap_err();
    match error {
        sumup_rs::Error::ApiError { status, body } => {
            assert_eq!(status, 404);
            assert!(body.error_code.as_ref().unwrap() == "not_found");
        }
        _ => panic!("Expected ApiError, got {:?}", error),
    }
} 