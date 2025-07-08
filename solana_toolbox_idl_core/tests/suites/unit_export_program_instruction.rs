use serde_json::json;
use solana_toolbox_idl_core::ToolboxIdlFormat;
use solana_toolbox_idl_core::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse(&json!({
        "instructions": {
            "my_ix": {
                "docs": ["My Ix"],
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "myAddr",
                        "signer": true,
                        "writable": true,
                        "optional": true,
                        "docs": ["My Ix Account Addr"],
                    }
                ],
                "args": [
                    { "name": "arg", "type": "i16", "docs": ["My Ix Arg"] },
                ]
            }
        },
    }))
    .unwrap();
    // Check the JSON human compact version
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::human()),
        json!({
            "metadata": {},
            "accounts": {},
            "instructions": {
                "my_ix": {
                    "docs": ["My Ix"],
                    "discriminator": [77, 78],
                    "accounts": [
                        {
                            "name": "my_addr",
                            "signer": true,
                            "writable": true,
                            "optional": true,
                            "docs": ["My Ix Account Addr"],
                        }
                    ],
                    "args": [
                        { "name": "arg", "type": "i16", "docs": ["My Ix Arg"] },
                    ]
                }
            },
            "events": {},
            "errors": {},
            "types": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_26()),
        json!({
            "metadata": {},
            "types": [],
            "accounts": [],
            "instructions": [
                {
                    "name": "myIx",
                    "docs": ["My Ix"],
                    "discriminator": [77, 78],
                    "accounts": [
                        {
                            "name": "myAddr",
                            "isSigner": true,
                            "isMut": true,
                            "isOptional": true,
                            "docs": ["My Ix Account Addr"],
                        }
                    ],
                    "args": [
                        { "name": "arg", "type": "i16", "docs": ["My Ix Arg"] },
                    ],
                }
            ],
            "events": [],
            "errors": [],
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::anchor_30()),
        json!({
            "metadata": {},
            "instructions": [
                {
                    "name": "my_ix",
                    "docs": ["My Ix"],
                    "discriminator": [77, 78],
                    "accounts": [
                        {
                            "name": "my_addr",
                            "signer": true,
                            "writable": true,
                            "optional": true,
                            "docs": ["My Ix Account Addr"],
                        }
                    ],
                    "args": [
                        { "name": "arg", "type": "i16", "docs": ["My Ix Arg"] },
                    ],
                }
            ],
            "accounts": [],
            "types": [],
            "events": [],
            "errors": [],
        })
    );
}
