# Solana Toolbox IDL

## Abstract

This crate provide a framework to interact with solana smart contracts by using IDLs directly.

For example, using an IDL file, or an IDL downloaded from chain we can:

- Read account data into JSON (resolve and decode the state of the account)
- Generate transaction data from a JSON IDL (resolve and encode an instruction)
- Resolve an instruction's accounts addresses (by looking at the seeds in the IDL)

## Install

Make sure to install the version that matches your solana version.
Each solana version may come with a different set of dependency hell, so you can specify the solana version you're using directly in the version of this crate.

in `Cargo.toml`:

```toml
# For example when using solana version 1.18.26"
solana_toolbox_idl = "=0.4.0-solana-1.18.26"
# Or when using solana version 2.1.4"
solana_toolbox_idl = "=0.4.0-solana-2.1.4"
```

## Examples

The main type provided is the `ToolboxIdlService`. It contains a cached set of `ToolboxIdlProgram` that can be parsed from an IDL's JSON string, or fetched from the chain directly by looking at a `program_id`'s anchor's IDL account.

`ToolboxIdlService` is useful for actions that may involve unknown programs such as fetching accounts or compiling an instruction, otherwise a `ToolboxIdlProgram` can be used directly for specialized computations.

Note: see `solana_toolbox_endpoint` crate for interacting with a RPC or ProgramTest environment.

```rust
// Instantiate our IDL service that fetch and caches all known IDLs
let mut idl_service = ToolboxIdlService::new();
// We can easily fetch, resolve and decode an account
let my_account_decoded = idl_service
    .get_and_decode_account(&mut endpoint, &my_account_address)
    .await?;
// We'll need a ToolboxIdlProgram when we know exactly which program we're using
let idl_program = idl_service.load_program(&mut endpoint, &my_program_id).await?;
// From there we can read the content of our IDL's program
let idl_instruction = idl_program.instructions.get("my_ix").unwrap();
// We can smartly generate an instruction from JSON data and accounts addresses
let instruction: Instruction = idl_service
    .resolve_and_encode_instruction(
        &mut endpoint,
        &idl_instruction,
        &my_program_id,
        json!({ "param_object": {"info": 42} }),
        HashMap::from_iter([
            ("payer".to_string(), payer.pubkey()),
        ]),
    )?;
// We can also resolve the accounts named addresses using the seeds in the IDL
let instruction_addresses: HashMap<String, Pubkey> = idl_service
    .resolve_instruction_addresses(
        &mut endpoint,
        &idl_instruction,
        &my_program_id,
        json!({ "param_object": {"info": 42} }),
        HashMap::from_iter([
            ("payer".to_string(), payer.pubkey()),
        ]),
    )?;
```

If a program's IDL is not available to be automatically downloaded from endpoint to the `ToolboxIdlService`, we can preload it and provide it for future lookups manually:

```rust
// We can pre-load and save IDLs from file or JSON string directly
let idl_program = ToolboxIdlProgram::try_parse_from_str(
    &read_to_string("./my_idl.json").unwrap()
)?;
idl_service.preload_program(&program_id, Some(idl_program.into()));
// We can also manually generate IDLs inline (with or without shortcut syntax)
let idl_program = ToolboxIdlProgram::try_parse(&json!({
    "name": "my_program",
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
                { "name": "data", "type": ["u8", 42] },
            ]
        }
    },
    "errors": {},
}))?;
idl_service.preload_program(&program_id, Some(idl_program.into()));
```

## Documentation

See the docs for the exhaustive list of the `ToolboxIdlService` capabilities:

- [https://docs.rs/solana_toolbox_idl/latest/solana_toolbox_idl/struct.ToolboxIdlService.html](https://docs.rs/solana_toolbox_idl/latest/solana_toolbox_idl/struct.ToolboxIdlService.html)
