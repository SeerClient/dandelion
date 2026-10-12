//! Ported from net.azureaaron.dandelion.api.controllers.IntegerController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::number_controller::NumberController;
use crate::net::azureaaron::dandelion::r#impl::controllers::integer_controller_impl::IntegerControllerBuilderImpl;

/// java: public non-sealed interface IntegerController extends NumberController<Integer>
pub trait IntegerController: NumberController + compat::DynAny {
}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (IntegerControllerBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder extends NumberController.Builder<Integer>
pub trait Builder: crate::net::azureaaron::dandelion::api::controllers::number_controller::Builder + compat::DynAny {
}
