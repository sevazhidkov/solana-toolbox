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
    let mut payer_signatures = endpoint
        .search_signatures(&payer.pubkey(), None, None, usize::MAX)
        .await
        .unwrap();
    payer_signatures.reverse();
    assert_eq!(payer_signatures.len(), 11);
    assert_eq!(payer_signatures[0], airdrop_signature);
    assert_eq!(payer_signatures[1..11], fundings_signatures[..]);
    // Check the signatures on the system program
    let mut system_unfiltered_signatures = endpoint
        .search_signatures(&system_program::ID, None, None, usize::MAX)
        .await
        .unwrap();
    system_unfiltered_signatures.reverse();
    assert_eq!(system_unfiltered_signatures.len(), 21);
    assert_eq!(system_unfiltered_signatures[0], airdrop_signature);
    assert_eq!(system_unfiltered_signatures[1..11], fundings_signatures[..]);
    assert_eq!(system_unfiltered_signatures[11..], transfers_signatures[..]);
    // Check the signatures on the system program with filter
    let mut system_filtered_signatures = endpoint
        .search_signatures(
            &system_program::ID,
            Some(transfers_signatures[6]),
            Some(fundings_signatures[2]),
            usize::MAX,
        )
        .await
        .unwrap();
    system_filtered_signatures.reverse();
    assert_eq!(system_filtered_signatures.len(), 14);
    assert_eq!(system_filtered_signatures[..8], fundings_signatures[2..]);
    assert_eq!(system_filtered_signatures[8..], transfers_signatures[..6]);
}
