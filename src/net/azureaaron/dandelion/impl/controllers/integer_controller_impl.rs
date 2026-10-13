//! Ported from net.azureaaron.dandelion.impl.controllers.IntegerControllerImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::{self, Controller, ControllerKind, NumberControllerKind};
use crate::net::azureaaron::dandelion::api::controllers::integer_controller::{self, IntegerController};
use crate::net::azureaaron::dandelion::api::controllers::number_controller::{self, NumberController};

/// java: public class IntegerControllerImpl implements IntegerController
pub struct IntegerControllerImpl {
    /// java: private final int min;
    pub min: i32,
    /// java: private final int max;
    pub max: i32,
    /// java: private final int step;
    pub step: i32,
    /// java: private final boolean slider;
    pub slider: bool,
}

impl IntegerControllerImpl {
    /// java: protected IntegerControllerImpl(int min, int max, int step, boolean slider)
    pub fn new(min: i32, max: i32, step: i32, slider: bool) -> Self {
        Self { min, max, step, slider }
    }
}

impl Controller<i32> for IntegerControllerImpl {
    fn kind(&self) -> ControllerKind<'_> {
        ControllerKind::Number(NumberControllerKind::Integer(self))
    }
}

impl NumberController<i32> for IntegerControllerImpl {
    /// java: @Override public Integer min()
    fn min(&self) -> i32 {
        self.min
    }

    /// java: @Override public Integer max()
    fn max(&self) -> i32 {
        self.max
    }

    /// java: @Override public Integer step()
    fn step(&self) -> i32 {
        self.step
    }

    /// java: @Override public boolean slider()
    fn slider(&self) -> bool {
        self.slider
    }
}

impl IntegerController for IntegerControllerImpl {}

/// java: public static class IntegerControllerBuilderImpl implements IntegerController.Builder
pub struct IntegerControllerBuilderImpl {
    /// java: private int min = Integer.MIN_VALUE;
    pub min: i32,
    /// java: private int max = Integer.MAX_VALUE;
    pub max: i32,
    /// java: private int step = 1;
    pub step: i32,
    /// java: private boolean slider = false;
    pub slider: bool,
}

impl IntegerControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { min: i32::MIN, max: i32::MAX, step: 1, slider: false }
    }
}

impl number_controller::Builder<i32> for IntegerControllerBuilderImpl {
    /// java: @Override public IntegerController.Builder min(Integer min)
    ///
    /// The Java body is `return this;` - it does not store `min` (kept as written).
    fn min(self, min: i32) -> Self {
        let _ = min;
        self
    }

    /// java: @Override public IntegerController.Builder max(Integer max)
    fn max(mut self, max: i32) -> Self {
        self.max = max;
        self
    }

    /// java: @Override public IntegerController.Builder range(Integer min, Integer max)
    fn range(mut self, min: i32, max: i32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// java: @Override public IntegerController.Builder slider(Integer step)
    fn slider(mut self, step: i32) -> Self {
        self.step = step;
        self.slider = true;
        self
    }
}

impl integer_controller::Builder for IntegerControllerBuilderImpl {}

impl controller::Builder<i32, dyn IntegerController> for IntegerControllerBuilderImpl {
    /// java: @Override public IntegerController build()
    fn build(self) -> Arc<dyn IntegerController> {
        Arc::new(IntegerControllerImpl::new(self.min, self.max, self.step, self.slider))
    }
}
