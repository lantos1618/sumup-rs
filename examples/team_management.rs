use sumup_rs::{
    SumUpClient, 
    CreateMembershipRequest, 
    CreateRoleRequest, 
    CreateMemberRequest,
    CreateReaderCheckoutRequest,
    TotalAmount
};
use sumup_rs::payouts::PayoutListQuery;
use sumup_rs::receipts::ReceiptRetrieveQuery;
use sumup_rs::CreateReaderRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = std::env::var("SUMUP_API_KEY")
        .expect("Please set SUMUP_API_KEY environment variable");
    
    // Create a client (use sandbox for testing)
    let client = SumUpClient::new(api_key, true)?;
    
    println!("=== SumUp Team Management Example ===\n");
    
    // Step 1: Create a membership
    println!("1. Creating a membership...");
    let membership_request = CreateMembershipRequest {
        name: "My Business Team".to_string(),
        description: Some("Team for managing our business operations".to_string()),
    };
    
    let membership = client.create_membership(&membership_request).await?;
    println!("✅ Created membership: {} (ID: {})", membership.name, membership.id);
    
    // Step 2: Create a custom role
    println!("\n2. Creating a custom role...");
    let role_request = CreateRoleRequest {
        name: "Manager".to_string(),
        permissions: vec![
            "read_transactions".to_string(),
            "read_customers".to_string(),
            "create_checkouts".to_string(),
        ],
    };
    
    let role = client.create_role(&membership.id, &role_request).await?;
    println!("✅ Created role: {} (ID: {})", role.name, role.id);
    println!("   Permissions: {:?}", role.permissions);
    println!("   Is predefined: {}", role.is_predefined);
    
    // Step 3: Create a member and assign them the role
    println!("\n3. Creating a team member...");
    let member_request = CreateMemberRequest {
        email: "john.doe@mybusiness.com".to_string(),
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        roles: Some(vec![role.id.clone()]),
    };
    
    let member = client.create_member(&membership.id, &member_request).await?;
    println!("✅ Created member: {} {} (ID: {})", 
        member.user.first_name.as_deref().unwrap_or(""), 
        member.user.last_name.as_deref().unwrap_or(""), 
        member.id);
    println!("   Email: {}", member.user.email);
    println!("   Status: {}", member.status);
    println!("   Roles: {:?}", member.roles);
    println!("   Permissions: {:?}", member.permissions);
    
    // Step 4: List all members in the membership
    println!("\n4. Listing all members...");
    let members_response = client.list_members(&membership.id).await?;
    println!("✅ Found {} members in the membership:", members_response.members.len());
    for member in &members_response.members {
        println!("   - {} {} ({}) - Status: {}", 
            member.user.first_name.as_deref().unwrap_or(""), 
            member.user.last_name.as_deref().unwrap_or(""), 
            member.user.email, 
            member.status);
    }
    
    // Step 5: List all roles in the membership
    println!("\n5. Listing all roles...");
    let roles_response = client.list_roles(&membership.id).await?;
    println!("✅ Found {} roles in the membership:", roles_response.roles.len());
    for role in &roles_response.roles {
        println!("   - {} (ID: {}) - Predefined: {}", role.name, role.id, role.is_predefined);
        println!("     Permissions: {:?}", role.permissions);
    }
    
    // Step 6: Demonstrate payout listing (requires date range)
    println!("\n6. Listing payouts...");
    let payout_query = PayoutListQuery {
        start_date: "2023-01-01".to_string(),
        end_date: "2023-12-31".to_string(),
        limit: Some(5),
        offset: Some(0),
    };
    
    match client.list_payouts(&payout_query).await {
        Ok(payouts) => {
            println!("✅ Found {} payouts:", payouts.payouts.len());
            for payout in &payouts.payouts {
                println!("   - {}: {} {} ({})", 
                    payout.id, 
                    payout.amount, 
                    payout.currency, 
                    payout.status);
            }
        }
        Err(e) => {
            println!("⚠️  Could not list payouts: {}", e);
        }
    }
    
    // Step 7: Demonstrate reader management
    println!("\n7. Creating a reader...");
    let reader_request = CreateReaderRequest {
        serial_number: "SUMUP123456789".to_string(),
        name: Some("Main Counter Reader".to_string()),
    };
    
    match client.create_reader(&reader_request).await {
        Ok(reader) => {
            println!("✅ Created reader: {} (ID: {})", 
                reader.serial_number, reader.id);
            println!("   Status: {}", reader.status);
            
            // Step 8: Create a checkout for the reader (in-person payment)
            println!("\n8. Creating a reader checkout...");
            let checkout_request = CreateReaderCheckoutRequest {
                total_amount: TotalAmount {
                    value: 15.99,
                    currency: "EUR".to_string(),
                    minor_unit: 2,
                },
                description: "Coffee and pastry".to_string(),
                return_url: "https://mybusiness.com/return".to_string(),
                installments: None,
                customer_id: None,
                external_reference: Some("counter-sale-001".to_string()),
            };
            
            match client.create_reader_checkout(&reader.id, &checkout_request).await {
                Ok(checkout) => {
                    println!("✅ Created reader checkout: {} (ID: {})", 
                        checkout.external_reference.as_deref().unwrap_or("N/A"), 
                        checkout.id);
                    println!("   Amount: {} {}", checkout.total_amount.value, checkout.total_amount.currency);
                    println!("   Status: {}", checkout.status);
                }
                Err(e) => {
                    println!("⚠️  Could not create reader checkout: {}", e);
                }
            }
        }
        Err(e) => {
            println!("⚠️  Could not create reader: {}", e);
        }
    }
    
    // Step 9: Demonstrate receipt retrieval
    println!("\n9. Retrieving receipt details...");
    let receipt_query = ReceiptRetrieveQuery {
        mid: "your-merchant-id".to_string(),
    };
    
    // Note: This would require a valid receipt ID
    println!("ℹ️  Receipt retrieval requires a valid receipt ID");
    println!("   Use: client.retrieve_receipt(\"receipt-id\", &receipt_query).await");
    
    println!("\n=== Team Management Example Complete ===");
    println!("✅ Successfully demonstrated:");
    println!("   - Membership creation and management");
    println!("   - Role creation with custom permissions");
    println!("   - Member creation and role assignment");
    println!("   - Payout listing with date filtering");
    println!("   - Reader management and in-person payments");
    println!("   - Receipt retrieval workflow");
    
    Ok(())
} 