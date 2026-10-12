//! Ported from net.azureaaron.dandelion.api.OptionBinding by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public interface OptionBinding<T>
pub trait OptionBinding<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: T defaultValue();
    fn default_value(&self) -> T;

    /// java: T get();
    fn get(&self) -> T;

    /// java: void set(T value);
    fn set(&mut self, value: &T);

}
