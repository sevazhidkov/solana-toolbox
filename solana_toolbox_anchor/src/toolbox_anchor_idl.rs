/*
pub async fn get_account_data_anchor_idl(
    toolbox_endpoint: &mut ToolboxEndpoint,
    program_id: &Pubkey,
) -> Result<(), ToolboxAnchorError> {
    let base = Pubkey::find_program_address(&[], program_id).0;
    let address =
        Pubkey::create_with_seed(&base, "anchor:idl", program_id).unwrap();

    let raw_account_data = toolbox_endpoint
        .get_account_data(&address)
        .await
        .map_err(ToolboxAnchorError::ToolboxEndpoint)?;
    eprintln!("raw_account_data:{:?}", raw_account_data);

    let mut discriminant_slice: [u8; 8] = Default::default();
    discriminant_slice.copy_from_slice(&raw_account_data[0..8]);
    eprintln!("discriminant_slice:{:?}", discriminant_slice);

    let mut authority_slice: [u8; 32] = Default::default();
    authority_slice.copy_from_slice(&raw_account_data[8..40]);
    let authority = Pubkey::new_from_array(authority_slice);
    eprintln!("authority:{:?}", authority);

    let mut length_slice: [u8; 4] = Default::default();
    length_slice.copy_from_slice(&raw_account_data[40..44]);
    let length = u32::from_le_bytes(length_slice);
    eprintln!("length:{:?}", length);

    let dudu =
        inflate_bytes_zlib(&raw_account_data[44..((44 + length) as usize)])
            .unwrap();
    eprintln!("dudu: {:?}", dudu);

    let wowow = String::from_utf8(dudu).unwrap();
    eprintln!("wowow: {:?}", wowow);

    panic!("lol");

    Ok(())
}
*/
