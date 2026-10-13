//! Ported from net.azureaaron.dandelion.api.controllers.StringController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::r#impl::controllers::string_controller_impl::StringControllerBuilderImpl;

/// java: public non-sealed interface StringController extends Controller<String>
pub trait StringController: Controller<String> {}

/// java: static Builder createBuilder()
pub fn create_builder() -> StringControllerBuilderImpl {
    StringControllerBuilderImpl::new()
}

/// java: interface Builder extends Controller.Builder<String, StringController> {}
pub trait Builder: controller::Builder<String, dyn StringController> {}
