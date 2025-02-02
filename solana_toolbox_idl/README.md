# Solana Toolbox IDL

## Abstract

This crate provide a framework to interact with solana smart contracts by using IDLs directly.

For example, using an IDL file, or an IDL downloaded from chain we can:

- Read account data into JSON (decompile the account)
- Generate transaction data from JSON (compile the transaction data)
- Resolve an instruction accounts addresses (by looking at the seeds in the IDL)

## Install

Make sure to install the version that matches your solana version.
Each solana version may come with a different set of dependency hell, so you can specify the solana version you're using directly in the version of this crate.

in `Cargo.toml`:

```toml
# For example when using solana version 1.18.26"
solana_toolbox_idl = "=0.1.38-solana-1.18.26"
# Or when using solana version 2.1.4"
solana_toolbox_idl = "=0.1.38-solana-2.1.4"
```

## Examples

The main type provided is the `ToolboxIdl`, it can be parsed from an IDL's JSON string, or downloaded from the chain directly by looking at a `program_id`.

Note: see `solana_toolbox_endpoint` crate for interacting with a RPC or ProgramTest environment.

We can create our IDL object in different ways:

```rust
// Parse IDL from file JSON string directly
let idl_string = read_to_string("./my_idl.json").unwrap();
let idl = ToolboxIdl::try_from_str(&idl_string).unwrap();
// Fetch an IDL from an endpoint and a program_id
let idl = ToolboxIdl::get_for_program_id(&mut endpoint, &program_id)
    .await
    .unwrap()
    .unwrap();
// We can also manually generate IDLs inline (with or without shortcut syntax)
let idl = ToolboxIdl::try_from_value(&json!({
    "instructions": {
        "my_instruction": {
            "accounts": [{ "name": "payer", "signer": true }],
            "args": [{ "name": "arg", "type": "MyArg" }]
        }
    },
    "types": {
        "MyArg": {
            "fields": [{ "name": "info", "type": "u64" }]
        }
    },
    "accounts": {
        "MyAccount": {
            "fields": [
                { "name": "id", "type": "pubkey" },
                { "name": "bytes", "type": ["u8", 42] },
            ]
        }
    },
    "errors": {},
})).unwrap();
```

Once we have our IDL object `ToolboxIdl` instanciated we can use it for various actions:

```rust
// We can fetch an account state and parse it into a JSON object
let account = idl
    .get_account(&mut endpoint, &my_account_address)
    .await
    .unwrap()
    .unwrap();
// We can generate an instruction from JSON args data and account addresses
let instruction = idl
    .compile_instruction(
        &ToolboxIdlInstruction {
            program_id,
            name: "my_instruction".to_string(),
            accounts_addresses: HashMap::from_iter([
                ("payer".to_string(), payer.pubkey()),
            ]),
            args: json!({ "arg": {"info": 42} }),
        },
    )
    .unwrap();
// We can also try to resolve all instruction informations by automatically
// filling in missing info from IDL and on-chain accounts state (smart-compile)
let instruction = idl
    .resolve_instruction(
        &mut endpoint, // See solana_toolbox_endpoint crate
        &ToolboxIdlInstruction {
            program_id,
            name: "my_instruction".to_string(),
            accounts_addresses: HashMap::from_iter([
                ("payer".to_string(), payer.pubkey()),
            ]),
            args: json!({ "arg": {"info": 42} }),
        },
    )
    .unwrap();
```

## Documentation

See the docs for the exhaustive list of the `ToolboxIdl` capabilities:

- [https://docs.rs/solana_toolbox_idl/latest/solana_toolbox_idl/struct.ToolboxIdl.html](https://docs.rs/solana_toolbox_idl/latest/solana_toolbox_idl/struct.ToolboxIdl.html)
