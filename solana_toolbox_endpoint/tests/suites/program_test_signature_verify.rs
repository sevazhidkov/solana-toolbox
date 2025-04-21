use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction::create_account;
use solana_toolbox_endpoint::ToolboxEndpoint;

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
    // Dummy key
    let account = Keypair::new();
    // Generate an instruction that requires a signature
    let instruction = create_account(
        &payer.pubkey(),
        &account.pubkey(),
        100_000_000,
        42,
        &Pubkey::new_unique(),
    );
    // Generate a transaction with missing signatures and check its behavior on the endpoint
    let transaction = ToolboxEndpoint::compile_transaction(
        &payer,
        &[instruction.clone()],
        &[],
        endpoint.get_latest_blockhash().await.unwrap(),
    )
    .unwrap();
    endpoint
        .simulate_transaction(transaction.clone(), false)
        .await
        .unwrap();
    endpoint
        .simulate_transaction(transaction.clone(), true)
        .await
        .unwrap_err();
    endpoint
        .process_transaction(transaction.clone(), false)
        .await
        .unwrap_err();
    endpoint
        .process_transaction(transaction.clone(), true)
        .await
        .unwrap_err();
}
