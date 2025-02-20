use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::TransactionError;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointDataExecution;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Make a payer
    let payer = Keypair::new();
    endpoint.request_airdrop(&payer.pubkey(), 2_000_000_000).await.unwrap();
    // Run an instruction that should succeed
    let signature_success = endpoint
        .process_system_create(
            &payer,
            &Keypair::new(),
            100_000_000,
            42,
            &Pubkey::new_unique(),
        )
        .await
        .unwrap();
    // Check that we get the expected failure
    assert_eq!(
        endpoint.get_execution(&signature_success).await.unwrap(),
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
    );
    // Run an instruction that should fail
    let signature_failure = endpoint
        .process_system_create(
            &payer,
            &Keypair::new(),
            10_000_000_000,
            42,
            &Pubkey::new_unique(),
        )
        .await
        .unwrap();
    // Check that we get the expected failure
    assert_eq!(
        endpoint.get_execution(&signature_failure).await.unwrap(),
        ToolboxEndpointDataExecution {
            slot: 1,
            error: Some(TransactionError::InstructionError(
                0,
                InstructionError::Custom(1)
            )),
            logs: Some(vec![
                "Program 11111111111111111111111111111111 invoke [1]"
                    .to_string(),
                "Transfer: insufficient lamports 1899980000, need 10000000000"
                    .to_string(),
                    "Program 11111111111111111111111111111111 failed: custom program error: 0x1".to_string(),
            ]),
            return_data: None,
            units_consumed: Some(150),
        },
    );
}
