//! Ported from net.azureaaron.dandelion.impl.OptionBindingImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;

/// java: public class OptionBindingImpl<T> implements OptionBinding<T>
pub struct OptionBindingImpl<T: Clone + 'static> {
    /// java: private final T defaultValue;
    pub default_value: T,
    /// java: private final Supplier<T> getter;
    pub getter: Arc<dyn Fn() -> T>,
    /// java: private final Consumer<T> setter;
    pub setter: Arc<dyn Fn(T)>,
}

impl<T: Clone + 'static> OptionBindingImpl<T> {
    /// java: protected OptionBindingImpl(T defaultValue, Supplier<T> getter, Consumer<T> setter)
    pub fn new(default_value: T, getter: Arc<dyn Fn() -> T>, setter: Arc<dyn Fn(T)>) -> Self {
        // java: Objects.requireNonNull(getter / setter, ..) - never null here
        Self { default_value, getter, setter }
    }
}

impl<T: Clone + 'static> OptionBinding<T> for OptionBindingImpl<T> {
    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        self.default_value.clone()
    }

    /// java: @Override public T get()
    fn get(&self) -> T {
        (self.getter)()
    }

    /// java: @Override public void set(T value)
    fn set(&self, value: T) {
        (self.setter)(value)
    }
}
