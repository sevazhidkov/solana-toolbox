use std::ops::Deref;
use std::ops::DerefMut;

use crate::toolbox_endpoint_inner::ToolboxEndpointInner;

pub struct ToolboxEndpoint {
    inner: Box<dyn ToolboxEndpointInner>,
}

impl From<Box<dyn ToolboxEndpointInner>> for ToolboxEndpoint {
    fn from(inner: Box<dyn ToolboxEndpointInner>) -> Self {
        Self { inner }
    }
}

impl Deref for ToolboxEndpoint {
    type Target = Box<dyn ToolboxEndpointInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ToolboxEndpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
