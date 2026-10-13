//! Ported from net.azureaaron.dandelion.impl.controllers.ItemControllerImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::{self, Controller, ControllerKind};
use crate::net::azureaaron::dandelion::api::controllers::item_controller::{self, ItemController};
use mcrust::net::minecraft::world::item::item::Item;

/// java: public class ItemControllerImpl implements ItemController
pub struct ItemControllerImpl {}

impl ItemControllerImpl {
    /// java: protected ItemControllerImpl() {}
    pub fn new() -> Self {
        Self {}
    }
}

impl Controller<compat::JavaRef<Item>> for ItemControllerImpl {
    fn kind(&self) -> ControllerKind<'_> {
        ControllerKind::Item(self)
    }
}

impl ItemController for ItemControllerImpl {}

/// java: public static class ItemControllerBuilderImpl implements ItemController.Builder
pub struct ItemControllerBuilderImpl {}

impl ItemControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {}
    }
}

impl item_controller::Builder for ItemControllerBuilderImpl {}

impl controller::Builder<compat::JavaRef<Item>, dyn ItemController> for ItemControllerBuilderImpl {
    /// java: @Override public ItemController build()
    fn build(self) -> Arc<dyn ItemController> {
        Arc::new(ItemControllerImpl::new())
    }
}
