//! Ported from net.azureaaron.dandelion.api.controllers.FloatController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::number_controller::NumberController;
use crate::net::azureaaron::dandelion::r#impl::controllers::float_controller_impl::FloatControllerBuilderImpl;

/// java: public non-sealed interface FloatController extends NumberController<Float>
pub trait FloatController: NumberController + compat::DynAny {
}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (FloatControllerBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder extends NumberController.Builder<Float>
pub trait Builder: crate::net::azureaaron::dandelion::api::controllers::number_controller::Builder + compat::DynAny {
}
