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
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_type_primitive::ToolboxIdlTypePrimitive;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_convert_to_snake_case;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_bool;
use crate::toolbox_idl_utils::idl_object_get_key_as_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_object_get_key_as_str_or_else;

impl ToolboxIdlInstructionAccount {
    pub fn try_parse(
        idl_instruction_account: &Value,
        args_type_flat_fields: &ToolboxIdlTypeFlatFields,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionAccount> {
        let idl_instruction_account =
            idl_as_object_or_else(idl_instruction_account)?;
        let name = idl_convert_to_snake_case(
            idl_object_get_key_as_str_or_else(idl_instruction_account, "name")
                .context("Parse Name")?,
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
        .with_context(|| format!("Parse {} Address", name))?;
        let pda = ToolboxIdlInstructionAccount::try_parse_pda(
            idl_instruction_account,
            args_type_flat_fields,
            accounts,
            typedefs,
        )
        .with_context(|| format!("Parse {} Pda", name))?;
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
        args_type_flat_fields: &ToolboxIdlTypeFlatFields,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
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
                        args_type_flat_fields,
                        accounts,
                        typedefs,
                    )
                    .with_context(|| format!("Seed: {}", index))?,
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
                    args_type_flat_fields,
                    accounts,
                    typedefs,
                )
                .context("Program Id")?,
            );
        }
        Ok(Some(ToolboxIdlInstructionAccountPda { seeds, program }))
    }

    fn try_parse_pda_blob(
        idl_instruction_account_pda_blob: &Value,
        args_type_flat_fields: &ToolboxIdlTypeFlatFields,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
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
                    idl_instruction_account_pda_blob_value.clone(),
                    typedefs,
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
                    args_type_flat_fields,
                    typedefs,
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
                accounts,
                typedefs,
            ).context("Blob account");
        }
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_array()
        {
            return ToolboxIdlInstructionAccount::try_parse_pda_blob_type_value(
                &json!("bytes"),
                json!(idl_instruction_account_pda_blob),
                typedefs,
            ).context("Blob array");
        }
        if let Some(idl_instruction_account_pda_blob) =
            idl_instruction_account_pda_blob.as_str()
        {
            return ToolboxIdlInstructionAccount::try_parse_pda_blob_type_value(
                &json!("string"),
                json!(idl_instruction_account_pda_blob),
                typedefs,
            ).context("Blob string");
        }
        Err(anyhow!(
            "Could not parse blob bytes (expected an object/array/string)"
        ))
    }

    fn try_parse_pda_blob_type_value(
        idl_instruction_account_pda_blob_type: &Value,
        idl_instruction_account_pda_blob_value: Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        let type_flat = ToolboxIdlTypeFlat::try_parse(
            idl_instruction_account_pda_blob_type,
        )
        .context("Const type parse")?;
        let type_full = type_flat
            .try_hydrate(&HashMap::new(), typedefs)
            .context("Const type hydrate")?;
        Ok(ToolboxIdlInstructionAccountPdaBlob::Const {
            type_flat,
            type_full,
            value: idl_instruction_account_pda_blob_value,
        })
    }

    fn try_parse_pda_blob_arg_path(
        idl_instruction_account_pda_blob_path: &str,
        args_type_flat_fields: &ToolboxIdlTypeFlatFields,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstructionAccountPdaBlob> {
        let path =
            ToolboxIdlPath::try_parse(idl_instruction_account_pda_blob_path)?;
        let type_flat = path
            .try_get_type_flat_fields(
                args_type_flat_fields,
                &HashMap::new(),
                typedefs,
            )
            .context("Extract arg type")?;
        let type_full = type_flat
            .try_hydrate(&HashMap::new(), typedefs)
            .context("Hydrate arg type")?;
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
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
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
        let type_flat = idl_instruction_account_pda_blob_account
            .and_then(|account| accounts.get(account))
            .and_then(|account| {
                if account_content_path.is_empty() {
                    None
                } else {
                    Some(
                        account_content_path
                            .try_get_type_flat(
                                &account.content_type_flat,
                                &HashMap::new(),
                                typedefs,
                            )
                            .context("Extract account content type"),
                    )
                }
            })
            .transpose()?
            .unwrap_or(ToolboxIdlTypePrimitive::PublicKey.into());
        let type_full = type_flat
            .try_hydrate(&HashMap::new(), typedefs)
            .context("Hydrate account content type")?;
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
