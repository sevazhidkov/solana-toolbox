use std::time::Duration;
use std::time::SystemTime;

use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use solana_sdk::transaction::TransactionError;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointExecution;
use solana_toolbox_endpoint::ToolboxEndpointExecutionStep;
use solana_toolbox_endpoint::ToolboxEndpointExecutionStepCall;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Make a payer
    let payer = Keypair::new();
    endpoint
        .request_airdrop(&payer.pubkey(), 2_000_000_000)
        .await
        .unwrap();
    // Fetch the clock
    let clock = endpoint.get_sysvar_clock().await.unwrap();
    let clock_time = SystemTime::UNIX_EPOCH
        + Duration::from_secs(clock.unix_timestamp as u64);
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
    assert_eq!(
        execution_success,
        ToolboxEndpointExecution {
            processed_time: Some(clock_time),
            slot: 1,
            payer: payer.pubkey(),
            instructions: vec![instruction_success],
            steps: Some(vec![ToolboxEndpointExecutionStep::Call(
                ToolboxEndpointExecutionStepCall {
                    program_id: ToolboxEndpoint::SYSTEM_PROGRAM_ID,
                    steps: vec![],
                    consumed: None,
                    returns: None,
                    failure: None,
                }
            )]),
            logs: Some(vec![
                "Program 11111111111111111111111111111111 invoke [1]"
                    .to_string(),
                "Program 11111111111111111111111111111111 success".to_string(),
            ]),
            error: None,
            units_consumed: Some(150),
        }
    );
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
            &[],
            false, // skip preflight to allow transaction that will fail to execute
        )
        .await
        .unwrap();
    // Check that we get the expected failure
    let execution_failure =
        endpoint.get_execution(&processed_failure.0).await.unwrap();
    assert_eq!(execution_failure, processed_failure.1);
    assert_eq!(execution_failure, ToolboxEndpointExecution {
        processed_time: Some(clock_time),
        slot: 1,
        payer: payer.pubkey(),
        instructions: vec![instruction_failure],
        steps: Some(vec![ToolboxEndpointExecutionStep::Call(
            ToolboxEndpointExecutionStepCall {
                program_id: ToolboxEndpoint::SYSTEM_PROGRAM_ID,
                steps: vec![
                    ToolboxEndpointExecutionStep::Unknown(
                        "Transfer: insufficient lamports 1899980000, need 10000000000".to_string()
                    ),
                ],
                consumed: None,
                returns: None,
                failure: Some("custom program error: 0x1".to_string()),
            }
        )]),
        logs: Some(vec![
            "Program 11111111111111111111111111111111 invoke [1]".to_string(),
            "Transfer: insufficient lamports 1899980000, need 10000000000".to_string(),
            "Program 11111111111111111111111111111111 failed: custom program error: 0x1".to_string(),
        ]),
        error: Some(TransactionError::InstructionError(
            0,
            InstructionError::Custom(1)
        )),
        units_consumed: Some(150),
    });
}
