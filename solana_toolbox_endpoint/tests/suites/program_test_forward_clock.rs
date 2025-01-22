use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint =
        ToolboxEndpoint::new_program_test_with_builtin_programs(&[]).await;
    // Read the initial clock sysvar
    let clock_01 = endpoint.get_sysvar_clock().await.unwrap();
    // Forward by some timestamp duration
    endpoint.forward_clock_unix_timestamp(42_000).await.unwrap();
    // Read the clock sysvar and check that it was updated properly
    let clock_02 = endpoint.get_sysvar_clock().await.unwrap();
    // Assumes 500ms per slot
    assert_eq!(clock_01.unix_timestamp + 42_000, clock_02.unix_timestamp);
    assert_eq!(clock_01.slot + (42_000 * 1_000 / 500), clock_02.slot);
    assert_eq!(clock_01.epoch, clock_02.epoch);
    // Forward by some slot amount
    endpoint.forward_clock_slot(7_000).await.unwrap();
    // Read the clock sysvar and check that it was updated properly
    let clock_03 = endpoint.get_sysvar_clock().await.unwrap();
    // Assumes 500ms per slot
    assert_eq!(
        clock_02.unix_timestamp + (7_000 * 500 / 1_000),
        clock_03.unix_timestamp
    );
    assert_eq!(clock_02.slot + 7000, clock_03.slot);
    assert_eq!(clock_02.epoch, clock_03.epoch);
    // Forward by 1 epoch
    endpoint.forward_clock_epoch(1).await.unwrap();
    // Read the clock sysvar and check that it was updated properly
    let clock_04 = endpoint.get_sysvar_clock().await.unwrap();
    // Assumes 500ms per slot
    assert_eq!(
        clock_03.unix_timestamp + 432_000 * 500 / 1_000,
        clock_04.unix_timestamp
    );
    assert_eq!(clock_03.slot + 432_000, clock_04.slot);
    assert_eq!(clock_03.epoch + 1, clock_04.epoch);
}
