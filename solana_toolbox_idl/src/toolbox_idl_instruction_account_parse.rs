use std::str::FromStr;

use anyhow::anyhow;
use anyhow::Context;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use anyhow::Result;

use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;

impl ToolboxIdlInstructionAccount {
    pub fn try_parse(
        idl_instruction_account: &Value,
    ) -> Result<ToolboxIdlInstructionAccount> {
        let idl_instruction_account =
            idl_as_object_or_else(idl_instruction_account)?;
        let name = ToolboxIdlInstructionAccount::sanitize_name(
            idl_object_get_key_as_str_or_else(idl_instruction_account, "name")?,
        );
        let docs = idl_instruction_account.get("docs").cloned();
        let writable =
            idl_object_get_key_as_bool(idl_instruction_account, "writable")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isMut",
                ))
                .unwrap_or(false);
        let signer =
            idl_object_get_key_as_bool(idl_instruction_account, "signer")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isSigner",
                ))
                .unwrap_or(false);
        let optional =
            idl_object_get_key_as_bool(idl_instruction_account, "optional")
                .or(idl_object_get_key_as_bool(
                    idl_instruction_account,
                    "isOptional",
                ))
                .unwrap_or(false);
        let address = ToolboxIdlInstructionAccount::try_parse_address(
            idl_instruction_account,
        )?;
        let pda = ToolboxIdlInstructionAccount::try_parse_pda(
            idl_instruction_account,
        )?;
        Ok(ToolboxIdlInstructionAccount {
            name,
            docs,
            writable,
            signer,
            optional,
            address,
            pda,
        })
    }

    fn try_parse_address(
        idl_instruction_account: &Map<String, Value>,
    ) -> Result<Option<Pubkey>> {
        let idl_instruction_account_address =
            match idl_object_get_key_as_str(idl_instruction_account, "address")
            {
                None => return Ok(None),
                Some(val) => val,
            };
        Ok(Some(Pubkey::from_str(idl_instruction_account_address)?))
    }

    fn try_parse_pda(
        idl_instruction_account: &Map<String, Value>,
    ) -> Result<Option<ToolboxIdlInstructionAccountPda>> {
        let idl_instruction_account_pda = match idl_object_get_key_as_object(
            idl_instruction_account,
            "pda",
        ) {
            None => return Ok(None),
            Some(val) => val,
        };
        let mut seeds = vec![];
        if let Some(idl_instruction_account_pda_seeds) =
            idl_object_get_key_as_array(idl_instruction_account_pda, "seeds")
        {
            for (_, idl_instruction_account_pda_seed, context) in
                idl_iter_get_scoped_values(idl_instruction_account_pda_seeds)
            {
                seeds.push(
                    ToolboxIdlInstructionAccount::try_parse_pda_blob(
                        idl_instruction_account_pda_seed,
                    )
                    .context(context)?,
                );
            }
        }
        let mut program = None;
        if let Some(idl_instruction_account_pda_program) =
            idl_instruction_account_pda.get("program")
        {
            program = Some(
                ToolboxIdlInstructionAccount::try_parse_pda_blob(
                    idl_instruction_account_pda_program,
                )
                .context("Program ID")?,
            );
        }
        Ok(Some(ToolboxIdlInstructionAccountPda { seeds, program }))
    }

    pub fn try_parse_pda_blob(
        idl_instruction_account_pda_blob: &Value,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_object()
        {
            if let Some(idl_instruction_account_pda_blob_value) =
                idl_instruction_account_pda_blob.get("value")
            {
                return ToolboxIdlInstructionAccount::try_parse_pda_blob_const(
                    idl_instruction_account_pda_blob_value,
                );
            }
            let idl_instruction_account_pda_blob_kind =
                idl_object_get_key_as_str_or_else(
                    idl_instruction_account_pda_blob,
                    "kind",
                )?;
            let idl_instruction_account_pda_blob_path =
                idl_object_get_key_as_str_or_else(
                    idl_instruction_account_pda_blob,
                    "path",
                )?;
            return match idl_instruction_account_pda_blob_kind {
                "arg" => Ok(ToolboxIdlInstructionAccountPdaBlob::Arg {
                    path: idl_instruction_account_pda_blob_path.to_string(),
                }),
                "account" => Ok(ToolboxIdlInstructionAccountPdaBlob::Account {
                    path: idl_instruction_account_pda_blob_path.to_string(),
                }),
                _ => Err(anyhow!(
                    "Unknown blob kind: {}",
                    idl_instruction_account_pda_blob_kind
                )),
            };
        }
        ToolboxIdlInstructionAccount::try_parse_pda_blob_const(
            idl_instruction_account_pda_blob,
        )
    }

    fn try_parse_pda_blob_const(
        idl_instruction_account_pda_blob: &Value,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_array()
        {
            return Ok(ToolboxIdlInstructionAccountPdaBlob::Const {
                bytes: idl_as_bytes_or_else(idl_instruction_account_pda_blob)?,
            });
        }
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_str()
        {
            return Ok(ToolboxIdlInstructionAccountPdaBlob::Const {
                bytes: idl_instruction_account_pda_blob.as_bytes().to_vec(),
            });
        }
        Err(anyhow!("Could not parse blob bytes"))
    }
}
