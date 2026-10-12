//! Ported from net.azureaaron.dandelion.api.controllers.ColourController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::r#impl::controllers::colour_controller_impl::ColourControllerBuilderImpl;

/// java: public non-sealed interface ColourController extends Controller<Color>
pub trait ColourController: Controller + compat::DynAny {
    /// java: boolean hasAlpha(); (dispatch: the body for this class)
    fn has_alpha(&self) -> fn(&compat::Shared<dyn ColourController>) -> bool;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (ColourControllerBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder extends Controller.Builder<Color, ColourController>
pub trait Builder: crate::net::azureaaron::dandelion::api::controllers::controller::Builder + compat::DynAny {
    /// java: Builder hasAlpha(boolean hasAlpha); (dispatch: the body for this class)
    fn has_alpha(&self) -> fn(&compat::Shared<dyn Builder>, bool) -> compat::Shared<dyn Builder>;

}
