//! Ported from net.azureaaron.dandelion.api.controllers.IntegerController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller;
use crate::net::azureaaron::dandelion::api::controllers::number_controller;
use crate::net::azureaaron::dandelion::api::controllers::number_controller::NumberController;
use crate::net::azureaaron::dandelion::r#impl::controllers::integer_controller_impl::IntegerControllerBuilderImpl;

/// java: public non-sealed interface IntegerController extends NumberController<Integer>
pub trait IntegerController: NumberController<i32> {}

/// java: static Builder createBuilder()
pub fn create_builder() -> IntegerControllerBuilderImpl {
    IntegerControllerBuilderImpl::new()
}

/// java: interface Builder extends NumberController.Builder<Integer> {}
pub trait Builder: number_controller::Builder<i32> + controller::Builder<i32, dyn IntegerController> {}
