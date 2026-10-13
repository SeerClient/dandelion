//! Ported from net.azureaaron.dandelion.api.OptionAdder.
//!
//! Builders are values (`fn x(self, ..) -> Self`, PATTERNS_Y2 P2): Java's `return this` is returning `self`, so the
//! `ConfigCategory.Builder` / `OptionGroup.Builder` overrides that only narrow the return type (`OptionAdder.super
//! .optionIf(..); return this;`) are these defaults.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option::AnyOption;

/// java: public sealed interface OptionAdder permits ConfigCategory.Builder, OptionGroup.Builder
pub trait OptionAdder: Sized {
    /// java: OptionAdder option(Option<?> option);
    fn option(self, option: Arc<dyn AnyOption>) -> Self;

    /// java: default OptionAdder optionIf(boolean condition, Option<?> option)
    fn option_if(self, condition: bool, option: Arc<dyn AnyOption>) -> Self {
        if condition { self.option(option) } else { self }
    }

    /// java: default OptionAdder optionIf(boolean condition, Supplier<Option<?>> option)
    ///
    /// Allows for an option to be created only when a condition is met.
    fn option_if_boolean_supplier(self, condition: bool, option: impl FnOnce() -> Arc<dyn AnyOption>) -> Self {
        if condition { self.option(option()) } else { self }
    }

    /// java: default OptionAdder options(List<? extends Option<?>> options)
    fn options(mut self, options: Vec<Arc<dyn AnyOption>>) -> Self {
        for option in options {
            self = self.option(option);
        }
        self
    }
}
