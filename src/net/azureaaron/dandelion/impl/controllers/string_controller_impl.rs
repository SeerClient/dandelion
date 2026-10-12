//! Ported from net.azureaaron.dandelion.impl.controllers.StringControllerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::controllers::string_controller::Builder;
use crate::net::azureaaron::dandelion::api::controllers::string_controller::StringController;

/// java: public class StringControllerImpl implements StringController
#[derive(Clone)]
pub struct StringControllerImpl {}

impl StringControllerImpl {
    /// java: protected StringControllerImpl()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

}

impl StringController for StringControllerImpl {
}

impl Controller<compat::Object> for StringControllerImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class StringControllerBuilderImpl implements StringController.Builder
#[derive(Clone)]
pub struct StringControllerBuilderImpl {}

impl StringControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

}

impl Builder for StringControllerBuilderImpl {
}

impl crate::net::azureaaron::dandelion::api::controllers::controller::Builder<compat::Object, compat::Object> for StringControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public StringController build()
    fn build(&self) -> compat::Shared<dyn StringController> {
        return (StringControllerImpl::new() as compat::Shared<dyn StringController>);
    }

}
