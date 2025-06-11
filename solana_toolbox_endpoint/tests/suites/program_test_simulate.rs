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
use spl_token::instruction::ui_amount_to_amount;

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
    // Simulate an instruction that should succeed
    let account_success = Keypair::new();
    let instruction_success = create_account(
        &payer.pubkey(),
        &account_success.pubkey(),
        100_000_000,
        42,
        &Pubkey::new_unique(),
    );
    let simulation_success = endpoint
        .simulate_instruction_with_signers(
            &payer,
            instruction_success.clone(),
            &[&account_success],
        )
        .await
        .unwrap();
    assert_eq!(
        simulation_success,
        ToolboxEndpointExecution {
            processed_time: None,
            slot: 1,
            payer: payer.pubkey(),
            instructions: vec![instruction_success],
            error: None,
            steps: Some(vec![ToolboxEndpointExecutionStep::Call(
                ToolboxEndpointExecutionStepCall {
                    program_id: ToolboxEndpoint::SYSTEM_PROGRAM_ID,
                    steps: vec![],
                    consumed: None,
                    returns: None,
                    failure: None
                }
            )]),
            logs: Some(vec![
                "Program 11111111111111111111111111111111 invoke [1]"
                    .to_string(),
                "Program 11111111111111111111111111111111 success".to_string(),
            ]),
            units_consumed: Some(150),
        },
    );
    // Simulate an instruction that should fail
    let account_failure = Keypair::new();
    let instruction_failure = create_account(
        &payer.pubkey(),
        &account_failure.pubkey(),
        10_000_000_000,
        42,
        &Pubkey::new_unique(),
    );
    let simulation_failure = endpoint
        .simulate_instruction_with_signers(
            &payer,
            instruction_failure.clone(),
            &[&account_failure],
        )
        .await
        .unwrap();
    assert_eq!(
        simulation_failure,
        ToolboxEndpointExecution {
            processed_time: None,
            slot: 1,
            payer: payer.pubkey(),
            instructions: vec![instruction_failure],
            error: Some(TransactionError::InstructionError(
                0,
                InstructionError::Custom(1)
            )),
            steps: Some(vec![
                ToolboxEndpointExecutionStep::Call(
                    ToolboxEndpointExecutionStepCall {
                        program_id: ToolboxEndpoint::SYSTEM_PROGRAM_ID,
                        steps: vec![
                            ToolboxEndpointExecutionStep::Unknown(
                                "Transfer: insufficient lamports 1999990000, need 10000000000".to_string()
                            ),
                        ],
                        consumed: None,
                        returns: None,
                        failure: Some("custom program error: 0x1".to_string())
                    }
                )
            ]),
            logs: Some(vec![
                "Program 11111111111111111111111111111111 invoke [1]".to_string(),
                "Transfer: insufficient lamports 1999990000, need 10000000000".to_string(),
                "Program 11111111111111111111111111111111 failed: custom program error: 0x1".to_string(),
            ]),
            units_consumed: Some(150),
        },
    );
    // Simulate an intreuction with return data
    let mint = endpoint
        .process_spl_token_mint_new(&payer, &payer.pubkey(), None, 6)
        .await
        .unwrap();
    let instruction_returned = ui_amount_to_amount(
        &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
        &mint,
        "12.34",
    )
    .unwrap();
    let simulation_returned = endpoint
        .simulate_instruction(&payer, instruction_returned.clone())
        .await
        .unwrap();
    assert_eq!(
        simulation_returned,
        ToolboxEndpointExecution {
            processed_time: None,
            slot: 1,
            payer: payer.pubkey(),
            instructions: vec![instruction_returned],
            error: None,
            steps: Some(vec![
                ToolboxEndpointExecutionStep::Call(
                    ToolboxEndpointExecutionStepCall {
                        program_id: ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
                        steps: vec![
                            ToolboxEndpointExecutionStep::Log(
                                "Instruction: UiAmountToAmount".to_string()
                            )
                        ],
                        consumed: Some((3034, 200000)),
                        returns: Some(12_340_000u64.to_le_bytes().to_vec()),
                        failure: None
                    }
                )
            ]),
            logs: Some(vec![
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]".to_string(),
                "Program log: Instruction: UiAmountToAmount".to_string(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3034 of 200000 compute units".to_string(),
                "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA IEu8AAAAAAA=".to_string(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
            ]),
            units_consumed: Some(3034),
        },
    );
}
