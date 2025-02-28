use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Fetch the clock (and wait a bit for it to be refreshed later)
    let clock = endpoint.get_sysvar_clock().await.unwrap();
    endpoint.forward_clock_slot(1).await.unwrap();
    // Fetch and check the rent
    let rent = endpoint.get_sysvar_rent().await.unwrap();
    assert_eq!(3480, rent.lamports_per_byte_year);
    assert_eq!(2.0, rent.exemption_threshold);
    assert_eq!(890880, rent.minimum_balance(0));
    assert_eq!(
        128, // It costs 128 bytes worth of lamports for account to exist empty
        ((rent.minimum_balance(0) / rent.lamports_per_byte_year) as f64
            / rent.exemption_threshold) as u64
    );
    // Fetch and check the slot_hashes
    let slot_hashes = endpoint.get_sysvar_slot_hashes().await.unwrap();
    assert_eq!(slot_hashes.len(), 512);
    // Check that the clock's slot is within the recent slots
    assert!(slot_hashes
        .iter()
        .map(|slot_hash| slot_hash.0)
        .collect::<Vec<_>>()
        .contains(&clock.slot));
}
