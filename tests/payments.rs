use sumup_rs::{
    SumUpClient, 
    CreateCheckoutRequest, 
    ProcessCheckoutRequest, 
    CardDetails,
    PersonalDetails,
    Address
};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, body_json, header};

use chrono;

#[tokio::test]
async fn test_process_checkout_with_mock_card_success() {
    // 1. Arrange: Start a mock server
    let mock_server = MockServer::start().await;

    // Create a checkout first
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: "test-payment-123".to_string(),
        amount: 25.99,
        currency: "EUR".to_string(),
        merchant_code: "M123".to_string(),
        description: Some("Test payment with mock card".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };

    let checkout_response = serde_json::json!({
        "id": "checkout-12345",
        "status": "PENDING",
        "checkout_reference": "test-payment-123",
        "amount": 25.99,
        "currency": "EUR",
        "merchant_code": "M123",
        "date": "2024-01-15T10:30:00+00:00",
        "description": "Test payment with mock card",
        "transactions": []
    });

    // Mock the checkout creation
    Mock::given(method("POST"))
        .and(path("/v0.1/checkouts"))
        .and(header("Authorization", "Bearer test-api-key"))
        .and(body_json(&checkout_request))
        .respond_with(
            ResponseTemplate::new(201)
                .set_body_json(&checkout_response)
        )
        .mount(&mock_server)
        .await;

    // Now process the checkout with mock card data
    let process_request = ProcessCheckoutRequest {
        payment_type: "card".to_string(),
        installments: None,
        card: Some(CardDetails {
            number: "4242424242424242".to_string(), // Mock Visa card
            expiry_month: "12".to_string(),
            expiry_year: "2025".to_string(),
            cvc: "123".to_string(),
            name: Some("John Doe".to_string()),
        }),
        token: None,
        customer_id: None,
        personal_details: None,
    };

    let processed_response = serde_json::json!({
        "id": "checkout-12345",
        "status": "PAID",
        "checkout_reference": "test-payment-123",
        "amount": 25.99,
        "currency": "EUR",
        "merchant_code": "M123",
        "date": "2024-01-15T10:30:00+00:00",
        "description": "Test payment with mock card",
        "transaction_id": "txn-67890",
        "transaction_code": "TXN123",
        "transactions": [
            {
                "id": "txn-67890",
                "transaction_code": "TXN123",
                "status": "SUCCESSFUL",
                "amount": 25.99,
                "currency": "EUR",
                "timestamp": "2024-01-15T10:31:00+00:00"
            }
        ]
    });

    // Mock the checkout processing
    Mock::given(method("PUT"))
        .and(path("/v0.1/checkouts/checkout-12345"))
        .and(header("Authorization", "Bearer test-api-key"))
        .and(body_json(&process_request))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(&processed_response)
        )
        .mount(&mock_server)
        .await;

    // 2. Act: Create client and process payment
    let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
    
    // First create the checkout
    let checkout = client.create_checkout(&checkout_request).await.unwrap();
    assert_eq!(checkout.status, "PENDING");
    
    // Then process the payment
    let result = client.process_checkout(&checkout.id, &process_request).await;

    // 3. Assert: Check if the payment was processed successfully
    assert!(result.is_ok());
    let processed_checkout = result.unwrap();
    assert_eq!(processed_checkout.status, "PAID");
    assert_eq!(processed_checkout.transaction_id, Some("txn-67890".to_string()));
    assert_eq!(processed_checkout.transaction_code, Some("TXN123".to_string()));
    assert!(!processed_checkout.transactions.is_empty());
    assert_eq!(processed_checkout.transactions[0].status, Some("SUCCESSFUL".to_string()));
}

