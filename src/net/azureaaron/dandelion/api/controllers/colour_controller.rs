//! Ported from net.azureaaron.dandelion.api.controllers.ColourController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::r#impl::controllers::colour_controller_impl::ColourControllerBuilderImpl;
use mcrust::java::awt::color::Color;

/// java: public non-sealed interface ColourController extends Controller<Color>
pub trait ColourController: Controller<Color> {
    /// java: boolean hasAlpha();
    fn has_alpha(&self) -> bool;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> ColourControllerBuilderImpl {
    ColourControllerBuilderImpl::new()
}

/// java: interface Builder extends Controller.Builder<Color, ColourController>
pub trait Builder: controller::Builder<Color, dyn ColourController> {
    /// java: Builder hasAlpha(boolean hasAlpha);
    fn has_alpha(self, has_alpha: bool) -> Self;
}
