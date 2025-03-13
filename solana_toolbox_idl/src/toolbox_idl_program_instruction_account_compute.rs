use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFull;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

use crate::ToolboxIdlBreadcrumbs;
use crate::ToolboxIdlError;
use crate::ToolboxIdlProgramInstructionAccount;
use crate::ToolboxIdlProgramInstructionAccountPda;
use crate::ToolboxIdlProgramInstructionAccountPdaBlob;
use crate::ToolboxIdlProgramTypeFullFields;

impl ToolboxIdlProgramInstructionAccount {
    pub fn try_compute(
        &self,
        program_id: Pubkey,
        accounts_addresses: &HashMap<String, Pubkey>,
        accounts: &HashMap<String, (ToolboxIdlProgramTypeFull, Value)>,
        args: &(ToolboxIdlProgramTypeFull, Value),
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Pubkey, ToolboxIdlError> {
        if let Some(instruction_account_address) =
            accounts_addresses.get(&self.name)
        {
            return Ok(*instruction_account_address);
        }
        if let Some(program_instruction_account_address) = &self.address {
            return Ok(*program_instruction_account_address);
        }
        if let Some(program_instruction_account_pda) = &self.pda {
            return program_instruction_account_pda.try_compute(
                program_id,
                accounts_addresses,
                accounts,
                args,
                &breadcrumbs.with_idl("pda"),
            );
        }
        idl_err("Unresolvable account", &breadcrumbs.as_idl("@"))
    }
}

impl ToolboxIdlProgramInstructionAccountPda {
    pub fn try_compute(
        &self,
        program_id: Pubkey,
        accounts_addresses: &HashMap<String, Pubkey>,
        accounts: &HashMap<String, (ToolboxIdlProgramTypeFull, Value)>,
        args: &(ToolboxIdlProgramTypeFull, Value),
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Pubkey, ToolboxIdlError> {
        let mut pda_seeds_bytes = vec![];
        for pda_seed_blob in &self.seeds {
            pda_seeds_bytes.push(pda_seed_blob.try_compute(
                accounts_addresses,
                accounts,
                args,
                breadcrumbs,
            )?);
        }
        let pda_program_id = if let Some(pda_program_blob) = &self.program {
            let pda_program_id_bytes = pda_program_blob.try_compute(
                accounts_addresses,
                accounts,
                args,
                &breadcrumbs.with_idl("program"),
            )?;
            Pubkey::new_from_array(pda_program_id_bytes.try_into().map_err(
                |err| {
                    ToolboxIdlError::Custom {
                        failure: format!("value:{:?}", err), // TODO - better error handling and breadcrumbs
                        context: breadcrumbs.as_idl("program_id"),
                    }
                },
            )?)
        } else {
            program_id
        };
        let mut pda_seeds_slices = vec![];
        for pda_seed_bytes in pda_seeds_bytes.iter() {
            pda_seeds_slices.push(&pda_seed_bytes[..]);
        }
        Ok(Pubkey::find_program_address(&pda_seeds_slices, &pda_program_id).0)
    }
}

impl ToolboxIdlProgramInstructionAccountPdaBlob {
    pub fn try_compute(
        &self,
        accounts_addresses: &HashMap<String, Pubkey>,
        accounts: &HashMap<String, (ToolboxIdlProgramTypeFull, Value)>,
        args: &(ToolboxIdlProgramTypeFull, Value),
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        match self {
            ToolboxIdlProgramInstructionAccountPdaBlob::Const { bytes } => {
                Ok(bytes.clone())
            },
            ToolboxIdlProgramInstructionAccountPdaBlob::Account { path } => {
                let idl_blob_parts = Vec::from_iter(path.split("."));
                if idl_blob_parts.len() == 1 {
                    return idl_map_get_key_or_else(
                        &accounts_addresses,
                        path,
                        &breadcrumbs.val(),
                    )
                    .map(|address| address.to_bytes().to_vec());
                }
                let (account_data_type_full, account_state) =
                    idl_map_get_key_or_else(
                        accounts,
                        idl_blob_parts[0],
                        &breadcrumbs.as_val("instruction_accounts_state"),
                    )?;
                ToolboxIdlProgramInstructionAccountPdaBlob::try_compute_path_data(
                    account_data_type_full,
                    account_state,
                    &idl_blob_parts[1..],
                    &breadcrumbs
                        .with_val(idl_blob_parts[0]),
                )
            },
            ToolboxIdlProgramInstructionAccountPdaBlob::Arg { path } => {
                let idl_blob_parts = Vec::from_iter(path.split("."));
                let (data_type_full, args) = args;
                ToolboxIdlProgramInstructionAccountPdaBlob::try_compute_path_data(
                    data_type_full,
                    args,
                    &idl_blob_parts,
                    &breadcrumbs
                        .with_idl("args"),
                )
            },
        }
    }

    fn try_compute_path_data(
        type_full: &ToolboxIdlProgramTypeFull,
        value: &Value,
        parts: &[&str],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let current = parts[0];
        // TODO - support unamed structs as arg ?
        let value_object = idl_as_object_or_else(value, &breadcrumbs.val())?;
        let named_fields = match type_full {
            ToolboxIdlProgramTypeFull::Struct {
                fields: ToolboxIdlProgramTypeFullFields::Named(fields),
            } => fields,
            _ => {
                return idl_err(
                    "Expected struct fields named",
                    &breadcrumbs.idl(),
                )
            },
        };
        // TODO - remove the need for snake case by parsing everything in snake case
        for (field_name, field_type_full) in named_fields {
            let breadcrumbs = &breadcrumbs.with_idl(field_name);
            if field_name.to_case(Case::Snake) == current.to_case(Case::Snake) {
                let value_field = idl_object_get_key_or_else(
                    value_object,
                    field_name,
                    &breadcrumbs.val(),
                )?;
                if parts.len() == 1 {
                    let mut bytes = vec![];
                    field_type_full.try_serialize(
                        value_field,
                        &mut bytes,
                        false,
                        &breadcrumbs.with_val(field_name),
                    )?;
                    return Ok(bytes);
                }
                return ToolboxIdlProgramInstructionAccountPdaBlob::try_compute_path_data(
                    field_type_full,
                    value_field,
                    &parts[1..],
                    &breadcrumbs.with_idl("*"),
                );
            }
        }
        idl_err("Unknown value field", &breadcrumbs.as_val(current))
    }
}
