use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use serde_json::json;
use serde_json::Map;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPda;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccountPdaBlob;
use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
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
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
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
        )
        .context("Address")?;
        let pda = ToolboxIdlInstructionAccount::try_parse_pda(
            idl_instruction_account,
            accounts,
        )
        .context("Pda")?;
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
        match idl_object_get_key_as_str(idl_instruction_account, "address") {
            None => Ok(None),
            Some(val) => Ok(Some(Pubkey::from_str(val)?)),
        }
    }

    fn try_parse_pda(
        idl_instruction_account: &Map<String, Value>,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
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
                        accounts,
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
                    accounts,
                )
                .context("Program")?,
            );
        }
        Ok(Some(ToolboxIdlInstructionAccountPda { seeds, program }))
    }

    pub fn try_parse_pda_blob(
        idl_instruction_account_pda_blob: &Value,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_object()
        {
            if let Some(idl_instruction_account_pda_blob_value) =
                idl_instruction_account_pda_blob.get("value")
            {
                let mut bytes = vec![];
                ToolboxIdlTypeFlat::try_parse_value(
                    &idl_instruction_account_pda_blob
                        .get("type")
                        .cloned()
                        .unwrap_or_else(|| json!("bytes")),
                )
                .context("Const type parse")?
                .try_hydrate(&HashMap::new(), &HashMap::new())
                .context("Const type hydrate")?
                .try_serialize(
                    idl_instruction_account_pda_blob_value,
                    &mut bytes,
                    false,
                )
                .context("Const type serialize")?;
                return Ok(ToolboxIdlInstructionAccountPdaBlob::Const {
                    bytes,
                });
            }
            let idl_instruction_account_pda_blob_kind =
                idl_object_get_key_as_str(
                    idl_instruction_account_pda_blob,
                    "kind",
                );
            let idl_instruction_account_pda_blob_path =
                idl_object_get_key_as_str_or_else(
                    idl_instruction_account_pda_blob,
                    "path",
                )?;
            if idl_instruction_account_pda_blob_kind == Some("arg") {
                return Ok(ToolboxIdlInstructionAccountPdaBlob::Arg {
                    path: ToolboxIdlPath::try_parse(
                        idl_instruction_account_pda_blob_path,
                    ),
                });
            }
            return Ok(ToolboxIdlInstructionAccountPdaBlob::Account {
                path: ToolboxIdlPath::try_parse(
                    idl_instruction_account_pda_blob_path,
                ),
                account: idl_object_get_key_as_str(
                    idl_instruction_account_pda_blob,
                    "account",
                )
                .map(|account_name| accounts.get(account_name))
                .flatten()
                .cloned(),
            });
        }
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
        Err(anyhow!(
            "Could not parse blob bytes (expected an object/array/string)"
        ))
    }
}
