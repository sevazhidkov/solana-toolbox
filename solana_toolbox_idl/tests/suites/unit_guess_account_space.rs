use serde_json::json;
use solana_toolbox_idl::ToolboxIdlAccount;
use solana_toolbox_idl::ToolboxIdlProgram;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypeFull;

#[tokio::test]
pub async fn run() {
    // Create IDLs using different shortened formats
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount1_x3": {
                "space": 3,
                "discriminator": [1],
                "fields": [],
            },
            "MyAccount1_x6": {
                "space": 6,
                "discriminator": [1],
                "fields": [],
            },
            "MyAccount2_x6": {
                "space": 6,
                "discriminator": [2],
            },
        },
        "types": {
            "MyAccount2_x6": {
                "fields": [],
            }
        }
    }))
    .unwrap();
    // Verify known accounts
    assert_eq!(
        *idl_program.accounts.get("MyAccount1_x3").unwrap(),
        ToolboxIdlAccount {
            name: "MyAccount1_x3".to_string(),
            docs: None,
            space: Some(3),
            blobs: vec![],
            discriminator: vec![1],
            content_type_flat: ToolboxIdlTypeFlat::nothing(),
            content_type_full: ToolboxIdlTypeFull::nothing()
        }
        .into()
    );
    assert_eq!(
        *idl_program.accounts.get("MyAccount1_x6").unwrap(),
        ToolboxIdlAccount {
            name: "MyAccount1_x6".to_string(),
            docs: None,
            space: Some(6),
            blobs: vec![],
            discriminator: vec![1],
            content_type_flat: ToolboxIdlTypeFlat::nothing(),
            content_type_full: ToolboxIdlTypeFull::nothing()
        }
        .into()
    );
    assert_eq!(
        *idl_program.accounts.get("MyAccount2_x6").unwrap(),
        ToolboxIdlAccount {
            name: "MyAccount2_x6".to_string(),
            docs: None,
            space: Some(6),
            blobs: vec![],
            discriminator: vec![2],
            content_type_flat: ToolboxIdlTypeFlat::Defined {
                name: "MyAccount2_x6".to_string(),
                generics: vec![]
            },
            content_type_full: ToolboxIdlTypeFull::Typedef {
                name: "MyAccount2_x6".to_string(),
                repr: None,
                content: Box::new(ToolboxIdlTypeFull::nothing())
            }
        }
        .into()
    );
    // Check that we'll pick the right accounts depending on data
    assert_eq!(
        idl_program.guess_account(&[1, 2, 3]),
        Some(idl_program.accounts.get("MyAccount1_x3").unwrap().clone())
    );
    assert_eq!(
        idl_program.guess_account(&[1, 9, 9]),
        Some(idl_program.accounts.get("MyAccount1_x3").unwrap().clone())
    );
    assert_eq!(
        idl_program.guess_account(&[1, 2, 3, 4, 5, 6]),
        Some(idl_program.accounts.get("MyAccount1_x6").unwrap().clone())
    );
    assert_eq!(
        idl_program.guess_account(&[1, 9, 9, 9, 9, 9]),
        Some(idl_program.accounts.get("MyAccount1_x6").unwrap().clone())
    );
    assert_eq!(
        idl_program.guess_account(&[2, 2, 2, 2, 2, 2]),
        Some(idl_program.accounts.get("MyAccount2_x6").unwrap().clone())
    );
    assert_eq!(
        idl_program.guess_account(&[2, 9, 9, 9, 9, 9]),
        Some(idl_program.accounts.get("MyAccount2_x6").unwrap().clone())
    );
    assert_eq!(idl_program.guess_account(&[1, 2]), None);
    assert_eq!(idl_program.guess_account(&[1, 2, 3, 4]), None);
    assert_eq!(idl_program.guess_account(&[1, 2, 3, 4, 5, 6, 7, 8]), None);
    assert_eq!(idl_program.guess_account(&[2, 2, 2]), None);
    assert_eq!(idl_program.guess_account(&[2, 2, 2, 2, 2, 2, 2, 2]), None);
}
