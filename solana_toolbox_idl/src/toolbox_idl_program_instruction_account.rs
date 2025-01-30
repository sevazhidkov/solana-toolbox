use std::str::FromStr;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_array_get_scoped_object_array_or_else;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_err;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstructionAccount {
    pub name: String,
    pub is_writable: bool,
    pub is_signer: bool,
    pub resolve: ToolboxIdlProgramInstructionAccountResolve,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramInstructionAccountResolve {
    Address(Pubkey),
    Pda {
        seeds: Vec<ToolboxIdlProgramInstructionAccountResolvePdaBlob>,
        program: Option<ToolboxIdlProgramInstructionAccountResolvePdaBlob>,
    },
    Unresolvable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToolboxIdlProgramInstructionAccountResolvePdaBlob {
    Const { bytes: Vec<u8> },
    Account { path: String },
    Arg { path: String },
}

impl ToolboxIdlProgramInstructionAccount {
    pub(crate) fn try_parse(
        idl_instruction_account_name: &str,
        idl_instruction_account_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccount, ToolboxIdlError> {
        let idl_instruction_account_is_writable = idl_object_get_key_as_bool(
            idl_instruction_account_object,
            "writable",
        )
        .or(idl_object_get_key_as_bool(idl_instruction_account_object, "isMut"))
        .unwrap_or(false);
        let idl_instruction_account_is_signer = idl_object_get_key_as_bool(
            idl_instruction_account_object,
            "signer",
        )
        .or(idl_object_get_key_as_bool(
            idl_instruction_account_object,
            "isSigner",
        ))
        .unwrap_or(false);
        Ok(ToolboxIdlProgramInstructionAccount {
            name: idl_instruction_account_name.to_string(),
            is_writable: idl_instruction_account_is_writable,
            is_signer: idl_instruction_account_is_signer,
            resolve: ToolboxIdlProgramInstructionAccount::try_parse_resolve(
                idl_instruction_account_object,
                breadcrumbs,
            )?,
        })
    }

    fn try_parse_resolve(
        idl_instruction_account_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccountResolve, ToolboxIdlError>
    {
        if let Some(idl_instruction_account_address) =
            idl_object_get_key_as_str(idl_instruction_account_object, "address")
        {
            return Pubkey::from_str(idl_instruction_account_address)
                .map_err(|err| ToolboxIdlError::InvalidPubkey {
                    parsing: err,
                    context: breadcrumbs.as_idl("address"),
                })
                .map(ToolboxIdlProgramInstructionAccountResolve::Address);
        }
        if let Some(idl_instruction_account_pda) =
            idl_object_get_key_as_object(idl_instruction_account_object, "pda")
        {
            let breadcrumbs = &breadcrumbs.with_idl("pda");
            let mut program_instruction_account_resolve_pda_seeds = vec![];
            if let Some(idl_instruction_account_pda_seeds_array) =
                idl_object_get_key_as_array(
                    idl_instruction_account_pda,
                    "seeds",
                )
            {
                for (idl_instruction_account_pda_seed_object, breadcrumbs) in
                    idl_array_get_scoped_object_array_or_else(
                        idl_instruction_account_pda_seeds_array,
                        breadcrumbs,
                    )?
                {
                    program_instruction_account_resolve_pda_seeds.push(ToolboxIdlProgramInstructionAccount::try_parse_resolve_pda_blob(
                        idl_instruction_account_pda_seed_object,
                        &breadcrumbs,
                    )?);
                }
            }
            let idl_instruction_account_pda_program_object =
                idl_object_get_key_as_object(
                    idl_instruction_account_pda,
                    "program",
                );
            let program_instruction_account_resolve_pda_program = idl_instruction_account_pda_program_object
                .map(|idl_instruction_account_pda_program_object| {
                    ToolboxIdlProgramInstructionAccount::try_parse_resolve_pda_blob(
                        idl_instruction_account_pda_program_object,
                        &breadcrumbs.with_idl("program"),
                    )
                })
                .transpose()?;
            return Ok(ToolboxIdlProgramInstructionAccountResolve::Pda {
                seeds: program_instruction_account_resolve_pda_seeds,
                program: program_instruction_account_resolve_pda_program,
            });
        }
        Ok(ToolboxIdlProgramInstructionAccountResolve::Unresolvable)
    }

    fn try_parse_resolve_pda_blob(
        idl_instruction_account_pda_blob_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<
        ToolboxIdlProgramInstructionAccountResolvePdaBlob,
        ToolboxIdlError,
    > {
        let idl_instruction_account_pda_blob_kind =
            idl_object_get_key_as_str_or_else(
                idl_instruction_account_pda_blob_object,
                "kind",
                &breadcrumbs.as_idl("datadef"),
            )?;
        match idl_instruction_account_pda_blob_kind {
            "const" => ToolboxIdlProgramInstructionAccount::try_parse_resolve_pda_blob_const(
                idl_instruction_account_pda_blob_object,
                &breadcrumbs.with_idl("const"),
            ),
            "account" => ToolboxIdlProgramInstructionAccount::try_parse_resolve_pda_blob_account(
                idl_instruction_account_pda_blob_object,
                &breadcrumbs.with_idl("account"),
            ),
            "arg" => ToolboxIdlProgramInstructionAccount::try_parse_resolve_pda_blob_arg(
                idl_instruction_account_pda_blob_object,
                &breadcrumbs.with_idl("arg"),
            ),
            _ => idl_err(
                "Expected a 'kind' value of: const/account/arg",
                &breadcrumbs.as_idl(idl_instruction_account_pda_blob_kind),
            ),
        }
    }

    fn try_parse_resolve_pda_blob_const(
        idl_instruction_account_pda_blob_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<
        ToolboxIdlProgramInstructionAccountResolvePdaBlob,
        ToolboxIdlError,
    > {
        let idl_instruction_account_pda_blob_value =
            idl_object_get_key_or_else(
                idl_instruction_account_pda_blob_object,
                "value",
                &breadcrumbs.idl(),
            )?;
        if let Some(idl_instruction_account_pda_blob_value_string) =
            idl_instruction_account_pda_blob_value.as_str()
        {
            return Ok(
                ToolboxIdlProgramInstructionAccountResolvePdaBlob::Const {
                    bytes: idl_instruction_account_pda_blob_value_string
                        .as_bytes()
                        .to_vec(),
                },
            );
        }
        if idl_instruction_account_pda_blob_value.is_array() {
            return Ok(
                ToolboxIdlProgramInstructionAccountResolvePdaBlob::Const {
                    bytes: idl_as_bytes_or_else(
                        idl_instruction_account_pda_blob_value,
                        &breadcrumbs.idl(),
                    )?,
                },
            );
        }
        idl_err(
            "Expected an array of string as value",
            &breadcrumbs.as_idl("datadef(const)"),
        )
    }

    fn try_parse_resolve_pda_blob_account(
        idl_instruction_account_pda_blob_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<
        ToolboxIdlProgramInstructionAccountResolvePdaBlob,
        ToolboxIdlError,
    > {
        let idl_instruction_account_pda_blob_path =
            idl_object_get_key_as_str_or_else(
                idl_instruction_account_pda_blob_object,
                "path",
                &breadcrumbs.idl(),
            )?;
        Ok(ToolboxIdlProgramInstructionAccountResolvePdaBlob::Account {
            path: idl_instruction_account_pda_blob_path.to_string(),
        })
    }

    fn try_parse_resolve_pda_blob_arg(
        idl_instruction_account_pda_blob_object: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<
        ToolboxIdlProgramInstructionAccountResolvePdaBlob,
        ToolboxIdlError,
    > {
        let idl_instruction_account_pda_blob_path =
            idl_object_get_key_as_str_or_else(
                idl_instruction_account_pda_blob_object,
                "path",
                &breadcrumbs.idl(),
            )?;
        Ok(ToolboxIdlProgramInstructionAccountResolvePdaBlob::Arg {
            path: idl_instruction_account_pda_blob_path.to_string(),
        })
    }
}
