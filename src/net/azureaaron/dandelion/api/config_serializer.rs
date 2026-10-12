//! Ported from net.azureaaron.dandelion.api.ConfigSerializer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::config_manager_impl::ConfigManagerImpl;

/// java: public abstract class ConfigSerializer<T>
#[derive(Clone)]
pub struct ConfigSerializerBase<T> {
    /// java: protected final ConfigManagerImpl<T> configManager;
    pub config_manager: compat::Shared<ConfigManagerImpl<T>>,
    /// java: protected final Path path;
    pub path: std::path::PathBuf,
}

impl<T: Clone + 'static> ConfigSerializerBase<T> {
    /// java: protected ConfigSerializer(ConfigManagerImpl<T> configManager, Path path)
    pub fn new(mut config_manager: compat::Shared<ConfigManagerImpl<T>>, mut path: std::path::PathBuf) -> Self {
        let __init_config_manager = config_manager.clone();
        let __init_path = path.clone();
        let mut this = Self {
            config_manager: __init_config_manager,
            path: __init_path,
        };
        this
    }

}

/// java: the virtual methods of ConfigSerializer (implemented by ConfigSerializerBase and every subclass)
pub trait ConfigSerializer<T: Clone + 'static> {
    fn config_serializer_base(&self) -> &ConfigSerializerBase<T>;
    fn config_serializer_base_mut(&mut self) -> &mut ConfigSerializerBase<T>;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: public abstract boolean save();
    fn save(&self) -> bool;

    /// java: public abstract boolean load();
    fn load(&mut self) -> bool;

    /// java: public abstract T copyObject(T source);
    fn copy_object(&self, source: &T) -> T;

}
