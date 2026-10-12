//! Ported from net.azureaaron.dandelion.api.controllers.StringController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::r#impl::controllers::string_controller_impl::StringControllerBuilderImpl;

/// java: public non-sealed interface StringController extends Controller<String>
pub trait StringController: Controller + compat::DynAny {
}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (StringControllerBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder extends Controller.Builder<String, StringController>
pub trait Builder: crate::net::azureaaron::dandelion::api::controllers::controller::Builder + compat::DynAny {
}
