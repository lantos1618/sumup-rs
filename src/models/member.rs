use super::enums::MembershipStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// User object in member response (per OpenAPI spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberUser {
    /// User's unique identifier (UUID)
    pub id: String,
}

/// Merchant member (per OpenAPI MerchantMember schema)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    /// Unique identifier for the member
    pub id: String,
    /// Member's email address
    pub email: String,
    /// Assigned roles
    pub roles: Vec<String>,
    /// Membership status
    pub status: MembershipStatus,
    /// User information
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<MemberUser>,
}

/// Request to create a new merchant member (per OpenAPI spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMemberRequest {
    /// Email address for the member (required)
    pub email: String,
    /// Roles to assign (required)
    pub roles: Vec<String>,
    /// Whether this is a managed user (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_managed_user: Option<bool>,
    /// Password for managed user (min 8 chars, required if is_managed_user)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Nickname for managed user (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Custom metadata (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Custom attributes (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
}

impl CreateMemberRequest {
    pub fn new(email: impl Into<String>, roles: Vec<String>) -> Self {
        Self {
            email: email.into(),
            roles,
            is_managed_user: None,
            password: None,
            nickname: None,
            metadata: None,
            attributes: None,
        }
    }

    pub fn managed_user(mut self, password: impl Into<String>) -> Self {
        self.is_managed_user = Some(true);
        self.password = Some(password.into());
        self
    }

    pub fn nickname(mut self, nickname: impl Into<String>) -> Self {
        self.nickname = Some(nickname.into());
        self
    }
}

/// Request to update a merchant member (per OpenAPI spec)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateMemberRequest {
    /// Updated roles (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

impl UpdateMemberRequest {
    pub fn roles(mut self, roles: Vec<String>) -> Self {
        self.roles = Some(roles);
        self
    }
}

/// Response for listing members (per OpenAPI spec - uses "items")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberListResponse {
    pub items: Vec<Member>,
    #[serde(default)]
    pub total_count: Option<i32>,
}
