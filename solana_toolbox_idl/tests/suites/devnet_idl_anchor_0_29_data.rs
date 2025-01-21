use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;
use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_rpc_with_url_and_commitment(
        "https://api.devnet.solana.com",
        CommitmentConfig::confirmed(),
    );
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Fetch the idl of an anchor program on chain
    let program_id = pubkey!("Ee5CDFHQmdUQMEnM3dJZMiLaBuP2Wr8WBVYM7UZPPb6E");
    let idl = ToolboxIdl::get_for_program_id(&mut endpoint, &program_id)
        .await
        .unwrap()
        .unwrap();
    // Read an account using the IDL directly
    let realm_pda = Pubkey::find_program_address(&[b"realm"], &program_id);
    let realm_address = realm_pda.0;
    let realm_bump = realm_pda.1;
    let realm_data_value = idl
        .get_account_value(&mut endpoint, &realm_address)
        .await
        .unwrap()
        .unwrap();
    eprintln!("realm_data_value: {:?}", realm_data_value);
    // Check that the account was parsed properly and values matches
    assert_eq!(
        u64::from(realm_bump),
        realm_data_value.get("bump").unwrap().as_u64().unwrap()
    );
    // Related "USDC mint" account checks
    let usdc_mint_address =
        pubkey!("H7JmSvR6w6Qrp9wEbw4xGEBkbh95Jc9C4yXYYYvWmF8B");
    assert_eq!(
        usdc_mint_address.to_string(),
        realm_data_value.get("usdcMint").unwrap().as_str().unwrap(),
    );
    // Related "UCT mint" account checks
    let uct_mint_pda = Pubkey::find_program_address(
        &[b"uct_mint", &realm_address.to_bytes()],
        &program_id,
    );
    assert_eq!(
        u64::from(uct_mint_pda.1),
        realm_data_value.get("uctMintBump").unwrap().as_u64().unwrap(),
    );
    assert_eq!(
        uct_mint_pda.0.to_string(),
        realm_data_value.get("uctMint").unwrap().as_str().unwrap(),
    );
}
