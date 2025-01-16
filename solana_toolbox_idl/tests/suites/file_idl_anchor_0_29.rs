use std::fs::canonicalize;
use std::fs::read_to_string;

use solana_toolbox_idl::ToolboxIdl;

#[tokio::test]
pub async fn file_idl_anchor_0_29() {
    let idl_path =
        canonicalize("./tests/fixtures/dummy_redemption_anchor_0_29.json")
            .unwrap();
    eprintln!("idl_path:{:?}", idl_path);

    let idl_string = read_to_string(idl_path).unwrap();
    eprintln!("idl_string:{:?}", idl_string);

    let idl = ToolboxIdl::try_from_str(&idl_string);
    eprintln!("idl:{:?}", idl);

    panic!("YESSSS :ok:");
}
