//! Ported from net.azureaaron.dandelion.impl.OptionBindingImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;

/// java: public class OptionBindingImpl<T> implements OptionBinding<T>
#[derive(Clone)]
pub struct OptionBindingImpl<T> {
    /// java: private final T defaultValue;
    pub default_value: T,
    /// java: private final Supplier<T> getter;
    pub getter: Arc<dyn Fn() -> T>,
    /// java: private final Consumer<T> setter;
    pub setter: Arc<dyn Fn(T)>,
}

impl<T: Clone + 'static> OptionBindingImpl<T> {
    /// java: protected OptionBindingImpl(T defaultValue, Supplier<T> getter, Consumer<T> setter)
    pub fn new(mut default_value: T, getter: Arc<dyn Fn() -> T>, setter: Arc<dyn Fn(T)>) -> Self {
        let __init_default_value = default_value.clone();
        let __init_getter = getter.clone();
        let __init_setter = setter.clone();
        let mut this = Self {
            default_value: __init_default_value,
            getter: __init_getter,
            setter: __init_setter,
        };
        this
    }

}

impl<T: Clone + 'static> OptionBinding<T> for OptionBindingImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public T defaultValue()
    fn default_value(&self) -> T {
        return self.default_value.clone();
    }

    /// java: @Override public T get()
    fn get(&self) -> T {
        return (self.getter)();
    }

    /// java: @Override public void set(T value)
    fn set(&mut self, value: &T) {
        (self.setter)(value.clone());
    }

}
