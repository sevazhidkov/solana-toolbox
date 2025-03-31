use serde_json::json;
use solana_toolbox_idl::ToolboxIdlFormat;
use solana_toolbox_idl::ToolboxIdlProgram;

#[tokio::test]
pub async fn run() {
    // Create an IDL on the fly
    let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
        "instructions": {
            "my_ix": {
                "docs": ["My Ix"],
                "discriminator": [77, 78],
                "accounts": [
                    {
                        "name": "addr",
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
        idl_program.export(&ToolboxIdlFormat::Human),
        json!({
            "metadata": {},
            "instructions": {
                "my_ix": {
                    "docs": ["My Ix"],
                    "discriminator": [77, 78],
                    "accounts": [
                        {
                            "name": "addr",
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
            "accounts": {},
            "errors": {},
            "types": {},
        })
    );
    // Check the JSON backward compatibility version for anchor 26
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Anchor26),
        json!({
            "accounts": [],
            "errors": [],
            "instructions": [
                {
                    "name": "my_ix",
                    "docs": ["My Ix"],
                    "discriminator": [77, 78],
                    "accounts": [
                        {
                            "name": "addr",
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
            "types": []
        })
    );
    // Check the JSON backward compatibility version for anchor 30
    assert_eq!(
        idl_program.export(&ToolboxIdlFormat::Anchor30),
        json!({
            "metadata": {},
            "accounts": [],
            "errors": [],
            "instructions": [
                {
                    "name": "my_ix",
                    "docs": ["My Ix"],
                    "discriminator": [77, 78],
                    "accounts": [
                        {
                            "name": "addr",
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
            "types": []
        })
    );
}
