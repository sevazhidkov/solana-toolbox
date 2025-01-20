use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use solana_sdk::system_transaction::create_account;
use solana_sdk::sysvar::rent;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerBuffer;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerTransaction;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint =
        ToolboxEndpoint::new_program_test_with_builtin_programs(&[]).await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrint::default()));
    // Create a logger buffer
    let logger_buffer = ToolboxEndpointLoggerBuffer::new();
    endpoint.add_logger(Box::new(logger_buffer.clone()));
    // Prepare a payer
    let payer = Keypair::new();
    endpoint.process_airdrop(&payer.pubkey(), 2_000_000_000).await.unwrap();
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
    ToolboxEndpointLoggerPrint::print_transaction(
        &ToolboxEndpointLoggerTransaction::from(&create_account(
            &payer,
            &Keypair::new(),
            endpoint.get_latest_blockhash().await.unwrap(),
            42_000_000,
            420,
            &Pubkey::new_from_array([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
                18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
            ]),
        )),
    );
    // Check the content of the logger's buffer history
    let transactions = logger_buffer.get_transactions();
    let accounts = logger_buffer.get_accounts();
    // Check the history vectors
    assert_eq!(2, transactions.len());
    assert_eq!(3, accounts.len());
    // First the simple transfer IX happened (system program)
    let tx0 = &transactions[0];
    assert_eq!(0, tx0.sequencing);
    assert_eq!(1, tx0.transaction.signers.len());
    assert_eq!(1, tx0.transaction.instructions.len());
    assert_eq!(system_program::ID, tx0.transaction.instructions[0].program_id);
    // Then we fetched the payers lamports
    let acc0 = &accounts[0];
    assert_eq!(1, acc0.sequencing);
    assert_eq!(payer.pubkey(), acc0.address);
    // Then fetched the rent as part of the mint create IX generation
    let acc1 = &accounts[1];
    assert_eq!(2, acc1.sequencing);
    assert_eq!(rent::ID, acc1.address);
    // Then the create+init of the mint happened (2 IXs, 2 signers)
    let tx1 = &transactions[1];
    assert_eq!(3, tx1.sequencing);
    assert_eq!(2, tx1.transaction.signers.len());
    assert_eq!(2, tx1.transaction.instructions.len());
    assert_eq!(system_program::ID, tx1.transaction.instructions[0].program_id);
    assert_eq!(spl_token::ID, tx1.transaction.instructions[1].program_id);
    // Then we fetched the created mint account
    let acc2 = &accounts[2];
    assert_eq!(4, acc2.sequencing);
    assert_eq!(mint.pubkey(), acc2.address);
}
