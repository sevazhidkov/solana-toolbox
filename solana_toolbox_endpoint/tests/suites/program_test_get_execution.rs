use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use solana_sdk::transaction::TransactionError;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Make a payer
    let payer = Keypair::new();
    endpoint.request_airdrop(&payer.pubkey(), 2_000_000_000).await.unwrap();
    // Run an instruction that should succeed
    let account_success = Keypair::new();
    let program_success = Pubkey::new_unique();
    let instruction_success = create_account(
        &payer.pubkey(),
        &account_success.pubkey(),
        100_000_000,
        42,
        &program_success,
    );
    let processed_success = endpoint
        .process_instruction_with_signers(
            &payer,
            instruction_success.clone(),
            &[&account_success],
        )
        .await
        .unwrap();
    // Check that we get the expected failure
    let execution_success =
        endpoint.get_execution(&processed_success.0).await.unwrap();
    assert_eq!(execution_success, processed_success.1);
    assert_eq!(execution_success.payer, payer.pubkey());
    assert_eq!(execution_success.instructions, vec![instruction_success]);
    assert_eq!(execution_success.slot, 1);
    assert_eq!(execution_success.error, None);
    assert_eq!(
        execution_success.logs,
        Some(vec![
            "Program 11111111111111111111111111111111 invoke [1]".to_string(),
            "Program 11111111111111111111111111111111 success".to_string(),
        ])
    );
    assert_eq!(execution_success.return_data, None);
    assert_eq!(execution_success.units_consumed, Some(150));
    // Run an instruction that should fail
    let account_failure = Keypair::new();
    let program_failure = Pubkey::new_unique();
    let instruction_failure = create_account(
        &payer.pubkey(),
        &account_failure.pubkey(),
        10_000_000_000,
        42,
        &program_failure,
    );
    let processed_failure = endpoint
        .process_instructions_with_options(
            &payer,
            &[instruction_failure.clone()],
            &[&account_failure],
            None,
            None,
            &[],
            true, // skip preflight to allow failed signature
        )
        .await
        .unwrap();
    // Check that we get the expected failure
    let execution_failure =
        endpoint.get_execution(&processed_failure.0).await.unwrap();
    assert_eq!(execution_failure, processed_failure.1);
    assert_eq!(execution_failure.payer, payer.pubkey());
    assert_eq!(execution_failure.instructions, vec![instruction_failure]);
    assert_eq!(execution_failure.slot, 1);
    assert_eq!(
        execution_failure.error,
        Some(TransactionError::InstructionError(
            0,
            InstructionError::Custom(1)
        ))
    );
    assert_eq!(
        execution_failure.logs,
        Some(vec![
            "Program 11111111111111111111111111111111 invoke [1]".to_string(),
            "Transfer: insufficient lamports 1899980000, need 10000000000".to_string(),
            "Program 11111111111111111111111111111111 failed: custom program error: 0x1".to_string(),
        ])
    );
}
