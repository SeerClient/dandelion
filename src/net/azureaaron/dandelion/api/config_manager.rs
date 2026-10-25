//! Ported from net.azureaaron.dandelion.api.ConfigManager.
//!
//! The config object is a Java reference every holder shares (the screen builder's `config.field = v` setters write
//! the object the manager saves), so `instance()` / `unpatchedInstance()` / `defaults()` answer a `compat::Shared<T>`
//! - the shape of YACL's `ConfigClassHandler.instance()`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::patching::config_patch::ConfigPatch;
use crate::net::azureaaron::dandelion::r#impl::config_manager_impl::{AnyConfigManagerImpl, ConfigManagerImpl};

/// java: `ConfigManager<?>` - what the backends see of a manager (they only cast it to `ConfigManagerImpl<?>`).
pub trait AnyConfigManager: compat::DynAny {
    /// java: `(ConfigManagerImpl<?>) manager` - `None` is the `ClassCastException`.
    fn as_config_manager_impl(&self) -> Option<&dyn AnyConfigManagerImpl>;
}

/// java: public interface ConfigManager<T>
pub trait ConfigManager<T: 'static>: AnyConfigManager {
    /// java: Class<T> configClass();
    fn config_class(&self) -> compat::ClassOf<T>;

    /// java: T instance();
    ///
    /// Returns the patched instance of the `configClass` or a default instance if the config hasn't been
    /// successfully loaded. The underlying instance may change at any time, retaining strong references to it is
    /// highly discouraged. Note: this instance is NEVER saved as the user's config! Do not write updated values to it.
    fn instance(&self) -> compat::Shared<T>;

    /// Port-only (no Java counterpart): a counter that changes every time `instance()` starts answering a
    /// different object. A caller that keeps something derived from `instance()` can compare one number instead
    /// of fetching (and reference-counting) the instance on every call. `None`: the implementation keeps no
    /// counter, so the caller must compare `instance()` itself.
    fn instance_epoch(&self) -> Option<u64> {
        None
    }

    /// java: T unpatchedInstance();
    fn unpatched_instance(&self) -> compat::Shared<T>;

    /// java: T defaults();
    fn defaults(&self) -> compat::Shared<T>;

    /// java: boolean save();
    fn save(&self) -> bool;

    /// java: boolean load();
    fn load(&self) -> bool;

    /// java: void updatePatchedInstance();
    fn update_patched_instance(&self);

    /// java: void setPatches(List<ConfigPatch> patches);
    fn set_patches(&self, patches: Vec<Arc<dyn ConfigPatch>>);
}

/// java: static <T> ConfigManager<T> create(Class<T> configClass, Path path, UnaryOperator<GsonBuilder> gsonBuilder)
pub fn create<T: 'static>(
    config_class: compat::ClassOf<T>,
    path: std::path::PathBuf,
    gson_builder: impl FnOnce(compat::GsonBuilder) -> compat::GsonBuilder,
) -> Arc<dyn ConfigManager<T>> {
    ConfigManagerImpl::new(config_class, path, gson_builder)
}
