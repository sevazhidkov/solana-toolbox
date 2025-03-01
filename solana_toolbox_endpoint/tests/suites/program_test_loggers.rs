use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_transaction::create_account;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerHistory;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Create a history logger
    let logger_history = ToolboxEndpointLoggerHistory::new();
    endpoint.add_logger(Box::new(logger_history.clone()));
    // Prepare a payer
    let payer = Keypair::new();
    endpoint.request_airdrop(&payer.pubkey(), 2_000_000_000).await.unwrap();
    // Unique wallet
    let destination = Keypair::new();
    // Send a simple transfer instruction
    endpoint
        .process_system_transfer(
            &payer,
            &payer,
            &destination.pubkey(),
            1_000_000_000,
        )
        .await
        .unwrap();
    // Read account and check result
    assert_eq!(
        2_000_000_000 // Original payer airdrop
            - 1_000_000_000 // Transfered lamports
            - 5_000, // Transaction fees
        endpoint.get_account_lamports(&payer.pubkey()).await.unwrap().unwrap()
    );
    // Create a dummy mint
    let mint = Keypair::new();
    endpoint
        .process_spl_token_mint_init(
            &payer,
            &mint,
            &destination.pubkey(),
            None,
            6,
        )
        .await
        .unwrap();
    // Dummy check that it was created properly
    assert_eq!(
        6,
        endpoint
            .get_spl_token_mint(&mint.pubkey())
            .await
            .unwrap()
            .unwrap()
            .decimals,
    );
    // Custom manual TX printing (no execution)
    let (decompiled_payer, decompiled_instructions) =
        ToolboxEndpoint::decompile_transaction(&create_account(
            &payer,
            &Keypair::new(),
            endpoint.get_latest_blockhash().await.unwrap(),
            42_000_000,
            420,
            &Pubkey::new_from_array([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            ]),
        ))
        .unwrap();
    assert_eq!(decompiled_payer, payer.pubkey());
    ToolboxEndpoint::print_instructions(&decompiled_instructions);
    // Check the content of the logger's buffer history
    let processed = logger_history.get_processed();
    assert_eq!(3, processed.len());
    // First the payer's airdrop
    let exec0 = &processed[0].1;
    assert_eq!(1, exec0.instructions.len());
    assert_eq!(
        ToolboxEndpoint::SYSTEM_PROGRAM_ID,
        exec0.instructions[0].program_id
    );
    // Then the simple transfer IX happened (system program)
    let exec1 = &processed[1].1;
    assert_eq!(1, exec1.instructions.len());
    assert_eq!(
        ToolboxEndpoint::SYSTEM_PROGRAM_ID,
        exec1.instructions[0].program_id
    );
    // Then the create+init of the mint happened (2 IXs, 2 signers)
    let exec2 = &processed[2].1;
    assert_eq!(2, exec2.instructions.len());
    assert_eq!(
        ToolboxEndpoint::SYSTEM_PROGRAM_ID,
        exec2.instructions[0].program_id
    );
    assert_eq!(
        ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
        exec2.instructions[1].program_id
    );
}
