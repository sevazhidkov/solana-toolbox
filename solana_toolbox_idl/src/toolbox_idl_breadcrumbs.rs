use crate::toolbox_idl_context::ToolboxIdlContext;

#[derive(Debug, Clone, Default)]
pub struct ToolboxIdlBreadcrumbs {
    idl: String,
    val: String,
}

impl ToolboxIdlBreadcrumbs {
    pub fn with_idl(
        &self,
        value: &str,
    ) -> ToolboxIdlBreadcrumbs {
        ToolboxIdlBreadcrumbs {
            idl: format!("{}:{}", self.idl, value),
            val: self.val.clone(),
        }
    }

    pub fn with_val(
        &self,
        value: &str,
    ) -> ToolboxIdlBreadcrumbs {
        ToolboxIdlBreadcrumbs {
            idl: self.idl.clone(),
            val: format!("{}:{}", self.val, value),
        }
    }

    pub fn as_idl(
        &self,
        value: &str,
    ) -> ToolboxIdlContext {
        ToolboxIdlContext::new(&format!("{}.{}.!", self.idl, value), &self.val)
    }

    pub fn as_val(
        &self,
        value: &str,
    ) -> ToolboxIdlContext {
        ToolboxIdlContext::new(&self.idl, &format!("{}.{}.!", self.val, value))
    }

    pub fn idl(&self) -> ToolboxIdlContext {
        ToolboxIdlContext::new(&format!("{}.!", self.idl), &self.val)
    }

    pub fn val(&self) -> ToolboxIdlContext {
        ToolboxIdlContext::new(&self.idl, &format!("{}.!", self.val))
    }
}
