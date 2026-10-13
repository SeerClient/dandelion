//! Ported from net.azureaaron.dandelion.impl.controllers.StringControllerImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::{self, Controller, ControllerKind};
use crate::net::azureaaron::dandelion::api::controllers::string_controller::{self, StringController};

/// java: public class StringControllerImpl implements StringController
pub struct StringControllerImpl {}

impl StringControllerImpl {
    /// java: protected StringControllerImpl() {}
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller<String> for StringControllerImpl {
    fn kind(&self) -> ControllerKind<'_> {
        ControllerKind::String(self)
    }
}

impl StringController for StringControllerImpl {}

/// java: public static class StringControllerBuilderImpl implements StringController.Builder
pub struct StringControllerBuilderImpl {}

impl StringControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl string_controller::Builder for StringControllerBuilderImpl {}

impl controller::Builder<String, dyn StringController> for StringControllerBuilderImpl {
    /// java: @Override public StringController build()
    fn build(self) -> Arc<dyn StringController> {
        Arc::new(StringControllerImpl::new())
    }
}
