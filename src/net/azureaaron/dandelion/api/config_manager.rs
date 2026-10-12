//! Ported from net.azureaaron.dandelion.api.ConfigManager by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::config_patch::ConfigPatch;
use crate::net::azureaaron::dandelion::r#impl::config_manager_impl::ConfigManagerImpl;

/// java: public interface ConfigManager<T>
pub trait ConfigManager<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Class<T> configClass();
    fn config_class(&self) -> compat::ClassOf<T>;

    /// java: T instance();
    fn instance(&self) -> T;

    /// java: T unpatchedInstance();
    fn unpatched_instance(&self) -> T;

    /// java: T defaults();
    fn defaults(&self) -> T;

    /// java: boolean save();
    fn save(&mut self) -> bool;

    /// java: boolean load();
    fn load(&mut self) -> bool;

    /// java: void updatePatchedInstance();
    fn update_patched_instance(&mut self);

    /// java: void setPatches(List<ConfigPatch> patches);
    fn set_patches(&mut self, patches: &Vec<compat::Shared<dyn ConfigPatch>>);

}

/// java: static <T> ConfigManager<T> create(Class<T> configClass, Path path, UnaryOperator<GsonBuilder> gsonBuilder)
pub fn create<T: 'static>(config_class: &compat::ClassOf<T>, path: &std::path::PathBuf, gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> compat::Shared<dyn ConfigManager<T>> {
    return (ConfigManagerImpl::new(config_class.clone(), path.clone(), gson_builder.clone()) as compat::Shared<dyn ConfigManager<compat::Object>>);
}
