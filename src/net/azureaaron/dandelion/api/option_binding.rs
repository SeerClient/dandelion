//! Ported from net.azureaaron.dandelion.api.OptionBinding.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface OptionBinding<T>
pub trait OptionBinding<T: Clone + 'static> {
    /// java: T defaultValue();
    fn default_value(&self) -> T;

    /// java: T get();
    fn get(&self) -> T;

    /// java: void set(T value);
    fn set(&self, value: T);
}
