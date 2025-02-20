use std::collections::HashSet;
use std::hash::RandomState;

use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
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
    // Fund each user with a base amount
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
    // Generate a dummy history of intra-user transfers
    let mut transfers_signatures = vec![];
    for (i, user) in users.iter().enumerate() {
        transfers_signatures.push(
            endpoint
                .process_system_transfer(
                    user,
                    user,
                    &users[(i + 1) % users.len()].pubkey(),
                    u64::try_from(i).unwrap(),
                )
                .await
                .unwrap(),
        );
    }
    // Check that every account has the correct signatures
    for i in 0..10 {
        let signatures: HashSet<Signature, RandomState> = HashSet::from_iter(
            endpoint
                .search_signatures(&users[i].pubkey(), None, None, usize::MAX)
                .await
                .unwrap(),
        );
        assert_eq!(signatures.len(), 3);
        assert!(signatures.contains(&fundings_signatures[i]));
        assert!(signatures.contains(&transfers_signatures[i]));
        assert!(signatures.contains(&transfers_signatures[(i + 9) % 10]));
    }
    // Check that the payer has the proper signatures
    let mut search_payer = endpoint
        .search_signatures(&payer.pubkey(), None, None, usize::MAX)
        .await
        .unwrap();
    search_payer.reverse();
    assert_eq!(search_payer.len(), 11);
    assert_eq!(search_payer[0], airdrop_signature);
    assert_eq!(search_payer[1..11], fundings_signatures[..]);
    // Check the signatures on the system program
    let mut search_system_unfiltered = endpoint
        .search_signatures(&system_program::ID, None, None, usize::MAX)
        .await
        .unwrap();
    search_system_unfiltered.reverse();
    assert_eq!(search_system_unfiltered.len(), 21);
    assert_eq!(search_system_unfiltered[0], airdrop_signature);
    assert_eq!(search_system_unfiltered[1..11], fundings_signatures[..]);
    assert_eq!(search_system_unfiltered[11..], transfers_signatures[..]);
    // Check the signatures on the system program with filter
    let mut search_system_filtered = endpoint
        .search_signatures(
            &system_program::ID,
            Some(transfers_signatures[6]),
            Some(fundings_signatures[2]),
            usize::MAX,
        )
        .await
        .unwrap();
    search_system_filtered.reverse();
    assert_eq!(search_system_filtered.len(), 14);
    assert_eq!(search_system_filtered[..8], fundings_signatures[2..]);
    assert_eq!(search_system_filtered[8..], transfers_signatures[..6]);
    // Search from before an invalid signature (must return nothing)
    let search_before_invalid = endpoint
        .search_signatures(
            &system_program::ID,
            Some(Signature::new_unique()),
            None,
            100,
        )
        .await
        .unwrap();
    assert_eq!(search_before_invalid.len(), 0);
    // Search until an invalid signature (must return everything)
    let mut search_until_invalid = endpoint
        .search_signatures(
            &system_program::ID,
            None,
            Some(Signature::new_unique()),
            usize::MAX,
        )
        .await
        .unwrap();
    search_until_invalid.reverse();
    assert_eq!(search_until_invalid, search_system_unfiltered);
    // Search with a limit
    let mut search_system_limited = endpoint
        .search_signatures(&system_program::ID, None, None, 13)
        .await
        .unwrap();
    search_system_limited.reverse();
    assert_eq!(search_system_limited.len(), 13);
    assert_eq!(
        search_system_limited[..],
        search_system_unfiltered[(search_system_unfiltered.len() - 13)..]
    );
}
