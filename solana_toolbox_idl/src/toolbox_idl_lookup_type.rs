use crate::toolbox_idl::ToolboxIdl;
use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;
use crate::toolbox_idl_error::ToolboxIdlError;
use crate::toolbox_idl_type::ToolboxIdlType;
use crate::toolbox_idl_utils::idl_object_get_key_or_else;

#[derive(Debug, Clone, PartialEq)]
pub struct ToolboxIdlLookupType {
    pub name: String,
    pub kind: ToolboxIdlType,
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
        let idl_type = idl_object_get_key_or_else(
            &self.types,
            type_name,
            &breadcrumbs.as_idl("types"),
        )?;
        Ok(ToolboxIdlLookupType {
            name: type_name.to_string(),
            kind: self.parse_type(idl_type, breadcrumbs)?,
        })
    }
}

impl ToolboxIdlLookupType {
    pub fn print(&self) {
        println!("----");
        println!("type.name: {}", self.name);
        println!("type.kind: {}", self.kind.describe());
    }
}
