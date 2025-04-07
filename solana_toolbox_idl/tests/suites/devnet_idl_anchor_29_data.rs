use serde_json::json;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;
use solana_toolbox_idl::ToolboxIdlService;

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
    let realm_decoded = ToolboxIdlService::new()
        .get_and_decode_account(&mut endpoint, &realm)
        .await
        .unwrap();
    // Check that the account was parsed properly and values matches
    assert_eq!(
        realm_decoded.program.metadata.name,
        Some("Redemption".to_string()),
    );
    assert_eq!(realm_decoded.account.name, "Realm");
    assert_eq!(realm_decoded.state.get("bump").unwrap(), &json!(realm_bump));
    // Related "USDC mint" account checks
    assert_eq!(
        realm_decoded.state.get("usdc_mint").unwrap(),
        &json!("H7JmSvR6w6Qrp9wEbw4xGEBkbh95Jc9C4yXYYYvWmF8B"),
    );
    // Related "UCT mint" account checks
    assert_eq!(
        realm_decoded.state.get("uct_mint_bump").unwrap(),
        &json!(uct_mint_bump),
    );
    assert_eq!(
        realm_decoded.state.get("uct_mint").unwrap(),
        &json!(uct_mint.to_string()),
    );
}
