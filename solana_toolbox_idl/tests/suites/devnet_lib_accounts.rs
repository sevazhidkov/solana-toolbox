use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlService;
use solana_toolbox_idl::ToolboxIdlServiceAccountDecoded;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Prepare known accounts available on devnet
    let program_id = pubkey!("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j");
    let program_data = pubkey!("9rtcXuviJngSZTRSCXxsHyd6qaWpqWSQ56SNumXAuLJ1");
    let mint_authority = pubkey!("7poxwHXi62Cwa57xdrpfoW2bUF7s8iXm1CU4jJqYPhu");
    let user = pubkey!("Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9");
    let collateral_mint =
        pubkey!("EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3");
    let user_collateral = ToolboxEndpoint::find_spl_associated_token_account(
        &user,
        &collateral_mint,
    );
    let name_record_header =
        pubkey!("8EodedXFv8DAJ6jGTg4DVXaBVJTVL3o4T2BWwTJTTJjw");
    let name_record_owner =
        pubkey!("8aU2gq8XgzNZr8z4noV87Sx8a3EV29gmi645qQERsaTD");
    // Lookup our program
    let mut idl_service = ToolboxIdlService::new();
    let idl_program_ata = idl_service
        .resolve_program(
            &mut endpoint,
            &ToolboxEndpoint::SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
        )
        .await
        .unwrap()
        .unwrap();
    let idl_instruction_create_ata =
        idl_program_ata.instructions.get("create").unwrap();
    // Check that we can resolve ATA with just the IDL
    let create_ata_instruction_addresses = idl_service
        .resolve_instruction_addresses(
            &mut endpoint,
            idl_instruction_create_ata,
            &ToolboxEndpoint::SPL_ASSOCIATED_TOKEN_PROGRAM_ID,
            &json!(null),
            &HashMap::from_iter([
                ("wallet".to_string(), user),
                ("mint".to_string(), collateral_mint),
            ]),
        )
        .await
        .unwrap();
    assert_eq!(
        *create_ata_instruction_addresses.get("ata").unwrap(),
        user_collateral,
    );
    // Check the state of a system account
    let user_decoded = idl_service
        .get_and_decode_account(&mut endpoint, &user)
        .await
        .unwrap();
    assert_account_decoded_properly(
        user_decoded,
        "system",
        "Wallet",
        json!(null),
    );
    // Check the state of the collateral mint
    let collateral_mint_decoded = idl_service
        .get_and_decode_account(&mut endpoint, &collateral_mint)
        .await
        .unwrap();
    assert_account_decoded_properly(
        collateral_mint_decoded,
        "spl_token",
        "TokenMint",
        json!({
            "mint_authority": mint_authority.to_string(),
            "supply": 1000000000000000u64,
            "decimals": 9,
            "is_initialized": true,
            "freeze_authority": null,
        }),
    );
    // Check the state of the collateral ATA
    let user_collateral_decoded = idl_service
        .get_and_decode_account(&mut endpoint, &user_collateral)
        .await
        .unwrap();
    assert_account_decoded_properly(
        user_collateral_decoded,
        "spl_token",
        "TokenAccount",
        json!({
            "mint": collateral_mint.to_string(),
            "owner": user.to_string(),
            "amount": 996906108000000u64,
            "delegate": null,
            "state": "Initialized",
            "is_native": null,
            "delegated_amount": 0,
            "close_authority": null,
        }),
    );
    // Check the state of a known program
    let program_id_decoded = idl_service
        .get_and_decode_account(&mut endpoint, &program_id)
        .await
        .unwrap();
    assert_account_decoded_properly(
        program_id_decoded,
        "bpf_loader_upgradeable",
        "Program",
        json!({
            "program_data": program_data.to_string()
        }),
    );
    // Check the state of a known program's executable data
    let program_data_decoded = idl_service
        .get_and_decode_account(&mut endpoint, &program_data)
        .await
        .unwrap();
    assert_account_decoded_properly(
        program_data_decoded,
        "bpf_loader_upgradeable",
        "ProgramData",
        json!({
            "slot": 347133692,
            "upgrade_authority": mint_authority.to_string(),
        }),
    );
    // Check the state of a known name record header
    let name_record_header_decoded = idl_service
        .get_and_decode_account(&mut endpoint, &name_record_header)
        .await
        .unwrap();
    assert_account_decoded_properly(
        name_record_header_decoded,
        "spl_name_service",
        "NameRecordHeader",
        json!({
            "class": ToolboxEndpoint::SYSTEM_PROGRAM_ID.to_string(),
            "owner": name_record_owner.to_string(),
            "parent_name": ToolboxEndpoint::SYSTEM_PROGRAM_ID.to_string(),
        }),
    );
}

fn assert_account_decoded_properly(
    account_decoded: ToolboxIdlServiceAccountDecoded,
    program_name: &str,
    account_name: &str,
    account_state: Value,
) {
    assert_eq!(
        account_decoded.program.metadata.name,
        Some(program_name.to_string())
    );
    assert_eq!(account_decoded.account.name, account_name);
    assert_eq!(account_decoded.state, account_state);
}
