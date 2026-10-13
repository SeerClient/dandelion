//! Ported from net.azureaaron.dandelion.impl.controllers.ColourControllerImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::colour_controller::{self, ColourController};
use crate::net::azureaaron::dandelion::api::controllers::controller::{self, Controller, ControllerKind};
use mcrust::java::awt::color::Color;

/// java: public class ColourControllerImpl implements ColourController
pub struct ColourControllerImpl {
    /// java: private final boolean hasAlpha;
    pub has_alpha: bool,
}

impl ColourControllerImpl {
    /// java: protected ColourControllerImpl(boolean hasAlpha)
    pub fn new(has_alpha: bool) -> Self {
        Self { has_alpha }
    }
}

impl Controller<Color> for ColourControllerImpl {
    fn kind(&self) -> ControllerKind<'_> {
        ControllerKind::Colour(self)
    }
}

impl ColourController for ColourControllerImpl {
    /// java: @Override public boolean hasAlpha()
    fn has_alpha(&self) -> bool {
        self.has_alpha
    }
}

/// java: public static class ColourControllerBuilderImpl implements ColourController.Builder
pub struct ColourControllerBuilderImpl {
    /// java: private boolean hasAlpha = false;
    pub has_alpha: bool,
}

impl ColourControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { has_alpha: false }
    }
}

impl colour_controller::Builder for ColourControllerBuilderImpl {
    /// java: @Override public ColourController.Builder hasAlpha(boolean hasAlpha)
    fn has_alpha(mut self, has_alpha: bool) -> Self {
        self.has_alpha = has_alpha;
        self
    }
}

impl controller::Builder<Color, dyn ColourController> for ColourControllerBuilderImpl {
    /// java: @Override public ColourController build()
    fn build(self) -> Arc<dyn ColourController> {
        Arc::new(ColourControllerImpl::new(self.has_alpha))
    }
}
