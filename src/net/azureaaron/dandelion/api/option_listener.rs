//! Ported from net.azureaaron.dandelion.api.OptionListener by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: @FunctionalInterface public interface OptionListener<T>
pub trait OptionListener<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: void onUpdate(Option<T> option, UpdateType type);
    fn on_update(&self, option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>, mut r#type: UpdateType);

}

/// java: enum UpdateType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum UpdateType {
    ValueChange,
    YaclUnsupported,
}

impl UpdateType {
    pub const VALUES: [UpdateType; 2] = [UpdateType::ValueChange, UpdateType::YaclUnsupported];

    /// java: values()
    pub fn values() -> Vec<UpdateType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            UpdateType::ValueChange => "VALUE_CHANGE",
            UpdateType::YaclUnsupported => "YACL_UNSUPPORTED",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> UpdateType {
        match name {
            "VALUE_CHANGE" => UpdateType::ValueChange,
            "YACL_UNSUPPORTED" => UpdateType::YaclUnsupported,
            _ => panic!("IllegalArgumentException: No enum constant UpdateType.{name}"),
        }
    }

}

impl std::fmt::Display for UpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
