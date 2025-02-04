use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_array_get_scoped_object_array_or_else;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstructionAccountPda {
    pub seeds: Vec<ToolboxIdlProgramInstructionAccountPdaBlob>,
    pub program: Option<ToolboxIdlProgramInstructionAccountPdaBlob>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramInstructionAccountPdaBlob {
    Const { bytes: Vec<u8> },
    Account { path: String },
    Arg { path: String },
}

impl ToolboxIdlProgramInstructionAccountPda {
    pub(crate) fn try_parse(
        idl_instruction_account_pda: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Option<ToolboxIdlProgramInstructionAccountPda>, ToolboxIdlError>
    {
        let mut pda_seeds = vec![];
        if let Some(idl_instruction_account_pda_seeds) =
            idl_object_get_key_as_array(idl_instruction_account_pda, "seeds")
        {
            for (idl_instruction_account_pda_seed, breadcrumbs) in
                idl_array_get_scoped_object_array_or_else(
                    idl_instruction_account_pda_seeds,
                    breadcrumbs,
                )?
            {
                pda_seeds.push(
                    ToolboxIdlProgramInstructionAccountPda::try_parse_blob(
                        idl_instruction_account_pda_seed,
                        &breadcrumbs,
                    )?,
                );
            }
        }
        let pda_program = idl_object_get_key_as_object(
            idl_instruction_account_pda,
            "program",
        )
        .map(|idl_instruction_account_pda_program| {
            ToolboxIdlProgramInstructionAccountPda::try_parse_blob(
                idl_instruction_account_pda_program,
                &breadcrumbs.with_idl("program"),
            )
        })
        .transpose()?;
        Ok(Some(ToolboxIdlProgramInstructionAccountPda {
            seeds: pda_seeds,
            program: pda_program,
        }))
    }

    fn try_parse_blob(
        idl_instruction_account_pda_blob: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccountPdaBlob, ToolboxIdlError>
    {
        // TODO - we could remove the lookup to "kind" and provide a shortcut
        let idl_instruction_account_pda_blob_kind =
            idl_object_get_key_as_str_or_else(
                idl_instruction_account_pda_blob,
                "kind",
                &breadcrumbs.as_idl("datadef"),
            )?;
        match idl_instruction_account_pda_blob_kind {
            "const" => {
                ToolboxIdlProgramInstructionAccountPda::try_parse_blob_const(
                    idl_instruction_account_pda_blob,
                    &breadcrumbs.with_idl("const"),
                )
            },
            "account" => {
                ToolboxIdlProgramInstructionAccountPda::try_parse_blob_account(
                    idl_instruction_account_pda_blob,
                    &breadcrumbs.with_idl("account"),
                )
            },
            "arg" => {
                ToolboxIdlProgramInstructionAccountPda::try_parse_blob_arg(
                    idl_instruction_account_pda_blob,
                    &breadcrumbs.with_idl("arg"),
                )
            },
            _ => {
                idl_err(
                    "Expected a 'kind' value of: const/account/arg",
                    &breadcrumbs.as_idl(idl_instruction_account_pda_blob_kind),
                )
            },
        }
    }

    fn try_parse_blob_const(
        idl_instruction_account_pda_blob: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccountPdaBlob, ToolboxIdlError>
    {
        let idl_instruction_account_pda_blob = idl_object_get_key_or_else(
            idl_instruction_account_pda_blob,
            "value",
            &breadcrumbs.idl(),
        )?;
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_str()
        {
            return Ok(ToolboxIdlProgramInstructionAccountPdaBlob::Const {
                bytes: idl_instruction_account_pda_blob.as_bytes().to_vec(),
            });
        }
        if idl_instruction_account_pda_blob.is_array() {
            return Ok(ToolboxIdlProgramInstructionAccountPdaBlob::Const {
                bytes: idl_as_bytes_or_else(
                    idl_instruction_account_pda_blob,
                    &breadcrumbs.idl(),
                )?,
            });
        }
        idl_err(
            "Expected an array or string",
            &breadcrumbs.as_idl("datadef(const)"),
        )
    }

    fn try_parse_blob_account(
        idl_instruction_account_pda_blob: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccountPdaBlob, ToolboxIdlError>
    {
        let idl_instruction_account_pda_blob_path =
            idl_object_get_key_as_str_or_else(
                idl_instruction_account_pda_blob,
                "path",
                &breadcrumbs.idl(),
            )?;
        Ok(ToolboxIdlProgramInstructionAccountPdaBlob::Account {
            path: idl_instruction_account_pda_blob_path.to_string(),
        })
    }

    fn try_parse_blob_arg(
        idl_instruction_account_pda_blob: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccountPdaBlob, ToolboxIdlError>
    {
        let idl_instruction_account_pda_blob_path =
            idl_object_get_key_as_str_or_else(
                idl_instruction_account_pda_blob,
                "path",
                &breadcrumbs.idl(),
            )?;
        Ok(ToolboxIdlProgramInstructionAccountPdaBlob::Arg {
            path: idl_instruction_account_pda_blob_path.to_string(),
        })
    }
}
