use super::enums::MembershipStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Response for listing memberships (per OpenAPI spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipListResponse {
    pub items: Vec<Membership>,
    pub total_count: i32,
}

/// Resource type for memberships (per OpenAPI spec)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceType {
    Merchant,
    Organization,
}

/// Membership object (per OpenAPI spec)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    pub roles: Vec<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    pub status: MembershipStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<MembershipResource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipResource {
    pub id: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMembershipRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMembershipRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
