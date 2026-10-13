//! Ported from net.azureaaron.dandelion.api.ConfigType.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::net::minecraft::client::resources::language::i18n;

/// java: public enum ConfigType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConfigType {
    Yacl,
    MoulConfig,
}

impl ConfigType {
    pub const VALUES: [ConfigType; 2] = [ConfigType::Yacl, ConfigType::MoulConfig];

    /// java: values()
    pub fn values() -> Vec<ConfigType> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            ConfigType::Yacl => "YACL",
            ConfigType::MoulConfig => "MOUL_CONFIG",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> ConfigType {
        match name {
            "YACL" => ConfigType::Yacl,
            "MOUL_CONFIG" => ConfigType::MoulConfig,
            _ => panic!("IllegalArgumentException: No enum constant ConfigType.{name}"),
        }
    }

    /// java: @Override public String toString()
    pub fn to_string(&self) -> String {
        return i18n::get(&format!("text.dandelion.configType.{}", self.name()), &[]);
    }

}

impl std::fmt::Display for ConfigType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ConfigType::to_string(self))
    }
}
