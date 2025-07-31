use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipListResponse {
    pub items: Vec<Membership>,
    pub total_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Membership {
    pub id: String,
    pub resource_id: String,
    #[serde(rename = "type")]
    pub resource_type: String, // "merchant"
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: String,
    pub resource: MembershipResource,
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
