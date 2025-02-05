use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramInstructionAccountBlob {
    Const { bytes: Vec<u8> },
    Arg { path: String },
    Account { path: String },
}

impl ToolboxIdlProgramInstructionAccountBlob {
    pub(crate) fn try_parse(
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
            "account" => {
                Ok(ToolboxIdlProgramInstructionAccountBlob::Account {
                    path: idl_instruction_account_blob_path.to_string(),
                })
            },
            "arg" => {
                Ok(ToolboxIdlProgramInstructionAccountBlob::Arg {
                    path: idl_instruction_account_blob_path.to_string(),
                })
            },
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
}