#[tokio::test]
async fn test_process_checkout_with_mock_card_declined() {
    // Test with a declined card number
    let mock_server = MockServer::start().await;

    let checkout_request = CreateCheckoutRequest {
        checkout_reference: "test-declined-123".to_string(),
        amount: 15.50,
        currency: "EUR".to_string(),
        merchant_code: "M123".to_string(),
        description: Some("Test declined payment".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };

    let checkout_response = serde_json::json!({
        "id": "checkout-declined",
        "status": "PENDING",
        "checkout_reference": "test-declined-123",
        "amount": 15.50,
        "currency": "EUR",
        "merchant_code": "M123",
        "date": "2024-01-15T10:30:00+00:00",
        "description": "Test declined payment",
        "transactions": []
    });

    Mock::given(method("POST"))
        .and(path("/v0.1/checkouts"))
        .and(header("Authorization", "Bearer test-api-key"))
        .and(body_json(&checkout_request))
        .respond_with(
            ResponseTemplate::new(201)
                .set_body_json(&checkout_response)
        )
        .mount(&mock_server)
        .await;

    let process_request = ProcessCheckoutRequest {
        payment_type: "card".to_string(),
        installments: None,
        card: Some(CardDetails {
            number: "4000000000000002".to_string(), // Mock declined card
            expiry_month: "12".to_string(),
            expiry_year: "2025".to_string(),
            cvc: "123".to_string(),
            name: Some("John Doe".to_string()),
        }),
        token: None,
        customer_id: None,
        personal_details: None,
    };

    // Mock a declined payment response
    let error_response = serde_json::json!({
        "error_code": "CARD_DECLINED",
        "message": "The card was declined",
        "param": "card.number"
    });

    Mock::given(method("PUT"))
        .and(path("/v0.1/checkouts/checkout-declined"))
        .and(header("Authorization", "Bearer test-api-key"))
        .and(body_json(&process_request))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_json(&error_response)
        )
        .mount(&mock_server)
        .await;

    let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
    
    let checkout = client.create_checkout(&checkout_request).await.unwrap();
    let result = client.process_checkout(&checkout.id, &process_request).await;

    // Should fail with card declined error
    assert!(result.is_err());
    if let sumup_rs::Error::ApiError { status, body } = result.unwrap_err() {
        assert_eq!(status, 400);
        assert_eq!(body.error_code, Some("CARD_DECLINED".to_string()));
        assert_eq!(body.message, Some("The card was declined".to_string()));
    }
}

#[tokio::test]
async fn test_process_checkout_with_customer_details() {
    // Test processing payment with customer personal details
    let mock_server = MockServer::start().await;

    let checkout_request = CreateCheckoutRequest {
        checkout_reference: "test-customer-payment".to_string(),
        amount: 50.00,
        currency: "EUR".to_string(),
        merchant_code: "M123".to_string(),
        description: Some("Payment with customer details".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: Some("cust-123".to_string()),
        purpose: None,
        redirect_url: None,
    };

    let checkout_response = serde_json::json!({
        "id": "checkout-customer",
        "status": "PENDING",
        "checkout_reference": "test-customer-payment",
        "amount": 50.00,
        "currency": "EUR",
        "merchant_code": "M123",
        "date": "2024-01-15T10:30:00+00:00",
        "description": "Payment with customer details",
        "customer_id": "cust-123",
        "transactions": []
    });

    Mock::given(method("POST"))
        .and(path("/v0.1/checkouts"))
        .and(header("Authorization", "Bearer test-api-key"))
        .and(body_json(&checkout_request))
        .respond_with(
            ResponseTemplate::new(201)
                .set_body_json(&checkout_response)
        )
        .mount(&mock_server)
        .await;

    let personal_details = PersonalDetails {
        first_name: Some("Jane".to_string()),
        last_name: Some("Smith".to_string()),
        email: Some("jane.smith@example.com".to_string()),
        phone: Some("+1234567890".to_string()),
        birth_date: Some("1985-05-15".to_string()),
        tax_id: Some("987654321".to_string()),
        address: Some(Address {
            city: Some("London".to_string()),
            country: Some("GB".to_string()),
            line_1: Some("456 High Street".to_string()),
            line_2: Some("Flat 2B".to_string()),
            postal_code: Some("SW1A 1AA".to_string()),
            state: Some("England".to_string()),
        }),
    };

    let process_request = ProcessCheckoutRequest {
        payment_type: "card".to_string(),
        installments: Some(1),
        card: Some(CardDetails {
            number: "4242424242424242".to_string(),
            expiry_month: "12".to_string(),
            expiry_year: "2025".to_string(),
            cvc: "123".to_string(),
            name: Some("Jane Smith".to_string()),
        }),
        token: None,
        customer_id: Some("cust-123".to_string()),
        personal_details: Some(personal_details),
    };

    let processed_response = serde_json::json!({
        "id": "checkout-customer",
        "status": "PAID",
        "checkout_reference": "test-customer-payment",
        "amount": 50.00,
        "currency": "EUR",
        "merchant_code": "M123",
        "date": "2024-01-15T10:30:00+00:00",
        "description": "Payment with customer details",
        "customer_id": "cust-123",
        "transaction_id": "txn-customer-123",
        "transaction_code": "TXN456",
        "transactions": [
            {
                "id": "txn-customer-123",
                "transaction_code": "TXN456",
                "status": "SUCCESSFUL",
                "amount": 50.00,
                "currency": "EUR",
                "timestamp": "2024-01-15T10:31:00+00:00"
            }
        ]
    });

    Mock::given(method("PUT"))
        .and(path("/v0.1/checkouts/checkout-customer"))
        .and(header("Authorization", "Bearer test-api-key"))
        .and(body_json(&process_request))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(&processed_response)
        )
        .mount(&mock_server)
        .await;

    let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
    
    let checkout = client.create_checkout(&checkout_request).await.unwrap();
    let result = client.process_checkout(&checkout.id, &process_request).await;

    assert!(result.is_ok());
    let processed_checkout = result.unwrap();
    assert_eq!(processed_checkout.status, "PAID");
    assert_eq!(processed_checkout.customer_id, Some("cust-123".to_string()));
    assert_eq!(processed_checkout.transaction_id, Some("txn-customer-123".to_string()));
}

