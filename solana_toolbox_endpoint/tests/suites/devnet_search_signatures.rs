use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Create the endpoint pointing to devnet
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Tests constants
    let program_id =
        Pubkey::from_str("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j")
            .unwrap();
    let program_signature_n4 = Signature::from_str("4gqmT5jrEZ35BEkq2x1K8WHwhWVz9Z46Un5w1sddLvmx1c5fUTyzd4J389bcCsHgCBzQam4Qn5MdKuw5ydUyJ62L").unwrap();
    let program_signature_n3 = Signature::from_str("5KUaXrTAjeuHg3XPYo8ve6UJR5u5vP8cS9JDEKoG6Cq3V2gBp52QHQcfKkeHLcfDDMpEf27aRrZ5EtG8bBjHAXf5").unwrap();
    let program_signature_n2 = Signature::from_str("LWzVhua28HoamZ81JuB1EQJ8JLsLdtTTNVXUWJcUzUwqVPSu4SpQhjiUfaxhFdL2TPPcmmeN8sJKe1QSeMRiP4L").unwrap();
    let program_signature_n1 = Signature::from_str("3eHgwNJHqSHYHroGZimcCQKSWzyr3rrohRfXG3YmtpL4FAkbkZ8G4STwVBXsd3QTrURkNiUqttqfRCxuc6s7NJzP").unwrap();
    // Search all the way through the history until transaction n2
    let search_until_n2 = endpoint
        .search_signatures(
            &program_id,
            None,
            Some(program_signature_n2),
            usize::MAX,
        )
        .await
        .unwrap();
    assert!(search_until_n2.len() > 200);
    assert_eq!(
        search_until_n2[search_until_n2.len() - 3],
        program_signature_n4
    );
    assert_eq!(
        search_until_n2[search_until_n2.len() - 2],
        program_signature_n3
    );
    assert_eq!(
        search_until_n2[search_until_n2.len() - 1],
        program_signature_n2
    );
    // Search from before the 4th transaction all the way to the start
    let search_before_n4 = endpoint
        .search_signatures(
            &program_id,
            Some(program_signature_n4),
            None,
            usize::MAX,
        )
        .await
        .unwrap();
    assert_eq!(search_before_n4.len(), 3);
    assert_eq!(search_before_n4[0], program_signature_n3);
    assert_eq!(search_before_n4[1], program_signature_n2);
    assert_eq!(search_before_n4[2], program_signature_n1);
    // Search from before the 4th transaction until the 2nd
    let search_before_n4_until_n2 = endpoint
        .search_signatures(
            &program_id,
            Some(program_signature_n4),
            Some(program_signature_n2),
            usize::MAX,
        )
        .await
        .unwrap();
    assert_eq!(search_before_n4_until_n2.len(), 2);
    assert_eq!(search_before_n4_until_n2[0], program_signature_n3);
    assert_eq!(search_before_n4_until_n2[1], program_signature_n2);
    // Search from before an invalid signature (must return nothing)
    let search_before_invalid = endpoint
        .search_signatures(
            &program_id,
            Some(Signature::new_unique()),
            None,
            usize::MAX,
        )
        .await
        .unwrap();
    assert_eq!(search_before_invalid.len(), 0);
    // Search until an invalid signature (must return everything)
    let search_until_invalid = endpoint
        .search_signatures(
            &program_id,
            None,
            Some(Signature::new_unique()),
            usize::MAX,
        )
        .await
        .unwrap();
    assert_eq!(search_until_invalid.len(), search_until_n2.len() + 1);
    assert_eq!(
        search_until_invalid[..search_until_invalid.len() - 1],
        search_until_n2[..]
    );
    // Search with a limit
    let search_limited =
        endpoint.search_signatures(&program_id, None, None, 100).await.unwrap();
    assert_eq!(search_limited.len(), 100);
    assert_eq!(search_limited[..], search_until_n2[..100]);
    // Search invalid order
    let search_order_invalid = endpoint
        .search_signatures(
            &program_id,
            Some(program_signature_n3),
            Some(program_signature_n4),
            100,
        )
        .await
        .unwrap();
    assert_eq!(
        search_order_invalid,
        vec![program_signature_n2, program_signature_n1]
    );
}
