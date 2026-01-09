use sumup_rs::{CreateMemberRequest, CreateRoleRequest, Role, SumUpClient};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_create_role_success() {
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

    Mock::given(method("POST"))
        .and(path("/v0.1/merchants/merchant_123/roles"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "role_123",
            "name": "Admin",
            "membership_id": "membership_123",
            "permissions": ["read", "write"],
            "is_predefined": false,
            "created_at": expected_role.created_at.to_rfc3339(),
            "updated_at": null
        })))
        .mount(&mock_server)
        .await;

    let request = CreateRoleRequest {
        name: "Admin".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    let result = client.create_role("merchant_123", &request).await;

    assert!(result.is_ok());
    let role = result.unwrap();
    assert_eq!(role.id, "role_123");
    assert_eq!(role.name, "Admin");
}

#[tokio::test]
async fn test_create_member_success() {
    let mock_server = MockServer::start().await;
    let client = SumUpClient::with_custom_url("test_api_key".to_string(), mock_server.uri()).unwrap();

    let now = chrono::Utc::now();

    Mock::given(method("POST"))
        .and(path("/v0.1/merchants/merchant_123/members"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": "member_123",
            "membership_id": "membership_123",
            "user": {
                "id": "user_123",
                "email": "john@example.com",
                "first_name": "John",
                "last_name": "Doe",
                "created_at": now.to_rfc3339(),
                "updated_at": null
            },
            "status": "ACTIVE",
            "roles": ["role_123"],
            "permissions": ["read"],
            "created_at": now.to_rfc3339(),
            "updated_at": null
        })))
        .mount(&mock_server)
        .await;

    let request = CreateMemberRequest {
        email: "john@example.com".to_string(),
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        roles: Some(vec!["role_123".to_string()]),
    };
    let result = client.create_member("merchant_123", &request).await;

    assert!(result.is_ok());
    let member = result.unwrap();
    assert_eq!(member.id, "member_123");
    assert_eq!(member.user.email, "john@example.com");
}
