use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_utils::idl_describe_type_of_object;
use crate::toolbox_idl_utils::idl_object_get_key_as_object_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_scoped_named_object_array_or_else;
use crate::toolbox_idl_utils::idl_object_get_key_as_str;

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupType {
    pub name: String,
    pub kind: String,
    pub items: Vec<ToolboxIdlLookupTypeItem>,
}

#[derive(Debug, Clone)]
pub struct ToolboxIdlLookupTypeItem {
    pub name: String,
    pub description: String,
}

impl ToolboxIdl {
    pub fn lookup_types(
        &self
    ) -> Result<Vec<ToolboxIdlLookupType>, ToolboxIdlError> {
        let mut types = vec![];
        for idl_type_name in self.types.keys() {
            types.push(self.lookup_type(idl_type_name)?);
        }
        Ok(types)
    }

    pub fn lookup_type(
        &self,
        type_name: &str,
    ) -> Result<ToolboxIdlLookupType, ToolboxIdlError> {
        let breadcrumbs = &ToolboxIdlBreadcrumbs::default();
        let idl_type_object = idl_object_get_key_as_object_or_else(
            &self.types,
            type_name,
            &breadcrumbs.as_idl("types"),
        )?;
        if let Some(idl_type_kind) =
            idl_object_get_key_as_str(idl_type_object, "kind")
        {
            if idl_type_kind == "struct" {
                let mut type_fields = vec![];
                for (idl_field_object, idl_field_name, breadcrumbs) in
                    idl_object_get_key_as_scoped_named_object_array_or_else(
                        idl_type_object,
                        "fields",
                        &breadcrumbs.with_idl(type_name),
                    )?
                {
                    type_fields.push(ToolboxIdlLookupTypeItem {
                        name: idl_field_name.to_string(),
                        description: idl_describe_type_of_object(
                            idl_field_object,
                            &breadcrumbs,
                        )?,
                    });
                }
                return Ok(ToolboxIdlLookupType {
                    name: type_name.to_string(),
                    kind: "struct".to_string(),
                    items: type_fields,
                });
            }
            if idl_type_kind == "enum" {
                let mut type_variants = vec![];
                for (index, (_, idl_variant_name, _)) in
                    idl_object_get_key_as_scoped_named_object_array_or_else(
                        idl_type_object,
                        "variants",
                        &breadcrumbs.with_idl(type_name),
                    )?
                    .into_iter()
                    .enumerate()
                {
                    type_variants.push(ToolboxIdlLookupTypeItem {
                        name: index.to_string(),
                        description: idl_variant_name.to_string(),
                    });
                }
                return Ok(ToolboxIdlLookupType {
                    name: type_name.to_string(),
                    kind: "enum".to_string(),
                    items: type_variants,
                });
            }
        }
        Ok(ToolboxIdlLookupType {
            name: type_name.to_string(),
            kind: "unparsable".to_string(),
            items: vec![],
        })
    }
}

impl ToolboxIdlLookupType {
    pub fn print(&self) {
        println!("----");
        println!("{}.name: {:?}", self.kind, self.name);
        for item in &self.items {
            println!("{}.item: {}: {}", self.kind, item.name, item.description);
        }
    }
}
