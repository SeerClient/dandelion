//! Ported from net.azureaaron.dandelion.impl.controllers.ItemControllerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::controllers::item_controller::Builder;
use crate::net::azureaaron::dandelion::api::controllers::item_controller::ItemController;

/// java: public class ItemControllerImpl implements ItemController
#[derive(Clone)]
pub struct ItemControllerImpl {}

impl ItemControllerImpl {
    /// java: protected ItemControllerImpl()
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

}

impl ItemController for ItemControllerImpl {
}

impl Controller<compat::Object> for ItemControllerImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class ItemControllerBuilderImpl implements ItemController.Builder
#[derive(Clone)]
pub struct ItemControllerBuilderImpl {}

impl ItemControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
        });
        __this
    }

}

impl Builder for ItemControllerBuilderImpl {
}

impl crate::net::azureaaron::dandelion::api::controllers::controller::Builder<compat::Object, compat::Object> for ItemControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ItemController build()
    fn build(&self) -> compat::Shared<dyn ItemController> {
        return (ItemControllerImpl::new() as compat::Shared<dyn ItemController>);
    }

}
