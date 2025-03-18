use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::toolbox_idl_utils::idl_ok_or_else;

impl ToolboxIdlInstructionAccount {
    pub fn try_compute(
        &self,
        program_id: &Pubkey,
        addresses: &HashMap<String, Pubkey>,
        snapshots: &HashMap<String, (Arc<ToolboxIdlTypeFull>, Value)>,
        args_payload: &(&ToolboxIdlTypeFullFields, &Value),
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Pubkey, ToolboxIdlError> {
        if let Some(address) = addresses.get(&self.name) {
            return Ok(*address);
        }
        if let Some(instruction_account_address) = &self.address {
            return Ok(*instruction_account_address);
        }
        if let Some(instruction_account_pda) = &self.pda {
            return instruction_account_pda.try_compute(
                program_id,
                addresses,
                snapshots,
                args_payload,
                &breadcrumbs.with_idl("pda"),
            );
        }
        idl_err("Unresolvable account", &breadcrumbs.as_idl("@"))
    }
}

impl ToolboxIdlInstructionAccountPda {
    pub fn try_compute(
        &self,
        program_id: &Pubkey,
        addresses: &HashMap<String, Pubkey>,
        snapshots: &HashMap<String, (Arc<ToolboxIdlTypeFull>, Value)>,
        args_payload: &(&ToolboxIdlTypeFullFields, &Value),
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Pubkey, ToolboxIdlError> {
        let mut pda_seeds_bytes = vec![];
        for (_, pda_seed_blob, breadcrumbs) in idl_iter_get_scoped_values(
            &self.seeds,
            &breadcrumbs.with_idl("seeds"),
        )? {
            pda_seeds_bytes.push(pda_seed_blob.try_compute(
                addresses,
                snapshots,
                args_payload,
                &breadcrumbs,
            )?);
        }
        let pda_program_id = if let Some(pda_program_blob) = &self.program {
            let pda_program_id_bytes = pda_program_blob.try_compute(
                addresses,
                snapshots,
                args_payload,
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
            *program_id
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
        addresses: &HashMap<String, Pubkey>,
        snapshots: &HashMap<String, (Arc<ToolboxIdlTypeFull>, Value)>,
        args_payload: &(&ToolboxIdlTypeFullFields, &Value),
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        match self {
            ToolboxIdlInstructionAccountPdaBlob::Const { bytes } => {
                Ok(bytes.clone())
            },
            ToolboxIdlInstructionAccountPdaBlob::Arg { path } => {
                let idl_blob_parts = Vec::from_iter(path.split("."));
                let (args_type_full_fields, payload) = args_payload;
                ToolboxIdlInstructionAccountPdaBlob::try_compute_path_data(
                    args_type_full_fields,
                    payload,
                    &idl_blob_parts,
                    &breadcrumbs.with_idl("args"),
                )
            },
            ToolboxIdlInstructionAccountPdaBlob::Account { path } => {
                let idl_blob_parts = Vec::from_iter(path.split("."));
                if idl_blob_parts.len() == 1 {
                    return idl_map_get_key_or_else(
                        addresses,
                        path,
                        &breadcrumbs.val(),
                    )
                    .map(|address| address.to_bytes().to_vec());
                }
                let (content_type_full, state) = idl_map_get_key_or_else(
                    snapshots,
                    idl_blob_parts[0],
                    &breadcrumbs.as_val("snapshots"),
                )?;
                let data_type_full_fields = idl_ok_or_else(
                    content_type_full.as_struct_fields(),
                    "expected a struct fields",
                    &breadcrumbs.idl(),
                )?;
                ToolboxIdlInstructionAccountPdaBlob::try_compute_path_data(
                    data_type_full_fields,
                    state,
                    &idl_blob_parts[1..],
                    &breadcrumbs.with_val(idl_blob_parts[0]),
                )
            },
        }
    }

    fn try_compute_path_data(
        type_full_fields: &ToolboxIdlTypeFullFields,
        value: &Value,
        parts: &[&str],
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        let lookup_name = parts[0];
        // TODO - support unamed structs as arg ?
        let type_full_fields_named = idl_ok_or_else(
            type_full_fields.as_named(),
            "expected named fields",
            &breadcrumbs.idl(),
        )?;
        let value_object = idl_as_object_or_else(value, &breadcrumbs.val())?;
        for (field_name, field_type_full) in type_full_fields_named {
            let breadcrumbs = &breadcrumbs.with_idl(field_name);
            if field_name == lookup_name {
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
                let type_full_fields = idl_ok_or_else(
                    field_type_full.as_struct_fields(),
                    "expected a struct fields",
                    &breadcrumbs.idl(),
                )?;
                return ToolboxIdlInstructionAccountPdaBlob::try_compute_path_data(
                    type_full_fields,
                    value_field,
                    &parts[1..],
                    &breadcrumbs.with_idl("*"),
                );
            }
        }
        idl_err("Unknown value field", &breadcrumbs.as_val(lookup_name))
    }
}
