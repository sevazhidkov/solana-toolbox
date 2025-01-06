pub struct ToolboxAnchorEndpoint {
    toolbox_endpoint: ToolboxEndpoint,
}

impl From<ToolboxEndpoint> for ToolboxAnchorEndpoint {
    fn from(toolbox_endpoint: ToolboxEndpoint) -> Self {
        Self { toolbox_endpoint }
    }
}

impl Deref for ToolboxAnchorEndpoint {
    type Target = ToolboxEndpoint;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ToolboxAnchorEndpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
