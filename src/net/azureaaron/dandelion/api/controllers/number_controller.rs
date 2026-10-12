//! Ported from net.azureaaron.dandelion.api.controllers.NumberController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;

/// java: public sealed interface NumberController<T extends Number> extends Controller<T> permits IntegerController, FloatController
pub trait NumberController<T: Clone + 'static>: Controller {
    /// java: T min();
    fn min(&self) -> T;

    /// java: T max();
    fn max(&self) -> T;

    /// java: T step();
    fn step(&self) -> T;

    /// java: boolean slider();
    fn slider(&self) -> bool;

}

/// java: interface Builder<T extends Number> extends Controller.Builder<T, NumberController<T>>
pub trait Builder<T: Clone + 'static>: crate::net::azureaaron::dandelion::api::controllers::controller::Builder {
    /// java: Builder<T> min(T min);
    fn min(&mut self, min: &T) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> max(T max);
    fn max(&mut self, max: &T) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> range(T min, T max);
    fn range(&mut self, min: &T, max: &T) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> slider(T step);
    fn slider(&mut self, step: &T) -> compat::Shared<dyn Builder<T>>;

}
