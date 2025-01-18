use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

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

impl Display for ToolboxIdlContext {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result {
        f.write_fmt(format_args!("breadcrumbs: {:?}", self.breadcrumbs))?;
        f.write_str(&self.stage)?;
        Ok(())
    }
}
