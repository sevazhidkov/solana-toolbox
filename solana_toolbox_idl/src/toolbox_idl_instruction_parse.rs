use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_instruction::ToolboxIdlInstruction;
use crate::toolbox_idl_instruction_account::ToolboxIdlInstructionAccount;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_as_object_or_else;
use crate::toolbox_idl_utils::idl_convert_to_value_name;
use crate::toolbox_idl_utils::idl_hash_discriminator_from_string;
use crate::toolbox_idl_utils::idl_iter_get_scoped_values;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_object_get_key_as_array_or_else;

impl ToolboxIdlInstruction {
    pub fn try_parse(
        idl_instruction_name: &str,
        idl_instruction: &Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlInstruction, ToolboxIdlError> {
        let idl_instruction =
            idl_as_object_or_else(idl_instruction, &breadcrumbs.idl())?;
        let instruction_discriminator =
            ToolboxIdlInstruction::try_parse_discriminator(
                idl_instruction_name,
                idl_instruction,
                breadcrumbs,
            )?;
        let instruction_docs = idl_instruction.get("docs").cloned();
        let instruction_accounts = ToolboxIdlInstruction::try_parse_accounts(
            idl_instruction,
            breadcrumbs,
        )?;
        let instruction_args_type_flat_fields =
            ToolboxIdlInstruction::try_parse_args_type_flat_fields(
                idl_instruction,
                breadcrumbs,
            )?;
        let instruction_args_type_full_fields =
            instruction_args_type_flat_fields.try_hydrate(
                &HashMap::new(),
                typedefs,
                breadcrumbs,
            )?;
        Ok(ToolboxIdlInstruction {
            name: idl_instruction_name.to_string(),
            docs: instruction_docs,
            discriminator: instruction_discriminator,
            accounts: instruction_accounts,
            args_type_flat_fields: instruction_args_type_flat_fields,
            args_type_full_fields: instruction_args_type_full_fields.into(),
        })
    }

    fn try_parse_discriminator(
        idl_instruction_name: &str,
        idl_instruction: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<u8>, ToolboxIdlError> {
        if let Some(idl_instruction_discriminator) =
            idl_object_get_key_as_array(idl_instruction, "discriminator")
        {
            return idl_as_bytes_or_else(
                idl_instruction_discriminator,
                &breadcrumbs.as_val("discriminator"),
            );
        }
        Ok(idl_hash_discriminator_from_string(&format!(
            "global:{}",
            idl_convert_to_value_name(idl_instruction_name)
        )))
    }

    fn try_parse_accounts(
        idl_instruction: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<Vec<ToolboxIdlInstructionAccount>, ToolboxIdlError> {
        let idl_instruction_accounts_array =
            idl_object_get_key_as_array_or_else(
                idl_instruction,
                "accounts",
                &breadcrumbs.idl(),
            )?;
        let mut instruction_accounts = vec![];
        for (_, idl_instruction_account, breadcrumbs) in
            idl_iter_get_scoped_values(
                idl_instruction_accounts_array,
                breadcrumbs,
            )?
        {
            instruction_accounts.push(ToolboxIdlInstructionAccount::try_parse(
                idl_instruction_account,
                &breadcrumbs,
            )?);
        }
        Ok(instruction_accounts)
    }

    fn try_parse_args_type_flat_fields(
        idl_instruction: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlTypeFlatFields, ToolboxIdlError> {
        if let Some(idl_instruction_args) =
            idl_object_get_key_as_array(idl_instruction, "args")
        {
            return ToolboxIdlTypeFlatFields::try_parse(
                idl_instruction_args,
                breadcrumbs,
            );
        }
        Ok(ToolboxIdlTypeFlatFields::None)
    }
}
