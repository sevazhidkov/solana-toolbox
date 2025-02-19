use std::str::FromStr;

use solana_sdk::instruction::InstructionError;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::TransactionError;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Create the endpoint pointing to devnet
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Lookup a transaction execution that already happened and succeeded
    let signature_success = Signature::from_str("2pqW2HvC2FqVr1GkSgLrPCp55THBzYWP6oMkaB6bZzaRXKYNJ2wfcBCu3M9r64SVcX3fEC5EomwxF939kn4pYXBW").unwrap();
    let execution_success =
        endpoint.get_execution(&signature_success).await.unwrap();
    // Check that the execution details are correct
    assert_eq!(execution_success.slot, 331437116);
    assert_eq!(execution_success.error, None);
    assert_eq!(execution_success.units_consumed, Some(23988));
    // Lookup a transaction execution that already happened and failed
    let signature_failure = Signature::from_str("3VBrBZQERLxdNjqLTzwx7TMQYbUr8ti4547CUK53WByooyJHJGmnkccw2pCQVv7D7Xi65S1E7mSFZETw6ECjxdmd").unwrap();
    let execution_failure =
        endpoint.get_execution(&signature_failure).await.unwrap();
    // Check that the execution details are correct
    assert_eq!(execution_failure.slot, 356222939);
    assert_eq!(
        execution_failure.error,
        Some(TransactionError::InstructionError(
            1,
            InstructionError::Custom(3012),
        )),
    );
    assert_eq!(execution_failure.units_consumed, Some(33086));
}
