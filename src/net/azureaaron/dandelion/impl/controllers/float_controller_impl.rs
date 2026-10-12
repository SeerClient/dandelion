//! Ported from net.azureaaron.dandelion.impl.controllers.FloatControllerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::controllers::float_controller::Builder;
use crate::net::azureaaron::dandelion::api::controllers::float_controller::FloatController;
use crate::net::azureaaron::dandelion::api::controllers::number_controller::NumberController;

/// java: public class FloatControllerImpl implements FloatController
#[derive(Clone)]
pub struct FloatControllerImpl {
    /// java: private final float min;
    pub min: f32,
    /// java: private final float max;
    pub max: f32,
    /// java: private final float step;
    pub step: f32,
    /// java: private final boolean slider;
    pub slider: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl FloatControllerImpl {
    /// java: protected FloatControllerImpl(float min, float max, float step, boolean slider)
    pub fn new(mut min: f32, mut max: f32, mut step: f32, mut slider: bool) -> compat::Shared<Self> {
        let __init_min = min;
        let __init_max = max;
        let __init_step = step;
        let __init_slider = slider;
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            min: __init_min,
            max: __init_max,
            step: __init_step,
            slider: __init_slider,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl FloatController for FloatControllerImpl {
}

impl NumberController<compat::Object> for FloatControllerImpl {
    /// java: @Override public Float min()
    fn min(&self) -> f32 {
        return self.min;
    }

    /// java: @Override public Float max()
    fn max(&self) -> f32 {
        return self.max;
    }

    /// java: @Override public Float step()
    fn step(&self) -> f32 {
        return self.step;
    }

    /// java: @Override public boolean slider()
    fn slider(&self) -> bool {
        return self.slider;
    }

}

impl Controller<compat::Object> for FloatControllerImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class FloatControllerBuilderImpl implements FloatController.Builder
#[derive(Clone)]
pub struct FloatControllerBuilderImpl {
    /// java: private float min = Float.MIN_VALUE;
    pub min: f32,
    /// java: private float max = Float.MAX_VALUE;
    pub max: f32,
    /// java: private float step = 1f;
    pub step: f32,
    /// java: private boolean slider = false;
    pub slider: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl FloatControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            min: f32::MIN_POSITIVE,
            max: f32::MAX,
            step: 1.0f32,
            slider: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl Builder for FloatControllerBuilderImpl {
}

impl crate::net::azureaaron::dandelion::api::controllers::number_controller::Builder<compat::Object> for FloatControllerBuilderImpl {
    /// java: @Override public FloatController.Builder min(Float min)
    fn min(&mut self, min: &f32) -> compat::Shared<dyn Builder> {
        self.min = *min;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn Builder>);
    }

    /// java: @Override public FloatController.Builder max(Float max)
    fn max(&mut self, max: &f32) -> compat::Shared<dyn Builder> {
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn Builder>);
    }

    /// java: @Override public FloatController.Builder range(Float min, Float max)
    fn range(&mut self, min: &f32, max: &f32) -> compat::Shared<dyn Builder> {
        self.min = *min;
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn Builder>);
    }

    /// java: @Override public FloatController.Builder slider(Float step)
    fn slider(&mut self, step: &f32) -> compat::Shared<dyn Builder> {
        self.step = *step;
        self.slider = true;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn Builder>);
    }

}

impl crate::net::azureaaron::dandelion::api::controllers::controller::Builder<compat::Object, compat::Object> for FloatControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public FloatController build()
    fn build(&self) -> compat::Shared<dyn FloatController> {
        return (FloatControllerImpl::new(self.min, self.max, self.step, self.slider) as compat::Shared<dyn FloatController>);
    }

}
