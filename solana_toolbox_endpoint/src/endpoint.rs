use std::ops::Deref;
use std::ops::DerefMut;

use crate::endpoint_inner::EndpointInner;

pub struct Endpoint {
    inner: Box<dyn EndpointInner>,
}

impl From<Box<dyn EndpointInner>> for Endpoint {
    fn from(inner: Box<dyn EndpointInner>) -> Self {
        Self {
            inner,
        }
    }
}

impl Deref for Endpoint {
    type Target = Box<dyn EndpointInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Endpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
