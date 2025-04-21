use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::transfer;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Amounts
    let airdrop_lamports = 2_000_000_000;
    let transfer_lamports = 1_000_000_000;
    let paid_compute_units = 555_555;
    let micro_lamport_price_per_unit = 4_200_000;
    // Prepare a payer
    let payer = Keypair::new();
    endpoint
        .request_airdrop(&payer.pubkey(), airdrop_lamports)
        .await
        .unwrap();
    // Unique wallet
    let destination = Keypair::new().pubkey();
    // Send a custom transaction with compute budget customized
    let (_, execution) = endpoint
        .process_instructions_with_options(
            &payer,
            &ToolboxEndpoint::generate_instructions_with_compute_budget(
                &[transfer(&payer.pubkey(), &destination, transfer_lamports)],
                Some(paid_compute_units),
                Some(micro_lamport_price_per_unit),
            ),
            &[&payer],
            &[],
            true,
        )
        .await
        .unwrap();
    // Check the execution result
    assert_eq!(Some(450), execution.units_consumed);
    // Check where the lamports are
    assert_eq!(
        transfer_lamports,
        endpoint
            .get_account_lamports(&destination)
            .await
            .unwrap()
            .unwrap()
    );
    // Check the payer's lamport balance
    assert_eq!(
        airdrop_lamports
            - transfer_lamports
            - ToolboxEndpoint::LAMPORTS_PER_SIGNATURE
            - (u64::from(paid_compute_units) * micro_lamport_price_per_unit
                / 1_000_000),
        endpoint
            .get_account_lamports(&payer.pubkey())
            .await
            .unwrap()
            .unwrap()
    );
}
