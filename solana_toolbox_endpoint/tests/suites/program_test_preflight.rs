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
    // Make sure invalid transactions return an error
    let user = Keypair::new();
    endpoint
        .process_system_transfer(&user, &user, &user.pubkey(), 1_000_000_000)
        .await
        .unwrap_err();
}
