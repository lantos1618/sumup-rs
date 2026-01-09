use sumup_rs::payouts::PayoutListQuery;
use sumup_rs::{CreateMemberRequest, CreateRoleRequest, SumUpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.local").ok();

    let api_key = std::env::var("SUMUP_API_SECRET_KEY")
        .expect("SUMUP_API_SECRET_KEY must be set");

    let client = SumUpClient::new(api_key, true)?;
    let profile = client.get_merchant_profile().await?;
    let merchant_code = &profile.merchant_code;

    println!("Team Management Example");
    println!("=======================");
    println!("Merchant: {} ({})\n", profile.name, merchant_code);

    // List memberships
    println!("1. Memberships:");
    let memberships = client.list_memberships().await?;
    for m in &memberships {
        println!("   - {} ({})", m.resource_id, m.id);
    }

    // Create a role
    println!("\n2. Creating role...");
    let role = client.create_role(merchant_code, &CreateRoleRequest {
        name: "Manager".to_string(),
        permissions: vec!["read_transactions".to_string(), "create_checkouts".to_string()],
    }).await?;
    println!("   Created: {} ({})", role.name, role.id);

    // Create a member
    println!("\n3. Creating member...");
    let member = client.create_member(merchant_code, &CreateMemberRequest {
        email: "test@example.com".to_string(),
        first_name: Some("Test".to_string()),
        last_name: Some("User".to_string()),
        roles: Some(vec![role.id.clone()]),
    }).await?;
    println!("   Created: {} ({})", member.user.email, member.id);

    // List roles
    println!("\n4. Roles:");
    let roles = client.list_roles(merchant_code).await?;
    for r in &roles.roles {
        println!("   - {} (predefined: {})", r.name, r.is_predefined);
    }

    // List payouts
    println!("\n5. Payouts:");
    match client.list_merchant_payouts(merchant_code, &PayoutListQuery {
        start_date: "2024-01-01".to_string(),
        end_date: "2024-12-31".to_string(),
        limit: Some(5),
        offset: None,
    }).await {
        Ok(payouts) => {
            for p in &payouts.payouts {
                println!("   - {} {} ({})", p.amount, p.currency, p.status);
            }
        }
        Err(e) => println!("   Error: {}", e),
    }

    println!("\nDone!");
    Ok(())
}
