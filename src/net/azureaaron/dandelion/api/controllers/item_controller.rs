//! Ported from net.azureaaron.dandelion.api.controllers.ItemController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::r#impl::controllers::item_controller_impl::ItemControllerBuilderImpl;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public non-sealed interface ItemController extends Controller<Item>
///
/// An `Item` value is `compat::JavaRef<Item>` (a reference compared by identity), as in YACL's `ItemController`.
pub trait ItemController: Controller<compat::JavaRef<Item>> {}

/// java: static Builder createBuilder()
pub fn create_builder() -> ItemControllerBuilderImpl {
    ItemControllerBuilderImpl::new()
}

/// java: interface Builder extends Controller.Builder<Item, ItemController> {}
pub trait Builder: controller::Builder<compat::JavaRef<Item>, dyn ItemController> {}
