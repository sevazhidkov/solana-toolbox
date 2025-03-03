use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::SeedDerivable;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::transfer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Prepare a payer
    let payer = Keypair::from_seed(
        b"This is a dummy devnet payer for address lookup table testing",
    ) // 8sQEYJA7f5k3LrTDDkRDj46tWayc1fAdhurh61BtfUxF
    .unwrap();
    // Compute minimum rent amount
    let rent = endpoint.get_sysvar_rent().await.unwrap();
    let minimum_lamports = rent.minimum_balance(0);
    // Create users addresses
    let mut users = vec![];
    for _ in 0..50 {
        users.push(Pubkey::new_unique());
    }
    // Create a lookup table with the users
    let address_lookup_table_authority = Keypair::new();
    let address_lookup_table = endpoint
        .process_address_lookup_table_new(
            &payer,
            &address_lookup_table_authority,
            &users,
        )
        .await
        .unwrap();
    // Fetch the addresses we just uploaded
    let address_lookup_table_addresses = endpoint
        .get_address_lookup_table_addresses(&address_lookup_table)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(address_lookup_table_addresses, users);
    let resolved_address_lookup_tables = endpoint
        .resolve_address_lookup_tables(&[address_lookup_table])
        .await
        .unwrap()
        .clone();
    assert_eq!(
        resolved_address_lookup_tables,
        vec![(address_lookup_table, address_lookup_table_addresses)]
    );
    // Create a very large transaction with a lot of instructions
    let mut instructions = vec![];
    for user in &users {
        instructions.push(transfer(&payer.pubkey(), user, minimum_lamports));
    }
    let versioned_transaction = ToolboxEndpoint::compile_versioned_transaction(
        &payer,
        &instructions,
        &[],
        &resolved_address_lookup_tables,
        endpoint.get_latest_blockhash().await.unwrap(),
    )
    .unwrap();
    // Check that the transaction was successful
    let signature = endpoint
        .process_versioned_transaction(versioned_transaction.clone(), false)
        .await
        .unwrap();
    let execution = endpoint.get_execution(&signature).await.unwrap();
    assert_eq!(execution.payer, payer.pubkey());
    assert_eq!(execution.instructions, instructions);
    assert_eq!(execution.error, None);
    assert_eq!(
        execution.logs,
        Some({
            let mut expected_logs = vec![];
            for _ in users {
                expected_logs.push(
                    "Program 11111111111111111111111111111111 invoke [1]"
                        .to_string(),
                );
                expected_logs.push(
                    "Program 11111111111111111111111111111111 success"
                        .to_string(),
                );
            }
            expected_logs
        })
    );
    assert_eq!(execution.return_data, None);
    assert_eq!(execution.units_consumed, Some(7500));
}
