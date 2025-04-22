use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerPrinter;

#[tokio::test]
pub async fn run() {
    // Create the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Create a print logger
    endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
    // Make a payer
    let payer = Keypair::new();
    endpoint
        .request_airdrop(&payer.pubkey(), 2_000_000_000)
        .await
        .unwrap();
    // Make sure invalid transactions return an error (and not a signature)
    let user = Keypair::new();
    endpoint
        .process_system_transfer(&payer, &user, &user.pubkey(), 1_000_000_000)
        .await
        .unwrap_err();
}
