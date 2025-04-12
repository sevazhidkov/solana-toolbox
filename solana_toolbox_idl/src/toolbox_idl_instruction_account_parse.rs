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
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFullFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;

impl ToolboxIdlInstructionAccount {
    pub fn try_parse(
        idl_instruction_account: &Value,
        args: &ToolboxIdlTypeFullFields,
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
            args,
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
        args: &ToolboxIdlTypeFullFields,
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
            for (index, idl_instruction_account_pda_seed) in
                idl_instruction_account_pda_seeds.iter().enumerate()
            {
                seeds.push(
                    ToolboxIdlInstructionAccount::try_parse_pda_blob(
                        idl_instruction_account_pda_seed,
                        args,
                        accounts,
                    )
                    .context(index)
                    .context("Seeds")?,
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
                    args,
                    accounts,
                )
                .context("Program")?,
            );
        }
        Ok(Some(ToolboxIdlInstructionAccountPda { seeds, program }))
    }

    fn try_parse_pda_blob(
        idl_instruction_account_pda_blob: &Value,
        args: &ToolboxIdlTypeFullFields,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_object()
        {
            if let Some(idl_instruction_account_pda_blob_value) =
                idl_instruction_account_pda_blob.get("value")
            {
                let idl_instruction_account_pda_blob_type =
                    idl_instruction_account_pda_blob
                        .get("type")
                        .cloned()
                        .unwrap_or_else(|| json!("bytes"));
                return ToolboxIdlInstructionAccount::try_parse_pda_blob_type_value(
                    &idl_instruction_account_pda_blob_type,
                    idl_instruction_account_pda_blob_value.clone()
                ).context("Blob object value");
            }
            let idl_instruction_account_pda_blob_path =
                idl_object_get_key_as_str_or_else(
                    idl_instruction_account_pda_blob,
                    "path",
                )?;
            if idl_object_get_key_as_str(
                idl_instruction_account_pda_blob,
                "kind",
            ) == Some("arg")
            {
                return ToolboxIdlInstructionAccount::try_parse_pda_blob_arg_path(
                        idl_instruction_account_pda_blob_path,
                        args,
                    ).context("Blob arg");
            }
            let idl_instruction_account_pda_blob_account =
                idl_object_get_key_as_str(
                    idl_instruction_account_pda_blob,
                    "account",
                );
            return ToolboxIdlInstructionAccount::try_parse_pda_blob_account_path(
                idl_instruction_account_pda_blob_path,
                &idl_instruction_account_pda_blob_account,
                accounts
            ).context("Blob account");
        }
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_array()
        {
            return ToolboxIdlInstructionAccount::try_parse_pda_blob_type_value(
                &json!("bytes"),
                json!(idl_instruction_account_pda_blob),
            ).context("Blob array");
        }
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_str()
        {
            return ToolboxIdlInstructionAccount::try_parse_pda_blob_type_value(
                &json!("string"),
                json!(idl_instruction_account_pda_blob),
            ).context("Blob string");
        }
        Err(anyhow!(
            "Could not parse blob bytes (expected an object/array/string)"
        ))
    }

    fn try_parse_pda_blob_type_value(
        idl_instruction_account_pda_blob_type: &Value,
        idl_instruction_account_pda_blob_value: Value,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        let type_flat = ToolboxIdlTypeFlat::try_parse_value(
            idl_instruction_account_pda_blob_type,
        )
        .context("Const type parse")?;
        let type_full = type_flat
            .try_hydrate(&HashMap::new(), &HashMap::new())
            .context("Const type hydrate")?;
        Ok(ToolboxIdlInstructionAccountPdaBlob::Const {
            type_flat,
            type_full,
            value: idl_instruction_account_pda_blob_value,
        })
    }

    fn try_parse_pda_blob_arg_path(
        idl_instruction_account_pda_blob_path: &str,
        args: &ToolboxIdlTypeFullFields,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        let path =
            ToolboxIdlPath::try_parse(idl_instruction_account_pda_blob_path)?;
        let type_full = path
            .try_get_type_full_fields(args)
            .context("Extract arg type")?;
        let type_flat = type_full.flattened();
        Ok(ToolboxIdlInstructionAccountPdaBlob::Arg {
            path,
            type_flat,
            type_full,
        })
    }

    fn try_parse_pda_blob_account_path(
        idl_instruction_account_pda_blob_path: &str,
        idl_instruction_account_pda_blob_account: &Option<&str>,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        let path =
            ToolboxIdlPath::try_parse(idl_instruction_account_pda_blob_path)
                .context("Parse path")?;
        let (instruction_account_part, account_content_path) =
            path.split_first().context("Empty path")?;
        let instruction_account_name = instruction_account_part
            .key()
            .context("Path account name")?
            .to_string();
        let account = idl_instruction_account_pda_blob_account
            .map(|account| account.to_string());
        let type_full = idl_instruction_account_pda_blob_account
            .and_then(|account| accounts.get(account))
            .and_then(|account| {
                if account_content_path.is_empty() {
                    None
                } else {
                    Some(
                        account_content_path
                            .try_get_type_full(&account.content_type_full)
                            .context("Extract account content type"),
                    )
                }
            })
            .transpose()?
            .unwrap_or(ToolboxIdlTypeFull::Primitive {
                primitive: ToolboxIdlTypePrimitive::PublicKey,
            });
        let type_flat = type_full.flattened();
        Ok(ToolboxIdlInstructionAccountPdaBlob::Account {
            path,
            instruction_account_name,
            account_content_path,
            account,
            type_flat,
            type_full,
        })
    }
}
