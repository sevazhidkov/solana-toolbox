use solana_sdk::commitment_config::CommitmentConfig;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrint;

#[tokio::test]
pub async fn devnet_sysvars() {
    // Create the devnet endpoint
    let mut endpoint = ToolboxEndpoint::new_rpc_with_url_and_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrint::default()));
    // Fetch the sysvars
    let clock = endpoint.get_sysvar_clock().await;
    let rent = endpoint.get_sysvar_rent().await;
    // Check that the accounts are fetched properly
    assert!(clock.is_ok());
    assert!(rent.is_ok());
    let rent = rent.unwrap();
    assert_eq!(3480, rent.lamports_per_byte_year);
    assert_eq!(2.0, rent.exemption_threshold);
}
