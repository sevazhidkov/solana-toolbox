use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use clap::Args;
use serde_json::json;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_toolbox_endpoint::ToolboxEndpoint;
use solana_toolbox_idl::ToolboxIdlTypeFlat;
use solana_toolbox_idl::ToolboxIdlTypedef;

use crate::toolbox_cli_context::ToolboxCliContext;

#[derive(Debug, Clone, Args)]
#[command(about = "Find a Pda's pubkey from arbitrary seeds")]
pub struct ToolboxCliCommandPdaArgs {
    #[arg(value_name = "PROGRAM_ID", help = "The Pda's ProgramId")]
    program_id: String,
    #[arg(
        value_name = "SEED_TYPE:SEED_VALUE",
        help = "The Pda's seed type and value"
    )]
    seeds: Vec<String>,
    #[arg(
        long = "typedef",
        alias = "typedefs",
        value_name = "TYPEDEF_NAME:TYPEDEF_TYPE",
        help = "Define a type that can be used to serialize seeds"
    )]
    typedefs: Vec<String>,
}

impl ToolboxCliCommandPdaArgs {
    pub async fn process(&self, context: &ToolboxCliContext) -> Result<Value> {
        let program_id = context
            .parse_key(&self.program_id)
            .context("Parse ProgramId")?
            .address();

        let mut typedefs = HashMap::new();
        for typedef in &self.typedefs {
            if let Some((typedef_name, typedef_type)) = typedef.split_once(":")
            {
                let typedef = ToolboxIdlTypedef::try_parse(
                    typedef_name,
                    &context
                        .parse_hjson(typedef_type)
                        .context("Parse Typedef JSON")?,
                )
                .context("Parse Typedef Declaration")?;
                typedefs.insert(typedef_name.to_string(), Arc::new(typedef));
            }
        }

        let mut json_seeds = vec![];
        let mut seeds_bytes = vec![];
        for seed in &self.seeds {
            if let Some((seed_type, seed_value)) = seed.split_once(":") {
                let seed_type_flat = ToolboxIdlTypeFlat::try_parse_value(
                    &context
                        .parse_hjson(seed_type)
                        .context("Parse Seed Type JSON")?,
                )?;
                let seed_type_full =
                    seed_type_flat.try_hydrate(&HashMap::new(), &typedefs)?;
                let seed_value = context
                    .parse_hjson(seed_value)
                    .context("Parse Seed Value JSON")?;
                let mut seed_bytes = vec![];
                seed_type_full.try_serialize(
                    &seed_value,
                    &mut seed_bytes,
                    false,
                )?;
                json_seeds.push(json!({
                    "type": seed_type_full.explain(),
                    "value": seed_value,
                    "bytes": {
                        "base16": ToolboxEndpoint::encode_base16(&seed_bytes),
                        "base58": ToolboxEndpoint::encode_base58(&seed_bytes),
                        "base64": ToolboxEndpoint::encode_base64(&seed_bytes),
                        "utf8_lossy": String::from_utf8_lossy(&seed_bytes)
                    }
                }));
                seeds_bytes.push(seed_bytes);
            }
        }

        let mut seeds_slices = vec![];
        for seed_byte in &seeds_bytes {
            seeds_slices.push(&seed_byte[..]);
        }
        let pda = Pubkey::find_program_address(&seeds_slices, &program_id);

        Ok(json!({
            "seeds": json_seeds,
            "pda": {
                "address": pda.0.to_string(),
                "bump": pda.1,
            }
        }))
    }
}
