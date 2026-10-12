//! Ported from net.azureaaron.dandelion.impl.controllers.IntegerControllerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::controllers::integer_controller::Builder;
use crate::net::azureaaron::dandelion::api::controllers::integer_controller::IntegerController;
use crate::net::azureaaron::dandelion::api::controllers::number_controller::NumberController;

/// java: public class IntegerControllerImpl implements IntegerController
#[derive(Clone)]
pub struct IntegerControllerImpl {
    /// java: private final int min;
    pub min: i32,
    /// java: private final int max;
    pub max: i32,
    /// java: private final int step;
    pub step: i32,
    /// java: private final boolean slider;
    pub slider: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl IntegerControllerImpl {
    /// java: protected IntegerControllerImpl(int min, int max, int step, boolean slider)
    pub fn new(mut min: i32, mut max: i32, mut step: i32, mut slider: bool) -> compat::Shared<Self> {
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

impl IntegerController for IntegerControllerImpl {
}

impl NumberController<compat::Object> for IntegerControllerImpl {
    /// java: @Override public Integer min()
    fn min(&self) -> i32 {
        return self.min;
    }

    /// java: @Override public Integer max()
    fn max(&self) -> i32 {
        return self.max;
    }

    /// java: @Override public Integer step()
    fn step(&self) -> i32 {
        return self.step;
    }

    /// java: @Override public boolean slider()
    fn slider(&self) -> bool {
        return self.slider;
    }

}

impl Controller<compat::Object> for IntegerControllerImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class IntegerControllerBuilderImpl implements IntegerController.Builder
#[derive(Clone)]
pub struct IntegerControllerBuilderImpl {
    /// java: private int min = Integer.MIN_VALUE;
    pub min: i32,
    /// java: private int max = Integer.MAX_VALUE;
    pub max: i32,
    /// java: private int step = 1;
    pub step: i32,
    /// java: private boolean slider = false;
    pub slider: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl IntegerControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            min: i32::MIN,
            max: i32::MAX,
            step: 1,
            slider: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl Builder for IntegerControllerBuilderImpl {
}

impl crate::net::azureaaron::dandelion::api::controllers::number_controller::Builder<compat::Object> for IntegerControllerBuilderImpl {
    /// java: @Override public IntegerController.Builder min(Integer min)
    fn min(&mut self, min: &i32) -> compat::Shared<dyn Builder> {
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn Builder>);
    }

    /// java: @Override public IntegerController.Builder max(Integer max)
    fn max(&mut self, max: &i32) -> compat::Shared<dyn Builder> {
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn Builder>);
    }

    /// java: @Override public IntegerController.Builder range(Integer min, Integer max)
    fn range(&mut self, min: &i32, max: &i32) -> compat::Shared<dyn Builder> {
        self.min = *min;
        self.max = *max;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn Builder>);
    }

    /// java: @Override public IntegerController.Builder slider(Integer step)
    fn slider(&mut self, step: &i32) -> compat::Shared<dyn Builder> {
        self.step = *step;
        self.slider = true;
        return (self.self_ref.upgrade().unwrap() as compat::Shared<dyn Builder>);
    }

}

impl crate::net::azureaaron::dandelion::api::controllers::controller::Builder<compat::Object, compat::Object> for IntegerControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public IntegerController build()
    fn build(&self) -> compat::Shared<dyn IntegerController> {
        return (IntegerControllerImpl::new(self.min, self.max, self.step, self.slider) as compat::Shared<dyn IntegerController>);
    }

}
