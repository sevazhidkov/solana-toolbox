use std::collections::HashMap;

use convert_case::Case;
use convert_case::Casing;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFull;
use crate::toolbox_idl_program_type_full::ToolboxIdlProgramTypeFullFields;
use crate::toolbox_idl_transaction_instruction::ToolboxIdlTransactionInstruction;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;
use crate::ToolboxIdlProgramAccount;
use crate::ToolboxIdlProgramInstruction;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramInstructionAccountBlob {
    Const { bytes: Vec<u8> },
    Arg { path: String },
    Account { path: String },
}

impl ToolboxIdlProgramInstructionAccountBlob {
    pub fn try_parse(
        idl_instruction_account_blob: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccountBlob, ToolboxIdlError> {
        if let Some(idl_instruction_account_blob) =
            idl_instruction_account_blob.as_object()
        {
            return ToolboxIdlProgramInstructionAccountBlob::try_parse_object(
                idl_instruction_account_blob,
                breadcrumbs,
            );
        }
        ToolboxIdlProgramInstructionAccountBlob::try_parse_const(
            idl_instruction_account_blob,
            breadcrumbs,
        )
    }

    fn try_parse_object(
        idl_instruction_account_blob: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccountBlob, ToolboxIdlError> {
        if let Some(idl_instruction_account_blob_value) =
            idl_instruction_account_blob.get("value")
        {
            return ToolboxIdlProgramInstructionAccountBlob::try_parse_const(
                idl_instruction_account_blob_value,
                breadcrumbs,
            );
        }
        let idl_instruction_account_blob_kind =
            idl_object_get_key_as_str_or_else(
                idl_instruction_account_blob,
                "kind",
                &breadcrumbs.idl(),
            )?;
        let idl_instruction_account_blob_path =
            idl_object_get_key_as_str_or_else(
                idl_instruction_account_blob,
                "path",
                &breadcrumbs.idl(),
            )?;
        match idl_instruction_account_blob_kind {
            "account" => Ok(ToolboxIdlProgramInstructionAccountBlob::Account {
                path: idl_instruction_account_blob_path.to_string(),
            }),
            "arg" => Ok(ToolboxIdlProgramInstructionAccountBlob::Arg {
                path: idl_instruction_account_blob_path.to_string(),
            }),
            _ => idl_err("unknown blob kind", &breadcrumbs.idl()),
        }
    }

    fn try_parse_const(
        idl_instruction_account_blob: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccountBlob, ToolboxIdlError> {
        if let Some(idl_instruction_account_blob) =
            idl_instruction_account_blob.as_array()
        {
            return Ok(ToolboxIdlProgramInstructionAccountBlob::Const {
                bytes: idl_as_bytes_or_else(
                    idl_instruction_account_blob,
                    &breadcrumbs.idl(),
                )?,
            });
        }
        if let Some(idl_instruction_account_blob) =
            idl_instruction_account_blob.as_str()
        {
            return Ok(ToolboxIdlProgramInstructionAccountBlob::Const {
                bytes: idl_instruction_account_blob.as_bytes().to_vec(),
            });
        }
        idl_err("Could not parse blob bytes", &breadcrumbs.idl())
    }

    pub fn try_resolve(
        &self,
        program_instruction: &ToolboxIdlProgramInstruction,
        program_accounts: &HashMap<String, ToolboxIdlProgramAccount>,
        transaction_instruction: &ToolboxIdlTransactionInstruction,
        transaction_instruction_accounts: &HashMap<String, ToolboxIdlAccount>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        match self {
            ToolboxIdlProgramInstructionAccountBlob::Const { bytes } => {
                Ok(bytes.clone())
            },
            ToolboxIdlProgramInstructionAccountBlob::Account { path } => {
                let idl_blob_parts = Vec::from_iter(path.split("."));
                if idl_blob_parts.len() == 1 {
                    return idl_map_get_key_or_else(
                        &transaction_instruction.accounts_addresses,
                        path,
                        &breadcrumbs.val(),
                    )
                    .map(|address| address.to_bytes().to_vec());
                }
                let transaction_instruction_account_name = idl_blob_parts[0];
                let transaction_instruction_account = idl_map_get_key_or_else(
                    transaction_instruction_accounts,
                    idl_blob_parts[0],
                    &breadcrumbs.as_val("transaction_instruction_accounts"),
                )?;
                let program_account = idl_map_get_key_or_else(
                    &program_accounts,
                    &transaction_instruction_account.name,
                    &breadcrumbs.as_idl("$program_accounts"),
                )?;
                ToolboxIdlProgramInstructionAccountBlob::try_resolve_path_data(
                    &program_account.data_type_full,
                    &transaction_instruction_account.state,
                    &idl_blob_parts[1..],
                    &breadcrumbs
                        .with_idl(&program_account.name)
                        .with_val(transaction_instruction_account_name),
                )
            },
            ToolboxIdlProgramInstructionAccountBlob::Arg { path } => {
                let idl_blob_parts = Vec::from_iter(path.split("."));
                ToolboxIdlProgramInstructionAccountBlob::try_resolve_path_data(
                    &program_instruction.data_type_full,
                    &transaction_instruction.args,
                    &idl_blob_parts,
                    &breadcrumbs
                        .with_idl(&transaction_instruction.name)
                        .with_idl("args"),
                )
            },
        }
    }

    fn try_resolve_path_data(
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
                return ToolboxIdlProgramInstructionAccountBlob::try_resolve_path_data(
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
