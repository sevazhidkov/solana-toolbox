use crate::toolbox_idl_breadcrumbs::ToolboxIdlBreadcrumbs;

#[derive(Debug, Clone, Default)]
pub struct ToolboxIdlContext {
    breadcrumbs: ToolboxIdlBreadcrumbs,
    stage: String,
}

impl ToolboxIdlContext {
    pub fn new(
        breadcrumbs: &ToolboxIdlBreadcrumbs,
        stage: &str,
    ) -> ToolboxIdlContext {
        ToolboxIdlContext {
            breadcrumbs: breadcrumbs.clone(),
            stage: stage.to_string(),
        }
    }
}
