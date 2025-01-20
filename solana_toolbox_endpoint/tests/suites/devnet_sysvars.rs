use solana_sdk::commitment_config::CommitmentConfig;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_rpc_with_url_and_commitment(
        "https://api.devnet.solana.com",
        CommitmentConfig::confirmed(),
    );
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Fetch the sysvars
    let clock = endpoint.get_sysvar_clock().await;
    let rent = endpoint.get_sysvar_rent().await;
    // Check that the accounts are fetched properly
    assert!(clock.is_ok());
    assert!(rent.is_ok());
    let rent = rent.unwrap();
    assert_eq!(3480, rent.lamports_per_byte_year);
    assert_eq!(2.0, rent.exemption_threshold);
    assert_eq!(890880, rent.minimum_balance(0));
    assert_eq!(
        128, // It costs 128 bytes worth of lamports for account to exist empty
        ((rent.minimum_balance(0) / rent.lamports_per_byte_year) as f64
            / rent.exemption_threshold) as u64
    );
}
