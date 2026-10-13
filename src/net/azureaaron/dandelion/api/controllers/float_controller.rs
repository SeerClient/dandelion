//! Ported from net.azureaaron.dandelion.api.controllers.FloatController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller;
use crate::net::azureaaron::dandelion::api::controllers::number_controller;
use crate::net::azureaaron::dandelion::api::controllers::number_controller::NumberController;
use crate::net::azureaaron::dandelion::r#impl::controllers::float_controller_impl::FloatControllerBuilderImpl;

/// java: public non-sealed interface FloatController extends NumberController<Float>
pub trait FloatController: NumberController<f32> {}

/// java: static Builder createBuilder()
pub fn create_builder() -> FloatControllerBuilderImpl {
    FloatControllerBuilderImpl::new()
}

/// java: interface Builder extends NumberController.Builder<Float> {}
pub trait Builder: number_controller::Builder<f32> + controller::Builder<f32, dyn FloatController> {}
