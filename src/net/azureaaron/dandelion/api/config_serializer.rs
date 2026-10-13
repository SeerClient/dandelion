//! Ported from net.azureaaron.dandelion.api.ConfigSerializer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::config_manager_impl::ConfigManagerImpl;
use std::sync::Weak;

/// java: public abstract class ConfigSerializer<T> - its fields.
///
/// The manager owns its serializer (`ConfigManagerImpl.serializer`), so the serializer's reference back is a `Weak`
/// (PATTERNS_Y2 P5).
pub struct ConfigSerializerBase<T: 'static> {
    /// java: protected final ConfigManagerImpl<T> configManager;
    pub config_manager: Weak<ConfigManagerImpl<T>>,
    /// java: protected final Path path;
    pub path: std::path::PathBuf,
}

impl<T: 'static> ConfigSerializerBase<T> {
    /// java: protected ConfigSerializer(ConfigManagerImpl<T> configManager, Path path)
    pub fn new(config_manager: Weak<ConfigManagerImpl<T>>, path: std::path::PathBuf) -> Self {
        // java: Objects.requireNonNull(..) - neither can be null here
        Self { config_manager, path }
    }

    /// `this.configManager`
    pub fn config_manager(&self) -> Arc<ConfigManagerImpl<T>> {
        self.config_manager.upgrade().expect("a ConfigSerializer outlived its ConfigManager")
    }
}

/// java: the abstract methods of ConfigSerializer
pub trait ConfigSerializer<T: 'static> {
    fn config_serializer_base(&self) -> &ConfigSerializerBase<T>;

    /// java: public abstract boolean save();
    fn save(&self) -> bool;

    /// java: public abstract boolean load();
    fn load(&self) -> bool;

    /// java: public abstract T copyObject(T source);
    ///
    /// Performs a deep copy of the `source` object.
    fn copy_object(&self, source: &T) -> T;
}
