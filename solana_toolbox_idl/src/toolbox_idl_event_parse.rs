use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Context;
use anyhow::Result;
use serde_json::Map;
use serde_json::Value;

use crate::toolbox_idl_event::ToolboxIdlEvent;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_value_as_bytes_or_else;
use crate::toolbox_idl_utils::idl_value_as_object_or_else;
use crate::toolbox_idl_utils::idl_hash_discriminator_from_string;
use crate::toolbox_idl_utils::idl_object_get_key_as_array;

impl ToolboxIdlEvent {
    pub fn try_parse(
        idl_event_name: &str,
        idl_event: &Value,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlEvent> {
        let idl_event = idl_value_as_object_or_else(idl_event)?;
        let discriminator =
            ToolboxIdlEvent::try_parse_discriminator(idl_event_name, idl_event)
                .context("Parse Discriminator")?;
        let docs = idl_event.get("docs").cloned();
        let info_type_flat = ToolboxIdlEvent::try_parse_info_type_flat(
            idl_event_name,
            idl_event,
        )
        .context("Parse Info Type")?;
        let info_type_full = info_type_flat
            .try_hydrate(&HashMap::new(), typedefs)
            .context("Hydrate Info Type")?;
        Ok(ToolboxIdlEvent {
            name: idl_event_name.to_string(),
            docs,
            discriminator,
            info_type_flat,
            info_type_full,
        })
    }

    fn try_parse_discriminator(
        idl_event_name: &str,
        idl_event: &Map<String, Value>,
    ) -> Result<Vec<u8>> {
        if let Some(idl_event_discriminator) =
            idl_object_get_key_as_array(idl_event, "discriminator")
        {
            return idl_value_as_bytes_or_else(idl_event_discriminator);
        }
        Ok(idl_hash_discriminator_from_string(&format!(
            "event:{}",
            idl_event_name
        )))
    }

    fn try_parse_info_type_flat(
        idl_event_name: &str,
        idl_event: &Map<String, Value>,
    ) -> Result<ToolboxIdlTypeFlat> {
        if ToolboxIdlTypeFlat::try_parse_object_is_possible(idl_event) {
            return ToolboxIdlTypeFlat::try_parse_object(idl_event);
        }
        Ok(ToolboxIdlTypeFlat::Defined {
            name: idl_event_name.to_string(),
            generics: vec![],
        })
    }
}
