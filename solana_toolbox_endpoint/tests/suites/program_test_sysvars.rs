use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint =
        ToolboxEndpoint::new_program_test_with_builtin_programs(&[]).await;
    // Read the clock sysvar and check the default values
    let clock = endpoint.get_sysvar_clock().await.unwrap();
    assert_eq!(1, clock.slot);
    assert_eq!(0, clock.epoch);
    assert_eq!(1, clock.leader_schedule_epoch);
    // Read the rent sysvar and check the default values
    let rent = endpoint.get_sysvar_rent().await.unwrap();
    assert_eq!(3480, rent.lamports_per_byte_year);
    assert_eq!(2.0, rent.exemption_threshold);
    assert_eq!(890880, rent.minimum_balance(0));
    assert_eq!(
        128, // It costs 128 bytes worth of lamports for account to exist empty
        ((rent.minimum_balance(0) / rent.lamports_per_byte_year) as f64
            / rent.exemption_threshold) as u64
    );
}
