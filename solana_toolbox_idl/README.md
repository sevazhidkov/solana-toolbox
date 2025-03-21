# Solana Toolbox IDL

## Abstract

This crate provide a framework to interact with solana smart contracts by using IDLs directly.

For example, using an IDL file, or an IDL downloaded from chain we can:

- Read account data into JSON (resolve and decompile the account)
- Generate transaction data from JSON (compile the transaction data)
- Resolve an instruction's accounts addresses (by looking at the seeds in the IDL)

## Install

Make sure to install the version that matches your solana version.
Each solana version may come with a different set of dependency hell, so you can specify the solana version you're using directly in the version of this crate.

in `Cargo.toml`:

```toml
# For example when using solana version 1.18.26"
solana_toolbox_idl = "=0.3.4-solana-1.18.26"
# Or when using solana version 2.1.4"
solana_toolbox_idl = "=0.3.4-solana-2.1.4"
```

## Examples

The main type provided is the `ToolboxIdlResolver`. It contains cached a set of `ToolboxIdlProgram` that can be parsed from an IDL's JSON string, or downloaded from the chain directly by looking at a `program_id`.

Note: see `solana_toolbox_endpoint` crate for interacting with a RPC or ProgramTest environment.

```rust
// Instantiate our IDL resolver that fetch and caches all known IDLs
let mut idl_resolver = ToolboxIdlResolver::new();
// We can fetch and resolve all account details (metadata and state)
let account_details = idl_resolver
    .resolve_account_details(&mut endpoint, &my_account)
    .await?
    .unwrap();
// We can generate an instruction from JSON data and accounts addresses
let instruction: Instruction = idl_resolver
    .resolve_instruction(
        program_id,
        "my_ix",
        json!({ "param_object": {"info": 42} }),
        HashMap::from_iter([
            ("payer".to_string(), payer.pubkey()),
        ]),
    )?;
// We can also resolve the accounts named addresses using the seeds in the IDL
let instruction_addresses: HashMap<String, Pubkey> = idl_resolver
    .resolve_instruction_addresses(
        program_id,
        "my_ix",
        json!({ "param_object": {"info": 42} }),
        HashMap::from_iter([
            ("payer".to_string(), payer.pubkey()),
        ]),
    )?;
```

If a program's IDL is not available to be automatically downloaded from endpoint to the `ToolboxIdlResolver`, we can preload it and provide it for future lookups manually:

```rust
// We can pre-load and save IDLs from file or JSON string directly
let idl_string = read_to_string("./my_idl.json").unwrap();
let idl_program = ToolboxIdlProgram::try_parse_from_str(&idl_string)?;
idl_resolver.preload_program(&program_id, idl_program);
// We can also manually generate IDLs inline (with or without shortcut syntax)
let idl_program = ToolboxIdlProgram::try_parse_from_value(&json!({
    "instructions": {
        "my_ix": {
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
idl_resolver.preload_program(&program_id, idl_program);
```

## Documentation

See the docs for the exhaustive list of the `ToolboxIdlResolver` capabilities:

- [https://docs.rs/solana_toolbox_idl/latest/solana_toolbox_idl/struct.ToolboxIdlResolver.html](https://docs.rs/solana_toolbox_idl/latest/solana_toolbox_idl/struct.ToolboxIdlResolver.html)
