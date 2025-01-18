use crate::toolbox_idl_context::ToolboxIdlContext;

#[derive(Debug, Clone, Default)]
pub struct ToolboxIdlBreadcrumbs {
    name: String,
    kind: String,
}

impl ToolboxIdlBreadcrumbs {
    pub fn name(
        &self,
        value: &str,
    ) -> ToolboxIdlBreadcrumbs {
        ToolboxIdlBreadcrumbs {
            name: format!("{}.{}", self.name, value),
            kind: self.kind.clone(),
        }
    }
    pub fn kind(
        &self,
        value: &str,
    ) -> ToolboxIdlBreadcrumbs {
        ToolboxIdlBreadcrumbs {
            name: self.name.clone(),
            kind: format!("{}.{}", self.kind, value),
        }
    }
    pub fn context(
        &self,
        stage: &str,
    ) -> ToolboxIdlContext {
        ToolboxIdlContext::new(self, stage)
    }
}
