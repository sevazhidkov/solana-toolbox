use serde_json::json;
use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create IDLs on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "accounts": {
            "MyAccount": {
                "docs": ["My Account"],
                "discriminator": [77, 78],
            }
        },
        "types": {
            "MyAccount": {
                "fields": [],
            }
        }
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::human()),
        json!({
            "metadata": {},
            "instructions": {},
            "accounts": {
                "MyAccount": {
                    "docs": ["My Account"],
                    "discriminator": [77, 78],
                    "type": "MyAccount",
                }
            },
            "types": {
                "MyAccount": {
                    "fields": [],
                }
            },
            "errors": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_26()),
        json!({
            "metadata": {},
            "instructions": [],
            "accounts": [
                {
                    "name": "MyAccount",
                    "docs": ["My Account"],
                    "discriminator": [77, 78],
                    "type": { "defined": "MyAccount" },
                }
            ],
            "types": [
                {
                    "name": "MyAccount",
                    "type": {
                        "kind": "struct",
                        "fields": [],
                    }
                }
            ],
            "errors": [],
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_30()),
        json!({
            "metadata": {},
            "instructions": [],
            "accounts": [
                {
                    "name": "MyAccount",
                    "docs": ["My Account"],
                    "discriminator": [77, 78],
                    "type": { "defined": {"name": "MyAccount"} },
                }
            ],
            "types": [
                {
                    "name": "MyAccount",
                    "type": {
                        "kind": "struct",
                        "fields": [],
                    }
                }
            ],
            "errors": [],
        })
    );
}
