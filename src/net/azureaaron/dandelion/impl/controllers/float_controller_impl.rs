//! Ported from net.azureaaron.dandelion.impl.controllers.FloatControllerImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::{self, Controller, ControllerKind, NumberControllerKind};
use crate::net::azureaaron::dandelion::api::controllers::float_controller::{self, FloatController};
use crate::net::azureaaron::dandelion::api::controllers::number_controller::{self, NumberController};

/// java: public class FloatControllerImpl implements FloatController
pub struct FloatControllerImpl {
    /// java: private final float min;
    pub min: f32,
    /// java: private final float max;
    pub max: f32,
    /// java: private final float step;
    pub step: f32,
    /// java: private final boolean slider;
    pub slider: bool,
}

impl FloatControllerImpl {
    /// java: protected FloatControllerImpl(float min, float max, float step, boolean slider)
    pub fn new(min: f32, max: f32, step: f32, slider: bool) -> Self {
        Self { min, max, step, slider }
    }
}

impl Controller<f32> for FloatControllerImpl {
    fn kind(&self) -> ControllerKind<'_> {
        ControllerKind::Number(NumberControllerKind::Float(self))
    }
}

impl NumberController<f32> for FloatControllerImpl {
    /// java: @Override public Float min()
    fn min(&self) -> f32 {
        self.min
    }

    /// java: @Override public Float max()
    fn max(&self) -> f32 {
        self.max
    }

    /// java: @Override public Float step()
    fn step(&self) -> f32 {
        self.step
    }

    /// java: @Override public boolean slider()
    fn slider(&self) -> bool {
        self.slider
    }
}

impl FloatController for FloatControllerImpl {}

/// java: public static class FloatControllerBuilderImpl implements FloatController.Builder
pub struct FloatControllerBuilderImpl {
    /// java: private float min = Float.MIN_VALUE;
    pub min: f32,
    /// java: private float max = Float.MAX_VALUE;
    pub max: f32,
    /// java: private float step = 1f;
    pub step: f32,
    /// java: private boolean slider = false;
    pub slider: bool,
}

impl FloatControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        // `Float.MIN_VALUE` is the smallest POSITIVE float (0x1p-149), not `f32::MIN`
        Self { min: f32::from_bits(1), max: f32::MAX, step: 1.0, slider: false }
    }
}

impl number_controller::Builder<f32> for FloatControllerBuilderImpl {
    /// java: @Override public FloatController.Builder min(Float min)
    fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// java: @Override public FloatController.Builder max(Float max)
    fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// java: @Override public FloatController.Builder range(Float min, Float max)
    fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// java: @Override public FloatController.Builder slider(Float step)
    fn slider(mut self, step: f32) -> Self {
        self.step = step;
        self.slider = true;
        self
    }
}

impl float_controller::Builder for FloatControllerBuilderImpl {}

impl controller::Builder<f32, dyn FloatController> for FloatControllerBuilderImpl {
    /// java: @Override public FloatController build()
    fn build(self) -> Arc<dyn FloatController> {
        Arc::new(FloatControllerImpl::new(self.min, self.max, self.step, self.slider))
    }
}
