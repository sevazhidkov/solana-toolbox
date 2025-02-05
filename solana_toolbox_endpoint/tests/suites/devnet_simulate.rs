use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::SeedDerivable;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use solana_sdk::system_program;
use solana_sdk::transaction::TransactionError;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointSimulation;
use spl_token::instruction::ui_amount_to_amount;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Make a payer
    let payer =
        Keypair::from_seed(b"This is a dummy devnet payer used for simulation")
            .unwrap();
    // Simulate an instruction that should succeed
    let simulation_success = endpoint
        .simulate_instruction(
            create_account(
                &payer.pubkey(),
                &Keypair::new().pubkey(),
                100_000_000,
                42,
                &system_program::ID,
            ),
            &payer,
        )
        .await
        .unwrap();
    assert_eq!(
        ToolboxEndpointSimulation {
            err: None,
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
    let simulation_failure = endpoint
        .simulate_instruction(
            create_account(
                &payer.pubkey(),
                &Keypair::new().pubkey(),
                10_000_000_000,
                42,
                &system_program::ID,
            ),
            &payer,
        )
        .await
        .unwrap();
    assert_eq!(
        ToolboxEndpointSimulation {
            err: Some(TransactionError::InstructionError(
                0,
                InstructionError::Custom(1)
            )),
            logs: Some(vec![
                "Program 11111111111111111111111111111111 invoke [1]"
                    .to_string(),
                "Transfer: insufficient lamports 199990000, need 10000000000"
                    .to_string(),
                    "Program 11111111111111111111111111111111 failed: custom program error: 0x1".to_string(),
            ]),
            return_data: None,
            units_consumed: Some(150),
        },
        simulation_failure
    );
}
