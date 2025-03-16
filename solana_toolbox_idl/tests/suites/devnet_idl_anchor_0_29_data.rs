use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;
use solana_toolbox_idl::ToolboxIdlResolver;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // The devnet program we'll lookup
    let program_id = pubkey!("Ee5CDFHQmdUQMEnM3dJZMiLaBuP2Wr8WBVYM7UZPPb6E");
    // Important account addresses
    let realm_pda = Pubkey::find_program_address(&[b"realm"], &program_id);
    let realm = realm_pda.0;
    let realm_bump = realm_pda.1;
    let uct_mint_pda = Pubkey::find_program_address(
        &[b"uct_mint", &realm.to_bytes()],
        &program_id,
    );
    let uct_mint = uct_mint_pda.0;
    let uct_mint_bump = uct_mint_pda.1;
    // Actually fetch our account using the auto-resolved IDL on-chain
    let realm_details = ToolboxIdlResolver::new()
        .resolve_account_details(&mut endpoint, &realm)
        .await
        .unwrap()
        .unwrap();
    // Check that the account was parsed properly and values matches
    assert_eq!("Realm", realm_details.0.name);
    assert_eq!(
        u64::from(realm_bump),
        realm_details.1.get("bump").unwrap().as_u64().unwrap()
    );
    // Related "USDC mint" account checks
    assert_eq!(
        "H7JmSvR6w6Qrp9wEbw4xGEBkbh95Jc9C4yXYYYvWmF8B",
        realm_details.1.get("usdc_mint").unwrap().as_str().unwrap(),
    );
    // Related "UCT mint" account checks
    assert_eq!(
        u64::from(uct_mint_bump),
        realm_details
            .1
            .get("uct_mint_bump")
            .unwrap()
            .as_u64()
            .unwrap(),
    );
    assert_eq!(
        uct_mint.to_string(),
        realm_details.1.get("uct_mint").unwrap().as_str().unwrap(),
    );
}
