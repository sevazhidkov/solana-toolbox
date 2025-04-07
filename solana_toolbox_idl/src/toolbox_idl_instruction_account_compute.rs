use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

impl ToolboxIdlInstructionAccount {
    pub fn try_find(
        &self,
        instruction_program_id: &Pubkey,
        instruction_args_type_fields: &ToolboxIdlTypeFullFields,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_content_types_and_states: &HashMap<
            String,
            (Arc<ToolboxIdlTypeFull>, Value),
        >,
    ) -> Result<Pubkey> {
        if let Some(address) = instruction_addresses.get(&self.name) {
            return Ok(*address);
        }
        if let Some(instruction_account_address) = &self.address {
            return Ok(*instruction_account_address);
        }
        if let Some(instruction_account_pda) = &self.pda {
            return instruction_account_pda.try_find(
                instruction_program_id,
                instruction_args_type_fields,
                instruction_payload,
                instruction_addresses,
                instruction_content_types_and_states,
            );
        }
        Err(anyhow!("Could not find account (unresolvable)"))
    }
}

impl ToolboxIdlInstructionAccountPda {
    pub fn try_find(
        &self,
        instruction_program_id: &Pubkey,
        instruction_args_type_fields: &ToolboxIdlTypeFullFields,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_content_types_and_states: &HashMap<
            String,
            (Arc<ToolboxIdlTypeFull>, Value),
        >,
    ) -> Result<Pubkey> {
        let mut pda_seeds_bytes = vec![];
        for (_, pda_seed_blob, context) in
            idl_iter_get_scoped_values(&self.seeds)
        {
            pda_seeds_bytes.push(
                pda_seed_blob
                    .try_compute(
                        instruction_args_type_fields,
                        instruction_payload,
                        instruction_addresses,
                        instruction_content_types_and_states,
                    )
                    .context("Seeds")
                    .context(context)?,
            );
        }
        let pda_program_id = if let Some(pda_program_blob) = &self.program {
            let pda_program_id_bytes = pda_program_blob.try_compute(
                instruction_args_type_fields,
                instruction_payload,
                instruction_addresses,
                instruction_content_types_and_states,
            )?;
            Pubkey::new_from_array(pda_program_id_bytes.try_into().map_err(
                |error| anyhow!("Invalid pubkey bytes: {:?}", error),
            )?)
        } else {
            *instruction_program_id
        };
        let mut pda_seeds_slices = vec![];
        for pda_seed_bytes in pda_seeds_bytes.iter() {
            pda_seeds_slices.push(&pda_seed_bytes[..]);
        }
        Ok(Pubkey::find_program_address(&pda_seeds_slices, &pda_program_id).0)
    }
}

impl ToolboxIdlInstructionAccountPdaBlob {
    pub fn try_compute(
        &self,
        instruction_args_type_fields: &ToolboxIdlTypeFullFields,
        instruction_payload: &Value,
        instruction_addresses: &HashMap<String, Pubkey>,
        instruction_content_types_and_states: &HashMap<
            String,
            (Arc<ToolboxIdlTypeFull>, Value),
        >,
    ) -> Result<Vec<u8>> {
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const { bytes } => {
                Ok(bytes.clone())
            },
            ToolboxIdlInstructionAccountPdaBlob::Arg { path } => {
                let idl_blob_parts = path.split(".").collect::<Vec<_>>();
                ToolboxIdlInstructionAccountPdaBlob::try_compute_path_data(
                    instruction_args_type_fields,
                    instruction_payload,
                    &idl_blob_parts,
                )
            },
            ToolboxIdlInstructionAccountPdaBlob::Account { path } => {
                let idl_blob_parts = path.split(".").collect::<Vec<_>>();
                if idl_blob_parts.len() == 1 {
                    return idl_map_get_key_or_else(
                        instruction_addresses,
                        idl_blob_parts[0],
                    )
                    .map(|address| address.to_bytes().to_vec());
                }
                let (account_content_type, account_state) =
                    idl_map_get_key_or_else(
                        instruction_content_types_and_states,
                        idl_blob_parts[0],
                    )?;
                let account_content_type_fields = account_content_type
                    .as_struct_fields()
                    .context("Expected struct fields")?;
                ToolboxIdlInstructionAccountPdaBlob::try_compute_path_data(
                    account_content_type_fields,
                    account_state,
                    &idl_blob_parts[1..],
                )
            },
        }
    }

    fn try_compute_path_data(
        type_full_fields: &ToolboxIdlTypeFullFields,
        value: &Value,
        parts: &[&str],
    ) -> Result<Vec<u8>> {
        let lookup_name = parts[0];
        // TODO (FAR) - support unamed structs as arg ?
        let type_full_fields_named = type_full_fields
            .as_named()
            .context("Expected named fields")?;
        let value_object = idl_as_object_or_else(value)?;
        for (field_name, field_type_full) in type_full_fields_named {
            if field_name == lookup_name {
                let value_field =
                    idl_object_get_key_or_else(value_object, field_name)?;
                if parts.len() == 1 {
                    let mut bytes = vec![];
                    field_type_full.try_serialize(
                        value_field,
                        &mut bytes,
                        false,
                    )?;
                    return Ok(bytes);
                }
                let type_full_fields = field_type_full
                    .as_struct_fields()
                    .context("Expected struct fields")?;
                return ToolboxIdlInstructionAccountPdaBlob::try_compute_path_data(
                    type_full_fields,
                    value_field,
                    &parts[1..],
                );
            }
        }
        Err(anyhow!("Could not lookup value at: {}", lookup_name))
    }
}
