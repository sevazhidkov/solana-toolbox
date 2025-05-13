use anyhow::Context;
use anyhow::Result;
use serde_json::Value;

use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_array;
use crate::toolbox_idl_utils::idl_value_as_object_get_key_as_str;
use crate::toolbox_idl_utils::idl_value_as_str_or_object_with_key_as_str_or_else;

impl ToolboxIdlTypedef {
    pub fn try_parse(
        idl_typedef_name: &str,
        idl_typedef: &Value,
    ) -> Result<ToolboxIdlTypedef> {
        let docs = idl_typedef.get("docs").cloned();
        let serialization =
            idl_value_as_object_get_key_as_str(idl_typedef, "serialization")
                .map(String::from);
        let repr = ToolboxIdlTypedef::try_parse_repr(idl_typedef)?;
        let generics = ToolboxIdlTypedef::try_parse_generics(idl_typedef)?;
        let type_flat = ToolboxIdlTypeFlat::try_parse_value(idl_typedef)
            .context("Parse Type")?;
        Ok(ToolboxIdlTypedef {
            name: idl_typedef_name.to_string(),
            docs,
            serialization,
            repr,
            generics,
            type_flat,
        })
    }

    fn try_parse_repr(idl_typedef: &Value) -> Result<Option<String>> {
        if let Some(idl_typedef_repr) = idl_typedef.get("repr") {
            return Ok(Some(
                idl_value_as_str_or_object_with_key_as_str_or_else(
                    idl_typedef_repr,
                    "kind",
                )?
                .to_string(),
            ));
        }
        Ok(None)
    }

    fn try_parse_generics(idl_typedef: &Value) -> Result<Vec<String>> {
        let mut generics = vec![];
        if let Some(idl_typedef_generics) =
            idl_value_as_object_get_key_as_array(idl_typedef, "generics")
        {
            for (index, idl_typedef_generic) in
                idl_typedef_generics.iter().enumerate()
            {
                let idl_typedef_generic_name =
                    idl_value_as_str_or_object_with_key_as_str_or_else(
                        idl_typedef_generic,
                        "name",
                    )
                    .with_context(|| {
                        format!("Parse Generic Name: {}", index)
                    })?;
                generics.push(idl_typedef_generic_name.to_string());
            }
        }
        Ok(generics)
    }
}
