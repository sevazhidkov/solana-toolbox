use std::collections::HashMap;

use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_program_type::ToolboxIdlProgramType;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_full::ToolboxIdlTypeFull;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlProgramInstructionArg {
    pub name: String,
    pub type_flat: ToolboxIdlTypeFlat,
    pub type_full: ToolboxIdlTypeFull,
}

impl ToolboxIdlProgramInstructionArg {
    pub(crate) fn try_parse(
        program_types: &HashMap<String, ToolboxIdlProgramType>,
        idl_instruction_arg_name: &str,
        idl_instruction_arg: &Map<String, Value>,
        breadcrumbs: &ToolboxIdlBreadcrumbs,
    ) -> Result<ToolboxIdlProgramInstructionArg, ToolboxIdlError> {
        let idl_instruction_arg_type = idl_object_get_key_or_else(
            idl_instruction_arg,
            "type",
            &breadcrumbs.idl(),
        )?;
        // TODO - this could be abbreviated without the "type" wrapper, similar to unamed fields
        let type_flat = ToolboxIdlTypeFlat::try_parse(
            idl_instruction_arg_type,
            &breadcrumbs,
        )?;
        let type_full = ToolboxIdlTypeFull::try_hydrate(
            program_types,
            &HashMap::new(),
            &type_flat,
            &breadcrumbs,
        )?;
        Ok(ToolboxIdlProgramInstructionArg {
            name: idl_instruction_arg_name.to_string(),
            type_flat,
            type_full,
        })
    }
}
