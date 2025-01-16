use std::collections::HashSet;

use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrint;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn mainnet_beta_idl_anchor_0_29() {
    // Create the mainnet-beta endpoint
    let mut endpoint = ToolboxEndpoint::new_rpc_with_url_and_commitment(
        "https://api.mainnet-beta.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrint::new()));
    // Fetch the idl of an anchor program on chain
    let program_id = pubkey!("UXDReVg1YMckS2eVFh7TyhWDuDPaqF3wW8fX2NKeCGz");
    let idl = ToolboxIdl::get_for_program_id(&mut endpoint, &program_id)
        .await
        .unwrap()
        .unwrap();
    // Check the accounts in the IDL
    let mut account_names = HashSet::new();
    for account_item in idl.accounts.iter() {
        account_names.insert(account_item.0.to_owned());
    }
    assert_eq!(2, account_names.len());
    assert!(account_names.contains("ClaimAccount"));
    assert!(account_names.contains("Realm"));
    // Check the instructions in the IDL
    let mut instruction_names = HashSet::new();
    for instruction_item in idl.instructions_accounts {
        instruction_names.insert(instruction_item.0.to_owned());
    }
    assert_eq!(9, instruction_names.len());
    assert!(instruction_names.contains("initializeRealm"));
    assert!(instruction_names.contains("convertUxpToUct"));
    assert!(instruction_names.contains("redeemPhaseOne"));
    assert!(instruction_names.contains("redeemPhaseTwo"));
}
