use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_account::ToolboxIdlAccount;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_hash_discriminator_from_string;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;
use crate::toolbox_idl_utils::idl_value_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_value_as_object_or_else;

impl ToolboxIdlInstruction {
    pub fn try_parse(
        idl_instruction_name: &str,
        idl_instruction: &Value,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlInstruction> {
        let idl_instruction = idl_value_as_object_or_else(idl_instruction)?;
        let docs = idl_instruction.get("docs").cloned();
        let discriminator = ToolboxIdlInstruction::try_parse_discriminator(
            idl_instruction_name,
            idl_instruction,
        )
        .context("Discriminator")?;
        let args_type_flat_fields =
            ToolboxIdlInstruction::try_parse_args_type_flat_fields(
                idl_instruction,
            )
            .context("Parse Args Type")?;
        let args_type_full_fields = args_type_flat_fields
            .try_hydrate(&HashMap::new(), typedefs)
            .context("Hydrate Args Type")?;
        let return_type_flat =
            ToolboxIdlInstruction::try_parse_return_type_flat(idl_instruction)
                .context("Parse Returns Type")?;
        let return_type_full = return_type_flat
            .try_hydrate(&HashMap::new(), typedefs)
            .context("Hydrate Returns Type")?;
        let accounts = ToolboxIdlInstruction::try_parse_accounts(
            idl_instruction,
            &args_type_flat_fields,
            accounts,
            typedefs,
        )
        .context("Parse Accounts")?;
        Ok(ToolboxIdlInstruction {
            name: idl_instruction_name.to_string(),
            docs,
            discriminator,
            accounts,
            args_type_flat_fields,
            args_type_full_fields,
            return_type_flat,
            return_type_full,
        })
    }

    fn try_parse_discriminator(
        idl_instruction_name: &str,
        idl_instruction: &Map<String, Value>,
    ) -> Result<Vec<u8>> {
        if let Some(idl_instruction_discriminator) =
            idl_instruction.get("discriminator")
        {
            return idl_value_as_bytes_or_else(idl_instruction_discriminator);
        }
        Ok(idl_hash_discriminator_from_string(&format!(
            "global:{}",
            idl_instruction_name
        )))
    }

    fn try_parse_accounts(
        idl_instruction: &Map<String, Value>,
        args_type_flat_fields: &ToolboxIdlTypeFlatFields,
        accounts: &HashMap<String, Arc<ToolboxIdlAccount>>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<Vec<ToolboxIdlInstructionAccount>> {
        let idl_instruction_accounts_array =
            idl_object_get_key_as_array_or_else(idl_instruction, "accounts")?;
        let mut instruction_accounts = vec![];
        for (index, idl_instruction_account) in
            idl_instruction_accounts_array.iter().enumerate()
        {
            instruction_accounts.push(
                ToolboxIdlInstructionAccount::try_parse(
                    idl_instruction_account,
                    args_type_flat_fields,
                    accounts,
                    typedefs,
                )
                .with_context(|| format!("Parse Account: {}", index))?,
            );
        }
        Ok(instruction_accounts)
    }

    fn try_parse_args_type_flat_fields(
        idl_instruction: &Map<String, Value>,
    ) -> Result<ToolboxIdlTypeFlatFields> {
        if let Some(idl_instruction_args) =
            idl_object_get_key_as_array(idl_instruction, "args")
        {
            return ToolboxIdlTypeFlatFields::try_parse(idl_instruction_args);
        }
        Ok(ToolboxIdlTypeFlatFields::nothing())
    }

    fn try_parse_return_type_flat(
        idl_instruction: &Map<String, Value>,
    ) -> Result<ToolboxIdlTypeFlat> {
        if let Some(idl_instruction_returns) = idl_instruction.get("returns") {
            return ToolboxIdlTypeFlat::try_parse(idl_instruction_returns);
        }
        Ok(ToolboxIdlTypeFlat::nothing())
    }
}
