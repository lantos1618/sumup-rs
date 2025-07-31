use sumup_rs::{SumUpClient, CreateRoleRequest, CreateMemberRequest, Role, Member, Membership};
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path};

#[tokio::test]
async fn test_create_role_success() {
    // Arrange
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let expected_role = Role {
        id: "role_123".to_string(),
        name: "Admin".to_string(),
        membership_id: "membership_123".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
        is_predefined: false,
        created_at: chrono::Utc::now(),
        updated_at: None,
    };

    let response_body = serde_json::json!({
        "id": "role_123",
        "name": "Admin",
        "membership_id": "membership_123",
        "permissions": ["read", "write"],
        "is_predefined": false,
        "created_at": expected_role.created_at.to_rfc3339(),
        "updated_at": null
    });

    Mock::given(method("POST"))
        .and(path("/v0.1/merchants/merchant_123/roles"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    // Act
    let request = CreateRoleRequest {
        name: "Admin".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    let result = client.create_role("merchant_123", &request).await;

    // Assert
    assert!(result.is_ok());
    let role = result.unwrap();
    assert_eq!(role.id, "role_123");
    assert_eq!(role.name, "Admin");
    assert_eq!(role.membership_id, "membership_123");
    assert_eq!(role.permissions, vec!["read", "write"]);
    assert_eq!(role.is_predefined, false);
}

#[tokio::test]
async fn test_create_member_success() {
    // Arrange
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let expected_member = Member {
        id: "member_123".to_string(),
        membership_id: "membership_123".to_string(),
        user: sumup_rs::User {
            id: "user_123".to_string(),
            email: "john@example.com".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            created_at: chrono::Utc::now(),
            updated_at: None,
        },
        status: "ACTIVE".to_string(),
        roles: vec!["role_123".to_string()],
        permissions: vec!["read".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: None,
    };

    let response_body = serde_json::json!({
        "id": "member_123",
        "membership_id": "membership_123",
        "user": {
            "id": "user_123",
            "email": "john@example.com",
            "first_name": "John",
            "last_name": "Doe",
            "created_at": expected_member.user.created_at.to_rfc3339(),
            "updated_at": null
        },
        "status": "ACTIVE",
        "roles": ["role_123"],
        "permissions": ["read"],
        "created_at": expected_member.created_at.to_rfc3339(),
        "updated_at": null
    });

    Mock::given(method("POST"))
        .and(path("/v0.1/merchants/merchant_123/members"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    // Act
    let request = CreateMemberRequest {
        email: "john@example.com".to_string(),
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        roles: Some(vec!["role_123".to_string()]),
    };
    let result = client.create_member("merchant_123", &request).await;

    // Assert
    assert!(result.is_ok());
    let member = result.unwrap();
    assert_eq!(member.id, "member_123");
    assert_eq!(member.membership_id, "membership_123");
    assert_eq!(member.user.email, "john@example.com");
    assert_eq!(member.user.first_name, Some("John".to_string()));
    assert_eq!(member.user.last_name, Some("Doe".to_string()));
    assert_eq!(member.status, "ACTIVE");
    assert_eq!(member.roles, vec!["role_123"]);
    assert_eq!(member.permissions, vec!["read"]);
}

#[tokio::test]
async fn test_list_payouts_success() {
    // Arrange
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let response_body = serde_json::json!({
        "payouts": [
            {
                "id": "payout_123",
                "merchant_code": "merchant_123",
                "amount": 100.50,
                "currency": "EUR",
                "status": "COMPLETED",
                "created_at": "2023-01-01T00:00:00Z",
                "completed_at": "2023-01-02T00:00:00Z",
                "bank_account": {
                    "iban": "DE89370400440532013000",
                    "bic": "COBADEFFXXX",
                    "account_holder_name": "Test Account"
                }
            }
        ]
    });

    Mock::given(method("GET"))
        .and(path("/v1.0/me/payouts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    // Act
    let query = sumup_rs::payouts::PayoutListQuery {
        start_date: "2023-01-01".to_string(),
        end_date: "2023-01-31".to_string(),
        limit: Some(10),
        offset: Some(0),
    };
    let result = client.list_payouts(&query).await;

    // Assert
    assert!(result.is_ok());
    let payouts = result.unwrap();
    assert_eq!(payouts.payouts.len(), 1);
    let payout = &payouts.payouts[0];
    assert_eq!(payout.id, "payout_123");
    assert_eq!(payout.amount, 100.50);
    assert_eq!(payout.currency, "EUR");
    assert_eq!(payout.status, "COMPLETED");
}

#[tokio::test]
async fn test_create_reader_checkout_success() {
    // Arrange
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let expected_checkout = sumup_rs::ReaderCheckoutResponse {
        id: "checkout_123".to_string(),
        status: "PENDING".to_string(),
        total_amount: sumup_rs::TotalAmount {
            value: 29.99,
            currency: "EUR".to_string(),
            minor_unit: 2,
        },
        description: "Coffee and pastry".to_string(),
        return_url: "https://example.com/return".to_string(),
        installments: None,
        customer_id: None,
        external_reference: Some("order-123".to_string()),
        transaction_id: None,
        created_at: chrono::Utc::now(),
        completed_at: None,
    };

    let response_body = serde_json::json!({
        "id": "checkout_123",
        "status": "PENDING",
        "total_amount": {
            "value": 29.99,
            "currency": "EUR",
            "minor_unit": 2
        },
        "description": "Coffee and pastry",
        "return_url": "https://example.com/return",
        "installments": null,
        "customer_id": null,
        "external_reference": "order-123",
        "transaction_id": null,
        "created_at": expected_checkout.created_at.to_rfc3339(),
        "completed_at": null
    });

    Mock::given(method("POST"))
        .and(path("/v0.1/me/readers/reader_123/checkout"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    // Act
    let request = sumup_rs::CreateReaderCheckoutRequest {
        total_amount: sumup_rs::TotalAmount {
            value: 29.99,
            currency: "EUR".to_string(),
            minor_unit: 2,
        },
        description: "Coffee and pastry".to_string(),
        return_url: "https://example.com/return".to_string(),
        installments: None,
        customer_id: None,
        external_reference: Some("order-123".to_string()),
    };
    let result = client.create_reader_checkout("reader_123", &request).await;

    // Assert
    assert!(result.is_ok());
    let checkout = result.unwrap();
    assert_eq!(checkout.id, "checkout_123");
    assert_eq!(checkout.external_reference, Some("order-123".to_string()));
    assert_eq!(checkout.total_amount.value, 29.99);
    assert_eq!(checkout.total_amount.currency, "EUR");
    assert_eq!(checkout.status, "PENDING");
} 