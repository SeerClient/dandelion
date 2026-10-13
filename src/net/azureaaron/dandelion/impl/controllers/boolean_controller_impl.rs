//! Ported from net.azureaaron.dandelion.impl.controllers.BooleanControllerImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::boolean_controller::{self, BooleanController, BooleanStyle};
use crate::net::azureaaron::dandelion::api::controllers::controller::{self, Controller, ControllerKind};

/// java: public class BooleanControllerImpl implements BooleanController
pub struct BooleanControllerImpl {
    /// java: private final BooleanStyle style;
    pub style: BooleanStyle,
    /// java: private final boolean coloured;
    pub coloured: bool,
}

impl BooleanControllerImpl {
    /// java: protected BooleanControllerImpl(BooleanStyle style, boolean coloured)
    pub fn new(style: BooleanStyle, coloured: bool) -> Self {
        // java: Objects.requireNonNull(style, ..) - an enum value is never null here
        Self { style, coloured }
    }
}

impl Controller<bool> for BooleanControllerImpl {
    fn kind(&self) -> ControllerKind<'_> {
        ControllerKind::Boolean(self)
    }
}

impl BooleanController for BooleanControllerImpl {
    /// java: @Override public BooleanStyle style()
    fn style(&self) -> BooleanStyle {
        self.style
    }

    /// java: @Override public boolean coloured()
    fn coloured(&self) -> bool {
        self.coloured
    }
}

/// java: public static class BooleanControllerBuilderImpl implements BooleanController.Builder
pub struct BooleanControllerBuilderImpl {
    /// java: private BooleanStyle style = BooleanStyle.ON_OFF;
    pub style: BooleanStyle,
    /// java: private boolean coloured = false;
    pub coloured: bool,
}

impl BooleanControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { style: BooleanStyle::OnOff, coloured: false }
    }
}

impl boolean_controller::Builder for BooleanControllerBuilderImpl {
    /// java: @Override public BooleanController.Builder booleanStyle(BooleanStyle style)
    fn boolean_style(mut self, style: BooleanStyle) -> Self {
        self.style = style;
        self
    }

    /// java: @Override public BooleanController.Builder coloured(boolean coloured)
    fn coloured(mut self, coloured: bool) -> Self {
        self.coloured = coloured;
        self
    }
}

impl controller::Builder<bool, dyn BooleanController> for BooleanControllerBuilderImpl {
    /// java: @Override public BooleanController build()
    fn build(self) -> Arc<dyn BooleanController> {
        Arc::new(BooleanControllerImpl::new(self.style, self.coloured))
    }
}
