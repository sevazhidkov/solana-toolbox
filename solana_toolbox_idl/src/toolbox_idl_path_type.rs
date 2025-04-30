use std::collections::HashMap;
use std::sync::Arc;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use crate::toolbox_idl_path::ToolboxIdlPath;
use crate::toolbox_idl_path::ToolboxIdlPathPart;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlat;
use crate::toolbox_idl_type_flat::ToolboxIdlTypeFlatFields;
use crate::toolbox_idl_typedef::ToolboxIdlTypedef;
use crate::toolbox_idl_utils::idl_map_get_key_or_else;

// TODO - add test for path+type getters
impl ToolboxIdlPath {
    pub fn try_get_type_flat(
        &self,
        type_flat: &ToolboxIdlTypeFlat,
        generics_by_symbol: &HashMap<String, &ToolboxIdlTypeFlat>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFlat> {
        let Some((current, next)) = self.split_first() else {
            return Ok(type_flat.clone());
        };
        match type_flat {
            ToolboxIdlTypeFlat::Defined { name, generics } => {
                let typedef = idl_map_get_key_or_else(typedefs, name)
                    .context("Defined type")?;
                if generics.len() < typedef.generics.len() {
                    return Err(anyhow!(
                        "Insufficient number of generic parameter: expected: {}, found: {}",
                        typedef.generics.len(),
                        generics.len()
                    ));
                }
                let mut generics_by_symbol = HashMap::new();
                for (generic_name, generic_type) in
                    typedef.generics.iter().zip(generics)
                {
                    generics_by_symbol
                        .insert(generic_name.to_string(), generic_type);
                }
                self.try_get_type_flat(
                    &typedef.type_flat,
                    &generics_by_symbol,
                    typedefs,
                )
            },
            ToolboxIdlTypeFlat::Generic { symbol } => {
                let generic =
                    idl_map_get_key_or_else(generics_by_symbol, symbol)
                        .with_context(|| format!("Generic: {}", symbol))?;
                self.try_get_type_flat(generic, generics_by_symbol, typedefs)
            },
            ToolboxIdlTypeFlat::Option { content, .. } => {
                self.try_get_type_flat(content, generics_by_symbol, typedefs)
            },
            ToolboxIdlTypeFlat::Vec { items, .. } => {
                if let ToolboxIdlPathPart::Key(key) = current {
                    return Err(anyhow!("Invalid Vec Index: {}", key));
                }
                next.try_get_type_flat(items, generics_by_symbol, typedefs)
            },
            ToolboxIdlTypeFlat::Array { items, .. } => {
                if let ToolboxIdlPathPart::Key(key) = current {
                    return Err(anyhow!("Invalid Array Index: {}", key));
                }
                next.try_get_type_flat(items, generics_by_symbol, typedefs)
            },
            ToolboxIdlTypeFlat::Struct { fields } => self
                .try_get_type_flat_fields(fields, generics_by_symbol, typedefs),
            ToolboxIdlTypeFlat::Enum { variants, .. } => match current {
                ToolboxIdlPathPart::Empty => {
                    Err(anyhow!("Invalid Enum Variant: Empty String"))
                },
                ToolboxIdlPathPart::Key(key) => {
                    for variant in variants {
                        if variant.name == key {
                            return next.try_get_type_flat_fields(
                                &variant.fields,
                                generics_by_symbol,
                                typedefs,
                            );
                        }
                    }
                    Err(anyhow!("Could not find enum variant: {}", key))
                },
                ToolboxIdlPathPart::Code(code) => {
                    for variant in variants {
                        if variant.code == code {
                            return next.try_get_type_flat_fields(
                                &variant.fields,
                                generics_by_symbol,
                                typedefs,
                            );
                        }
                    }
                    Err(anyhow!("Could not find enum variant: {}", code))
                },
            },
            ToolboxIdlTypeFlat::Padded { content, .. } => {
                self.try_get_type_flat(content, generics_by_symbol, typedefs)
            },
            ToolboxIdlTypeFlat::Const { .. } => Err(anyhow!(
                "Type literal does not contain path: {}",
                self.value()
            )),
            ToolboxIdlTypeFlat::Primitive { .. } => Err(anyhow!(
                "Type primitive does not contain path: {}",
                self.value()
            )),
        }
    }

    pub fn try_get_type_flat_fields(
        &self,
        type_flat_fields: &ToolboxIdlTypeFlatFields,
        generics_by_symbol: &HashMap<String, &ToolboxIdlTypeFlat>,
        typedefs: &HashMap<String, Arc<ToolboxIdlTypedef>>,
    ) -> Result<ToolboxIdlTypeFlat> {
        let Some((current, next)) = self.split_first() else {
            return Ok(ToolboxIdlTypeFlat::Struct {
                fields: type_flat_fields.clone(),
            });
        };
        match type_flat_fields {
            ToolboxIdlTypeFlatFields::None => Err(anyhow!(
                "Empty fields does not contain path: {}",
                self.value()
            )),
            ToolboxIdlTypeFlatFields::Named(fields) => {
                let key = current.value();
                for field in fields {
                    if field.name == key {
                        return next.try_get_type_flat(
                            &field.content,
                            generics_by_symbol,
                            typedefs,
                        );
                    }
                }
                Err(anyhow!("Could not find named field: {}", key))
            },
            ToolboxIdlTypeFlatFields::Unnamed(fields) => {
                let length = fields.len();
                let index =
                    usize::try_from(current.code().context("Field index")?)?;
                if index >= length {
                    return Err(anyhow!(
                        "Invalid field index: {} (length: {})",
                        index,
                        length
                    ));
                }
                next.try_get_type_flat(
                    &fields[index].content,
                    generics_by_symbol,
                    typedefs,
                )
            },
        }
    }
}
