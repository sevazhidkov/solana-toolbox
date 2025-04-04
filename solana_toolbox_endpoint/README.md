# Solana Toolbox Endpoint

## Abstract

This crate provide boilerplate types to enable writing endpoint-independent code that interact interchangebly with:

- mainnet-meta (using an RpcClient internally)
- devnet (using an RpcClient internally)
- testnet (using an RpcClient internally)
- localnet (using an RpcClient internally)
- memnet (using a ProgramTest mocked Solana runtime)

The same code can then be run on a different cluster or a different runtime, just by swapping the `ToolboxEndpoint` object

## Install

Make sure to install the version that matches your solana version.
Each solana version may come with a different set of dependency hell, so you can specify the solana version you're using directly in the version of this crate.

in `Cargo.toml`:

```toml
# For example when using solana version 1.18.26"
solana_toolbox_endpoint = "=0.3.11-solana-1.18.26"
# Or when using solana version 2.1.4"
solana_toolbox_endpoint = "=0.3.11-solana-2.1.4"
```

## Examples

The main type that we will be using is the `ToolboxEndpoint` that can be initialized from many different types of runtimes/RPCs.

This `ToolboxEndpoint` type then expose capabilities to fetch accounts, execute transactions and a lot of useful boilerplate utilities.

How to create an endpoint object:

```rust
// First we create an endpoint object (here we use "program_test" or "memnet")
let mut endpoint = ToolboxEndpoint::new_program_test_with_builtin_programs(&[
    toolbox_endpoint_program_test_builtin_program_anchor!(
        "my_smart_contract",
        my_smart_contract::ID,
        my_smart_contract::entry
    ),
])
.await;
// Alternatively we can use some basic commonly used standard endpoints
let mut endpoint = ToolboxEndpoint::new_devnet().await;
let mut endpoint = ToolboxEndpoint::new_memnet().await;
// Alternatively we can create an endpoint that uses custom devnet URL instead
let mut endpoint = ToolboxEndpoint::new_rpc_with_url_or_moniker_and_commitment(
    "https://api.devnet.solana.com", // or "devnet"
    CommitmentConfig::confirmed(),
);
// Optionally make the endpoint print all transactions being processed
endpoint.add_logger(Box::new(ToolboxEndpointLoggerPrinter::default()));
```

Then we can use our endpoint previously created:

```rust
// Then we can use the endpoint to run arbitrary transaction instructions
endpoint
    .process_instruction(
        &payer,
        Instruction {
            program_id: my_smart_contract::ID,
            accounts: vec![],
            data: vec![],
        },
    )
    .await?;
// The endpoint object provides a lot of useful boilerplate utility functions
endpoint
    .process_system_transfer(
        &payer,
        &source,
        &destination.pubkey(),
        1_000_000_000,
    )
    .await?;
```

## Documentation

See the docs for the exhaustive list of the `ToolboxEndpoint` capabilities:

- [https://docs.rs/solana_toolbox_endpoint/latest/solana_toolbox_endpoint/struct.ToolboxEndpoint.html](https://docs.rs/solana_toolbox_endpoint/latest/solana_toolbox_endpoint/struct.ToolboxEndpoint.html)
