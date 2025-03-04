use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;

#[tokio::test]
pub async fn run() {
    // Create the endpoint pointing to devnet
    let mut endpoint = ToolboxEndpoint::new_devnet().await;
    // Tests constants
    let program_id =
        Pubkey::from_str("UCNcQRtrbGmvuLKA3Jv719Cc6DS4r661ZRpyZduxu2j")
            .unwrap();
    let discriminator = [50, 40, 49, 11, 157, 220, 229, 192];
    let blob_from_address1 =
        Pubkey::from_str("Ady55LhZxWFABzdg8NCNTAZv5XstBqyNZYCMfWqW3Rq9")
            .unwrap()
            .to_bytes();
    let blob_from_address2 =
        Pubkey::from_str("EsQycjp856vTPvrxMuH1L6ymd5K63xT7aULGepiTcgM3")
            .unwrap()
            .to_bytes();
    // Searching accounts with no filters, will return all the program's accounts
    let search_unfiltered = endpoint
        .search_addresses(&program_id, None, &[])
        .await
        .unwrap();
    assert!(!search_unfiltered.is_empty());
    // Searching accounts by matching on the discriminator
    let search_by_discriminator = endpoint
        .search_addresses(&program_id, None, &[(0, &discriminator)])
        .await
        .unwrap();
    assert!(search_by_discriminator.len() < search_unfiltered.len());
    assert_eq!(search_by_discriminator.len(), 5);
    // Searching accounts by matching the exact account size
    let search_by_data_len = endpoint
        .search_addresses(&program_id, Some(680), &[])
        .await
        .unwrap();
    assert_eq!(search_by_discriminator, search_by_data_len);
    // Searching accounts by matching a public key from the data content
    let search_by_data_blob1 = endpoint
        .search_addresses(&program_id, None, &[(17, &blob_from_address1)])
        .await
        .unwrap();
    assert_eq!(search_by_discriminator, search_by_data_blob1);
    // Searching accounts by matching a public key from the data content
    let search_by_data_blob2 = endpoint
        .search_addresses(&program_id, None, &[(49, &blob_from_address2)])
        .await
        .unwrap();
    assert_eq!(search_by_discriminator, search_by_data_blob2);
    // Searching accounts by applying all the restrictions at once
    let search_by_everything = endpoint
        .search_addresses(
            &program_id,
            Some(680),
            &[
                (17, &blob_from_address1),
                (0, &discriminator),
                (49, &blob_from_address2),
            ],
        )
        .await
        .unwrap();
    assert_eq!(search_by_discriminator, search_by_everything);
    // Searching accounts by applying one correct and one wrong filter
    let search_by_failure = endpoint
        .search_addresses(
            &program_id,
            Some(680),
            &[(0, &discriminator), (8, &[42])],
        )
        .await
        .unwrap();
    assert!(search_by_failure.is_empty());
}
