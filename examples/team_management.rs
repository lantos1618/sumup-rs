use sumup_rs::{CreateMemberRequest, CreateRoleRequest, SumUpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.local").ok();
    let api_key = std::env::var("SUMUP_API_SECRET_KEY")?;

    let client = SumUpClient::new(api_key, true)?;

    #[allow(deprecated)]
    let profile = client.get_merchant_profile().await?;
    let mc = &profile.merchant_code;
    println!("Merchant: {}", mc);

    // List memberships
    let memberships = client.list_memberships().await?;
    println!("Memberships: {}", memberships.len());

    // Create role
    let role = client
        .create_role(
            mc,
            &CreateRoleRequest {
                name: "Manager".to_string(),
                permissions: vec!["read_transactions".to_string()],
            },
        )
        .await?;
    println!("Created role: {} ({})", role.name, role.id);

    // Create member
    let member = client
        .create_member(mc, &CreateMemberRequest::new("test@example.com", vec![role.id.clone()]))
        .await?;
    println!("Created member: {} ({})", member.email, member.id);

    // List roles
    let roles = client.list_roles(mc).await?;
    println!("Roles: {}", roles.roles.len());

    Ok(())
}
