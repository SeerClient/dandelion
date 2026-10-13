//! Ported from net.azureaaron.dandelion.api.controllers.NumberController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;

/// java: public sealed interface NumberController<T extends Number> extends Controller<T> permits IntegerController,
/// FloatController
pub trait NumberController<T: Clone + PartialEq + 'static>: Controller<T> {
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
///
/// Java's `build()` here answers `NumberController<T>` and each implementation narrows it (`IntegerController`,
/// `FloatController`); in Rust the one `build()` is the narrowed one, declared by `IntegerController.Builder` /
/// `FloatController.Builder` through `Controller.Builder<T, I>`.
pub trait Builder<T: Clone + PartialEq + 'static>: Sized {
    /// java: Builder<T> min(T min);
    fn min(self, min: T) -> Self;

    /// java: Builder<T> max(T max);
    fn max(self, max: T) -> Self;

    /// java: Builder<T> range(T min, T max);
    fn range(self, min: T, max: T) -> Self;

    /// java: Builder<T> slider(T step);
    fn slider(self, step: T) -> Self;
}
