use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_endpoint::ToolboxEndpointLoggerHistory;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Add an history logger to keep track of signatures being executed
    let logger_history = ToolboxEndpointLoggerHistory::new();
    endpoint.add_logger(Box::new(logger_history.clone()));
    // Create a funded payer
    let payer = Keypair::new();
    endpoint
        .request_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await
        .unwrap();
    // Generate a bunch of accounts we'll use to generate an history
    let mut users = vec![];
    for _ in 0..10 {
        users.push(Keypair::new());
    }
    let receiver = Pubkey::new_unique();
    // Fund each user with a base amount and save the history
    for user in &users {
        endpoint
            .process_system_transfer(
                &payer,
                &payer,
                &user.pubkey(),
                1_000_000_000,
            )
            .await
            .unwrap();
    }
    // Generate a dummy history of transfers and save the history
    for (index, user) in users.iter().enumerate() {
        endpoint
            .process_system_transfer(
                user,
                user,
                &receiver,
                u64::try_from(index).unwrap() + 100_000_000,
            )
            .await
            .unwrap();
    }
    // Collect the in-order list of signatures we just generated
    let signatures_oldest_to_newest = logger_history
        .get_processed()
        .iter()
        .map(|processed| processed.0)
        .collect::<Vec<_>>();
    // Check that every user has the correct signatures
    for idx in 0..10 {
        let search_user = endpoint
            .search_signatures(&users[idx].pubkey(), usize::MAX, None, None)
            .await
            .unwrap();
        assert_eq!(search_user.len(), 2);
        assert_eq!(search_user[0], signatures_oldest_to_newest[1 + 10 + idx]);
        assert_eq!(search_user[1], signatures_oldest_to_newest[1 + idx]);
    }
    // Put the signatures in the descening order
    let signatures_newest_to_oldest = signatures_oldest_to_newest
        .into_iter()
        .rev()
        .collect::<Vec<_>>();
    // Check that the payer has the proper signatures
    let search_payer = endpoint
        .search_signatures(&payer.pubkey(), usize::MAX, None, None)
        .await
        .unwrap();
    assert_eq!(search_payer.len(), 11);
    assert_eq!(search_payer[..], signatures_newest_to_oldest[10..]);
    // Check that every receiver has all the transfers
    let search_receiver = endpoint
        .search_signatures(&receiver, usize::MAX, None, None)
        .await
        .unwrap();
    assert_eq!(search_receiver, signatures_newest_to_oldest[..10]);
    // Check the signatures on the system program
    let search_system_unfiltered = endpoint
        .search_signatures(
            &ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            usize::MAX,
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(search_system_unfiltered.len(), 21);
    assert_eq!(search_system_unfiltered, signatures_newest_to_oldest);
    // Check the signatures on the system program with a tight filter
    let search_system_filtered = endpoint
        .search_signatures(
            &ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            usize::MAX,
            Some(signatures_newest_to_oldest[6]),
            Some(signatures_newest_to_oldest[18]),
        )
        .await
        .unwrap();
    assert_eq!(search_system_filtered.len(), 12);
    assert_eq!(
        search_system_filtered[..],
        signatures_newest_to_oldest[7..19]
    );
    // Search from before an invalid signature (must return nothing)
    let search_before_invalid = endpoint
        .search_signatures(
            &ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            100,
            Some(Signature::new_unique()),
            None,
        )
        .await
        .unwrap();
    assert_eq!(search_before_invalid.len(), 0);
    // Search until an invalid signature (must return everything)
    let search_until_invalid = endpoint
        .search_signatures(
            &ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            usize::MAX,
            None,
            Some(Signature::new_unique()),
        )
        .await
        .unwrap();
    assert_eq!(search_until_invalid, search_system_unfiltered);
    // Search with a limit
    let search_system_limited = endpoint
        .search_signatures(&ToolboxEndpoint::SYSTEM_PROGRAM_ID, 13, None, None)
        .await
        .unwrap();
    assert_eq!(search_system_limited.len(), 13);
    assert_eq!(search_system_limited[..], search_system_unfiltered[..13]);
    // Search invalid order
    let search_order_invalid = endpoint
        .search_signatures(
            &ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            usize::MAX,
            Some(signatures_newest_to_oldest[18]),
            Some(signatures_newest_to_oldest[6]),
        )
        .await
        .unwrap();
    assert_eq!(search_order_invalid[..], signatures_newest_to_oldest[19..]);
}
