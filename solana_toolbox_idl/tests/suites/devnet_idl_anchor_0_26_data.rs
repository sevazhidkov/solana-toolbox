use std::fs::read_to_string;

use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlResolver;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Choosing our program_id
    let program_id = pubkey!("crdszSnZQu7j36KfsMJ4VEmMUTJgrNYXwoPVHUANpAu");
    // Prepare our IDL resolver
    let mut idl_resolver = ToolboxIdlResolver::new();
    // Parse and load IDL from file JSON directly (since it's not available onchain)
    idl_resolver.preload_program(
        &program_id,
        ToolboxIdlProgram::try_parse_from_str(
            &read_to_string("./tests/fixtures/idl_anchor_0_26.json").unwrap(),
        )
        .unwrap()
        .into(),
    );
    // Read the global market state content using the IDL
    let global_market_state =
        Pubkey::find_program_address(&[b"credix-marketplace"], &program_id).0;
    let global_market_state_details = idl_resolver
        .resolve_account_details(&mut endpoint, &global_market_state)
        .await
        .unwrap()
        .unwrap();
    assert_eq!("GlobalMarketState", global_market_state_details.0.name);
    assert_eq!(
        "credix-marketplace",
        global_market_state_details
            .1
            .get("seed")
            .unwrap()
            .as_str()
            .unwrap()
    );
    // Read the program state content using the IDL
    let program_state =
        Pubkey::find_program_address(&[b"program-state"], &program_id).0;
    let program_state_details = idl_resolver
        .resolve_account_details(&mut endpoint, &program_state)
        .await
        .unwrap()
        .unwrap();
    assert_eq!("ProgramState", program_state_details.0.name);
    assert_eq!(
        "Ej5zJzej7rrUoDngsJ3jcpfuvfVyWpcDcK7uv9cE2LdL",
        program_state_details
            .1
            .get("credix_multisig_key")
            .unwrap()
            .as_str()
            .unwrap()
    );
    // Read the market admins content using the IDL
    let market_admins = Pubkey::find_program_address(
        &[global_market_state.as_ref(), b"admins"],
        &program_id,
    )
    .0;
    let market_admins_details = idl_resolver
        .resolve_account_details(&mut endpoint, &market_admins)
        .await
        .unwrap()
        .unwrap();
    assert_eq!("MarketAdmins", market_admins_details.0.name);
    assert_eq!(
        "Ej5zJzej7rrUoDngsJ3jcpfuvfVyWpcDcK7uv9cE2LdL",
        market_admins_details
            .1
            .get("multisig")
            .unwrap()
            .as_str()
            .unwrap()
    );
}
