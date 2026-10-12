//! Ported from net.azureaaron.dandelion.api.controllers.ItemController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::r#impl::controllers::item_controller_impl::ItemControllerBuilderImpl;

/// java: public non-sealed interface ItemController extends Controller<Item>
pub trait ItemController: Controller + compat::DynAny {
}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (ItemControllerBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder extends Controller.Builder<Item, ItemController>
pub trait Builder: crate::net::azureaaron::dandelion::api::controllers::controller::Builder + compat::DynAny {
}
