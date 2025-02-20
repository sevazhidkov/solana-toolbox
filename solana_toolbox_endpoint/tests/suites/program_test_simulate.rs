use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use solana_sdk::transaction::TransactionError;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointDataExecution;
use spl_token::instruction::ui_amount_to_amount;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Make a payer
    let payer = Keypair::new();
    endpoint.request_airdrop(&payer.pubkey(), 2_000_000_000).await.unwrap();
    // Simulate an instruction that should succeed
    let instruction = create_account(
        &payer.pubkey(),
        &Keypair::new().pubkey(),
        100_000_000,
        42,
        &Pubkey::new_unique(),
    );
    let simulation_success =
        endpoint.simulate_instruction(instruction, &payer).await.unwrap();
    assert_eq!(
        ToolboxEndpointDataExecution {
            slot: 1,
            error: None,
            logs: Some(vec![
                "Program 11111111111111111111111111111111 invoke [1]"
                    .to_string(),
                "Program 11111111111111111111111111111111 success".to_string(),
            ]),
            return_data: None,
            units_consumed: Some(150),
        },
        simulation_success
    );
    // Simulate an instruction that should fail
    let instruction = create_account(
        &payer.pubkey(),
        &Keypair::new().pubkey(),
        10_000_000_000,
        42,
        &Pubkey::new_unique(),
    );
    let simulation_failure =
        endpoint.simulate_instruction(instruction, &payer).await.unwrap();
    assert_eq!(
        ToolboxEndpointDataExecution {
            slot: 1,
            error: Some(TransactionError::InstructionError(
                0,
                InstructionError::Custom(1)
            )),
            logs: Some(vec![
                "Program 11111111111111111111111111111111 invoke [1]"
                    .to_string(),
                "Transfer: insufficient lamports 1999990000, need 10000000000"
                    .to_string(),
                    "Program 11111111111111111111111111111111 failed: custom program error: 0x1".to_string(),
            ]),
            return_data: None,
            units_consumed: Some(150),
        },
        simulation_failure
    );
    // Simulate an intreuction with return data
    let mint = endpoint
        .process_spl_token_mint_new(&payer, &payer.pubkey(), None, 6)
        .await
        .unwrap();
    let simulation_returned = endpoint
        .simulate_instruction(
            ui_amount_to_amount(&spl_token::ID, &mint, "12.34").unwrap(),
            &payer,
        )
        .await
        .unwrap();
    assert_eq!(
        ToolboxEndpointDataExecution {
            slot: 1,
            error: None,
            logs: Some(vec![
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]".to_string(),
                "Program log: Instruction: UiAmountToAmount".to_string(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3034 of 200000 compute units".to_string(),
                "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA IEu8AAAAAAA=".to_string(),
                "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
            ]),
            return_data: Some(12_340_000u64.to_le_bytes().to_vec()),
            units_consumed: Some(3034),
        },
        simulation_returned
    );
}
