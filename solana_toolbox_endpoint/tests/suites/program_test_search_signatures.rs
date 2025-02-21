use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Initialize the endpoint
    let mut endpoint = ToolboxEndpoint::new_program_test().await;
    // Generate a bunch of accounts we'll use to generate an history
    let payer = Keypair::new();
    let airdrop_signature = endpoint
        .request_airdrop(&payer.pubkey(), 1_000_000_000_000)
        .await
        .unwrap();
    let mut users = vec![];
    for _ in 0..10 {
        users.push(Keypair::new());
    }
    let receiver = Pubkey::new_unique();
    // Fund each user with a base amount and save the history
    let mut fundings_signatures = vec![];
    for user in &users {
        fundings_signatures.push(
            endpoint
                .process_system_transfer(
                    &payer,
                    &payer,
                    &user.pubkey(),
                    1_000_000_000,
                )
                .await
                .unwrap(),
        );
    }
    // Generate a dummy history of transfers and save the history
    let mut transfers_signatures = vec![];
    for (idx, user) in users.iter().enumerate() {
        transfers_signatures.push(
            endpoint
                .process_system_transfer(
                    user,
                    user,
                    &receiver,
                    u64::try_from(idx).unwrap(),
                )
                .await
                .unwrap(),
        );
    }
    // Check that every user has the correct signatures
    for idx in 0..10 {
        let search_user = endpoint
            .search_signatures(&users[idx].pubkey(), None, None, usize::MAX)
            .await
            .unwrap();
        assert_eq!(search_user.len(), 2);
        assert_eq!(search_user[0], transfers_signatures[idx]);
        assert_eq!(search_user[1], fundings_signatures[idx]);
    }
    // After that we will be comparing signatures with rewinding history slices
    fundings_signatures.reverse();
    transfers_signatures.reverse();
    // Check that the payer has the proper signatures
    let search_payer = endpoint
        .search_signatures(&payer.pubkey(), None, None, usize::MAX)
        .await
        .unwrap();
    assert_eq!(search_payer.len(), 11);
    assert_eq!(search_payer[..10], fundings_signatures[..]);
    assert_eq!(search_payer[10], airdrop_signature);
    // Check that every receiver has all the transfers
    let search_receiver = endpoint
        .search_signatures(&receiver, None, None, usize::MAX)
        .await
        .unwrap();
    assert_eq!(search_receiver, transfers_signatures);
    // Check the signatures on the system program
    let search_system_unfiltered = endpoint
        .search_signatures(
            &ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            None,
            None,
            usize::MAX,
        )
        .await
        .unwrap();
    assert_eq!(search_system_unfiltered.len(), 21);
    assert_eq!(search_system_unfiltered[..10], transfers_signatures[..]);
    assert_eq!(search_system_unfiltered[10..20], fundings_signatures[..]);
    assert_eq!(search_system_unfiltered[20], airdrop_signature);
    // Check the signatures on the system program with filter
    let search_system_filtered = endpoint
        .search_signatures(
            &ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            Some(transfers_signatures[6]),
            Some(fundings_signatures[2]),
            usize::MAX,
        )
        .await
        .unwrap();
    assert_eq!(search_system_filtered.len(), 6);
    assert_eq!(search_system_filtered[..3], transfers_signatures[7..]);
    assert_eq!(search_system_filtered[3..], fundings_signatures[..3]);
    // Search from before an invalid signature (must return nothing)
    let search_before_invalid = endpoint
        .search_signatures(
            &ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            Some(Signature::new_unique()),
            None,
            100,
        )
        .await
        .unwrap();
    assert_eq!(search_before_invalid.len(), 0);
    // Search until an invalid signature (must return everything)
    let search_until_invalid = endpoint
        .search_signatures(
            &ToolboxEndpoint::SYSTEM_PROGRAM_ID,
            None,
            Some(Signature::new_unique()),
            usize::MAX,
        )
        .await
        .unwrap();
    assert_eq!(search_until_invalid, search_system_unfiltered);
    // Search with a limit
    let search_system_limited = endpoint
        .search_signatures(&ToolboxEndpoint::SYSTEM_PROGRAM_ID, None, None, 13)
        .await
        .unwrap();
    assert_eq!(search_system_limited.len(), 13);
    assert_eq!(search_system_limited[..], search_system_unfiltered[..13]);
}