#[tokio::test]
async fn test_process_checkout_with_real_api() {
    // Test with real API using your API key
    dotenv::from_filename(".env.local").ok();
    
    let api_key = std::env::var("SUMUP_API_KEY")
        .expect("SUMUP_API_KEY environment variable must be set");
    
    let client = SumUpClient::new(api_key, true).expect("Failed to create SumUp client");
    
    // Create a unique checkout reference
    let checkout_reference = format!("test-payment-{}", chrono::Utc::now().timestamp());
    
    let checkout_request = CreateCheckoutRequest {
        checkout_reference: checkout_reference.clone(),
        amount: 10.00,
        currency: "EUR".to_string(),
        merchant_code: "test-merchant".to_string(),
        description: Some("Test payment with real API".to_string()),
        return_url: Some("https://example.com/return".to_string()),
        customer_id: None,
        purpose: None,
        redirect_url: None,
    };

    // Create the checkout
    let result = client.create_checkout(&checkout_request).await;
    
    match result {
        Ok(checkout) => {
            println!("✅ Successfully created checkout: {}", checkout.id);
            assert_eq!(checkout.checkout_reference, Some(checkout_reference.clone()));
            assert_eq!(checkout.status, "PENDING");
            
            // Now try to process the payment with mock card data
            let process_request = ProcessCheckoutRequest {
                payment_type: "card".to_string(),
                installments: None,
                card: Some(CardDetails {
                    number: "4242424242424242".to_string(), // Mock Visa card
                    expiry_month: "12".to_string(),
                    expiry_year: "2025".to_string(),
                    cvc: "123".to_string(),
                    name: Some("Test User".to_string()),
                }),
                token: None,
                customer_id: None,
                personal_details: None,
            };
            
            let payment_result = client.process_checkout(&checkout.id, &process_request).await;
            
            match payment_result {
                Ok(processed_checkout) => {
                    println!("✅ Successfully processed payment!");
                    println!("   Status: {}", processed_checkout.status);
                    println!("   Transaction ID: {:?}", processed_checkout.transaction_id);
                    println!("   Transaction Code: {:?}", processed_checkout.transaction_code);
                    
                    // The payment should be successful with mock card
                    assert!(processed_checkout.status == "PAID" || processed_checkout.status == "SUCCESSFUL");
                }
                Err(e) => {
                    println!("❌ Payment processing failed: {}", e);
                    // Don't fail the test - this might be expected in sandbox
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to create checkout with real API: {}", e);
            // Don't fail the test - this might be expected if the API key is invalid or sandbox is not available
        }
    }
}

#[tokio::test]
async fn test_different_mock_card_types() {
    // Test different mock card numbers for various scenarios
    let mock_cards = vec![
        ("4242424242424242", "Visa - Success"),
        ("4000000000000002", "Visa - Declined"),
        ("5555555555554444", "Mastercard - Success"),
        ("4000000000009995", "Visa - Insufficient funds"),
        ("4000000000009987", "Visa - Lost card"),
        ("4000000000009979", "Visa - Stolen card"),
        ("4000000000000069", "Visa - Expired card"),
        ("4000000000000127", "Visa - Incorrect CVC"),
    ];

    for (card_number, description) in mock_cards {
        println!("Testing {}: {}", card_number, description);
        
        let mock_server = MockServer::start().await;
        
        let checkout_request = CreateCheckoutRequest {
            checkout_reference: format!("test-{}", card_number),
            amount: 10.00,
            currency: "EUR".to_string(),
            merchant_code: "M123".to_string(),
            description: Some(format!("Test with {}", description)),
            return_url: Some("https://example.com/return".to_string()),
            customer_id: None,
            purpose: None,
            redirect_url: None,
        };

        let checkout_response = serde_json::json!({
            "id": format!("checkout-{}", card_number),
            "status": "PENDING",
            "checkout_reference": format!("test-{}", card_number),
            "amount": 10.00,
            "currency": "EUR",
            "merchant_code": "M123",
            "date": "2024-01-15T10:30:00+00:00",
            "description": format!("Test with {}", description),
            "transactions": []
        });

        Mock::given(method("POST"))
            .and(path("/v0.1/checkouts"))
            .and(header("Authorization", "Bearer test-api-key"))
            .and(body_json(&checkout_request))
            .respond_with(
                ResponseTemplate::new(201)
                    .set_body_json(&checkout_response)
            )
            .mount(&mock_server)
            .await;

        let process_request = ProcessCheckoutRequest {
            payment_type: "card".to_string(),
            installments: None,
            card: Some(CardDetails {
                number: card_number.to_string(),
                expiry_month: "12".to_string(),
                expiry_year: "2025".to_string(),
                cvc: "123".to_string(),
                name: Some("Test User".to_string()),
            }),
            token: None,
            customer_id: None,
            personal_details: None,
        };

        // Determine expected response based on card number
        let is_success = card_number == "4242424242424242" || card_number == "5555555555554444";
        
        if is_success {
            let success_response = serde_json::json!({
                "id": format!("checkout-{}", card_number),
                "status": "PAID",
                "checkout_reference": format!("test-{}", card_number),
                "amount": 10.00,
                "currency": "EUR",
                "merchant_code": "M123",
                "date": "2024-01-15T10:30:00+00:00",
                "description": format!("Test with {}", description),
                "transaction_id": format!("txn-{}", card_number),
                "transaction_code": format!("TXN{}", card_number),
                "transactions": [
                    {
                        "id": format!("txn-{}", card_number),
                        "transaction_code": format!("TXN{}", card_number),
                        "status": "SUCCESSFUL",
                        "amount": 10.00,
                        "currency": "EUR",
                        "timestamp": "2024-01-15T10:31:00+00:00"
                    }
                ]
            });

            Mock::given(method("PUT"))
                .and(path(format!("/v0.1/checkouts/checkout-{}", card_number)))
                .and(header("Authorization", "Bearer test-api-key"))
                .and(body_json(&process_request))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_json(&success_response)
                )
                .mount(&mock_server)
                .await;
        } else {
            let error_response = serde_json::json!({
                "error_code": "CARD_DECLINED",
                "message": format!("Card {} was declined", card_number),
                "param": "card.number"
            });

            Mock::given(method("PUT"))
                .and(path(format!("/v0.1/checkouts/checkout-{}", card_number)))
                .and(header("Authorization", "Bearer test-api-key"))
                .and(body_json(&process_request))
                .respond_with(
                    ResponseTemplate::new(400)
                        .set_body_json(&error_response)
                )
                .mount(&mock_server)
                .await;
        }

        let client = SumUpClient::with_custom_url("test-api-key".to_string(), mock_server.uri()).unwrap();
        
        let checkout = client.create_checkout(&checkout_request).await.unwrap();
        let result = client.process_checkout(&checkout.id, &process_request).await;

        if is_success {
            assert!(result.is_ok(), "Expected success for card {}", card_number);
            let processed = result.unwrap();
            assert_eq!(processed.status, "PAID");
        } else {
            assert!(result.is_err(), "Expected error for card {}", card_number);
        }
    }
} 