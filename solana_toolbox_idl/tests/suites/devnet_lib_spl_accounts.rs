use std::collections::HashMap;

use serde_json::json;
use solana_sdk::pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;
use solana_toolbox_idl::ToolboxIdlResolver;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Prepare known accounts available on devnet
    let user = pubkey!("Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9");
    let dummy_mint = pubkey!("EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3");
    let user_dummy =
        ToolboxEndpoint::find_spl_associated_token_account(&user, &dummy_mint);
    // We'll use a resolver
    let mut idl_resolver = ToolboxIdlResolver::new();
    // Check that we can resolve ATA with just the IDL
    let create_ata_instruction_addresses = idl_resolver
        .resolve_instruction_addresses(
            &mut endpoint,
            &ToolboxEndpoint::SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
            "Create",
            &HashMap::from_iter([
                ("wallet".to_string(), user),
                ("mint".to_string(), dummy_mint),
            ]),
            &json!(null),
        )
        .await
        .unwrap();
    assert_eq!(
        *create_ata_instruction_addresses.get("ata").unwrap(),
        user_dummy,
    );
    // Check the state of a system account
    let user_details = idl_resolver
        .resolve_account_details(&mut endpoint, &user)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user_details.0.name, "Account");
    assert_eq!(user_details.1, json!(null));
    // Check the state of the dummy mint
    let dummy_mint_details = idl_resolver
        .resolve_account_details(&mut endpoint, &dummy_mint)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(dummy_mint_details.0.name, "Mint");
    assert_eq!(
        dummy_mint_details.1,
        json!({
            "mint_authority": "7poxwHXi62Cwa57xdrpfoW2bUF7s8iXm1CU4jJqYPhu",
            "supply": 1000000000000000u64,
            "decimals": 9,
            "is_initialized": true,
            "freeze_authority": null,
        })
    );
    // Check the state of the dummy ATA
    let user_dummy_details = idl_resolver
        .resolve_account_details(&mut endpoint, &user_dummy)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user_dummy_details.0.name, "Account");
    assert_eq!(
        user_dummy_details.1,
        json!({
            "mint": dummy_mint.to_string(),
            "owner": user.to_string(),
            "amount": 996906108000000u64,
            "delegate": null,
            "state": "Initialized",
            "is_native": null,
            "delegated_amount": 0,
            "close_authority": null,
        })
    );
}
