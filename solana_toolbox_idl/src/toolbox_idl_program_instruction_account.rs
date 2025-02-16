use std::str::FromStr;

use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_instruction_account_pda::ToolboxIdlProgramInstructionAccountPda;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstructionAccount {
    pub index: usize,
    pub name: String,
    pub is_writable: bool,
    pub is_signer: bool,
    pub address: Option<Pubkey>,
    pub pda: Option<ToolboxIdlProgramInstructionAccountPda>,
    // TODO - support is_optional ?
}

impl ToolboxIdlProgramInstructionAccount {
    pub(crate) fn try_parse(
        idl_instruction_account_index: usize,
        idl_instruction_account: &Value,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionAccount, ToolboxIdlError> {
        let idl_instruction_account =
            idl_as_object_or_else(idl_instruction_account, &breadcrumbs.idl())?;
        let idl_instruction_account_name = idl_object_get_key_as_str_or_else(
            idl_instruction_account,
            "name",
            &breadcrumbs.idl(),
        )?;
        let breadcrumbs = &breadcrumbs.with_idl(idl_instruction_account_name);
        let idl_instruction_account_is_writable =
            idl_object_get_key_as_bool(idl_instruction_account, "writable")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isMut",
                ))
                .unwrap_or(false);
        let idl_instruction_account_is_signer =
            idl_object_get_key_as_bool(idl_instruction_account, "signer")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isSigner",
                ))
                .unwrap_or(false);
        Ok(ToolboxIdlProgramInstructionAccount {
            index: idl_instruction_account_index + 1,
            name: idl_instruction_account_name.to_string(),
            is_writable: idl_instruction_account_is_writable,
            is_signer: idl_instruction_account_is_signer,
            address: ToolboxIdlProgramInstructionAccount::try_parse_address(
                idl_instruction_account,
                breadcrumbs,
            )?,
            pda: ToolboxIdlProgramInstructionAccount::try_parse_pda(
                idl_instruction_account,
                breadcrumbs,
            )?,
        })
    }

    fn try_parse_address(
        idl_instruction_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Option<Pubkey>, ToolboxIdlError> {
        let idl_instruction_account_address =
            match idl_object_get_key_as_str(idl_instruction_account, "address")
            {
                None => return Ok(None),
                Some(val) => val,
            };
        Ok(Some(Pubkey::from_str(idl_instruction_account_address).map_err(
            |err| {
                ToolboxIdlError::InvalidPubkey {
                    parsing: err,
                    context: breadcrumbs.as_idl("address"),
                }
            },
        )?))
    }

    fn try_parse_pda(
        idl_instruction_account: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Option<ToolboxIdlProgramInstructionAccountPda>, ToolboxIdlError>
    {
        let idl_instruction_account_pda = match idl_object_get_key_as_object(
            idl_instruction_account,
            "pda",
        ) {
            None => return Ok(None),
            Some(val) => val,
        };
        ToolboxIdlProgramInstructionAccountPda::try_parse(
            idl_instruction_account_pda,
            breadcrumbs,
        )
    }
}
