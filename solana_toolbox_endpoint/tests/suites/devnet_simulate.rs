use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::SeedDerivable;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use solana_sdk::transaction::TransactionError;
use solana_toolbox_endpoint::ToolboxEndpoint;
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
                &Pubkey::new_unique(),
            ),
            &payer,
        )
        .await
        .unwrap();
    assert_eq!(simulation_success.error, None);
    assert_eq!(
        simulation_success.logs,
        Some(vec![
            "Program 11111111111111111111111111111111 invoke [1]".to_string(),
            "Program 11111111111111111111111111111111 success".to_string(),
        ])
    );
    assert_eq!(simulation_success.return_data, None);
    assert_eq!(simulation_success.units_consumed, Some(150));
    // Simulate an instruction that should fail
    let simulation_failure = endpoint
        .simulate_instruction(
            create_account(
                &payer.pubkey(),
                &Keypair::new().pubkey(),
                10_000_000_000,
                42,
                &Pubkey::new_unique(),
            ),
            &payer,
        )
        .await
        .unwrap();
    assert_eq!(
        simulation_failure.error,
        Some(TransactionError::InstructionError(
            0,
            InstructionError::Custom(1)
        ))
    );
    assert_eq!(simulation_failure.logs, Some(vec![
        "Program 11111111111111111111111111111111 invoke [1]"
            .to_string(),
        "Transfer: insufficient lamports 199990000, need 10000000000"
            .to_string(),
            "Program 11111111111111111111111111111111 failed: custom program error: 0x1".to_string(),
    ]));
    assert_eq!(simulation_failure.return_data, None);
    assert_eq!(simulation_failure.units_consumed, Some(150));
    // Simulate an intreuction with return data
    let simulation_returned = endpoint
        .simulate_instruction(
            ui_amount_to_amount(
                &ToolboxEndpoint::SPL_TOKEN_PROGRAM_ID,
                &pubkey!("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr"),
                "12.34",
            )
            .unwrap(),
            &payer,
        )
        .await
        .unwrap();
    assert_eq!(simulation_returned.error, None);
    assert_eq!(
        simulation_returned.logs,
        Some(vec![
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [1]".to_string(),
            "Program log: Instruction: UiAmountToAmount".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 3034 of 200000 compute units".to_string(),
            "Program return: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA IEu8AAAAAAA=".to_string(),
            "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success".to_string(),
    ])
    );
    assert_eq!(
        simulation_returned.return_data,
        Some(12_340_000u64.to_le_bytes().to_vec())
    );
    assert_eq!(simulation_returned.units_consumed, Some(3034));
}
