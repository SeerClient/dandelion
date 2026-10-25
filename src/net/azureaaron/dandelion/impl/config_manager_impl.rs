//! Ported from net.azureaaron.dandelion.impl.ConfigManagerImpl.
//!
//! The manager's mutable fields (`patches`, `instance`, `unpatchedInstance`) are `RefCell`s borrowed for one
//! statement (PATTERNS_Y2 P1): `save` / `load` call back into the manager through the serializer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::{AnyConfigManager, ConfigManager};
use crate::net::azureaaron::dandelion::api::config_serializer::ConfigSerializer;
use crate::net::azureaaron::dandelion::api::patching::config_patch::ConfigPatch;
use crate::net::azureaaron::dandelion::r#impl::gson_config_serializer::GsonConfigSerializer;
use crate::net::azureaaron::dandelion::r#impl::patching::config_patcher::ConfigPatcher;
use crate::net::azureaaron::dandelion::r#impl::utils::reflection_utils::ReflectionUtils;
use mcrust::net::minecraft::resources::identifier::Identifier;
/// Java's plain fields, read from any thread (skin-download workers read the
/// config too): a lock instead of a `RefCell`, so a read racing a write waits
/// rather than panicking with "already mutably borrowed".
pub struct RefCell<T>(std::sync::RwLock<T>);
impl<T> RefCell<T> {
    pub fn new(v: T) -> Self { RefCell(std::sync::RwLock::new(v)) }
    pub fn borrow(&self) -> std::sync::RwLockReadGuard<'_, T> { self.0.read().unwrap_or_else(|p| p.into_inner()) }
    pub fn borrow_mut(&self) -> std::sync::RwLockWriteGuard<'_, T> { self.0.write().unwrap_or_else(|p| p.into_inner()) }
}
unsafe impl<T> Sync for RefCell<T> {}
unsafe impl<T> Send for RefCell<T> {}

/// java: `ConfigManagerImpl<?>` - the members the backends reach through `(ConfigManagerImpl<?>) manager`.
pub trait AnyConfigManagerImpl {
    /// java: public boolean isOptionPatched(@Nullable Identifier id)
    fn is_option_patched(&self, id: Option<&Identifier>) -> bool;

    /// java: public boolean saveAll()
    fn save_all(&self) -> bool;
}

/// java: public class ConfigManagerImpl<T> implements ConfigManager<T>
pub struct ConfigManagerImpl<T: 'static> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private final Path path;
    pub path: std::path::PathBuf,
    /// java: private final Gson gson;
    pub gson: compat::Gson,
    /// java: private final ConfigSerializer<T> serializer;
    pub serializer: Box<dyn ConfigSerializer<T>>,
    /// java: private List<ConfigPatch> patches = List.of();
    pub patches: RefCell<Vec<Arc<dyn ConfigPatch>>>,
    /// java: private T instance;
    pub instance: RefCell<compat::Shared<T>>,
    /// Port-only: bumped each time `instance` is replaced (see `ConfigManager::instance_epoch`).
    pub instance_epoch: std::sync::atomic::AtomicU64,
    /// java: private T unpatchedInstance;
    pub unpatched_instance: RefCell<compat::Shared<T>>,
}

impl<T: 'static> ConfigManagerImpl<T> {
    /// java: public ConfigManagerImpl(Class<T> configClass, Path path, UnaryOperator<GsonBuilder> gsonBuilder)
    ///
    /// The serializer keeps a reference back to the manager, so the manager is built with `Arc::new_cyclic`.
    pub fn new(config_class: compat::ClassOf<T>, path: std::path::PathBuf, gson_builder: impl FnOnce(compat::GsonBuilder) -> compat::GsonBuilder) -> Arc<ConfigManagerImpl<T>> {
        let gson = gson_builder(GsonConfigSerializer::<T>::create_default_gson_builder()).create();
        Arc::new_cyclic(|this| {
            let serializer: Box<dyn ConfigSerializer<T>> = Box::new(GsonConfigSerializer::new(this.clone(), path.clone(), gson.clone()));
            ConfigManagerImpl {
                instance: RefCell::new(compat::shared(ReflectionUtils::create_new_default_instance(&config_class))),
                instance_epoch: std::sync::atomic::AtomicU64::new(0),
                unpatched_instance: RefCell::new(compat::shared(ReflectionUtils::create_new_default_instance(&config_class))),
                config_class,
                path,
                gson,
                serializer,
                patches: RefCell::new(Vec::new()),
            }
        })
    }

    /// java: protected T createNewConfigInstance()
    pub fn create_new_config_instance(&self) -> T {
        ReflectionUtils::create_new_default_instance(&self.config_class)
    }

    /// java: protected void setUnpatchedInstance(T newInstance)
    pub fn set_unpatched_instance(&self, new_instance: T) {
        *self.unpatched_instance.borrow_mut() = compat::shared(new_instance);
        self.update_patched_instance();
    }

    /// java: public boolean isOptionPatched(@Nullable Identifier id)
    ///
    /// Whether the option with the corresponding `id` is patched.
    pub fn is_option_patched(&self, id: Option<&Identifier>) -> bool {
        // Short circuit for options without an id
        let Some(id) = id else {
            return false;
        };
        for patch in self.patches.borrow().iter() {
            if let Some(option_id) = patch.option_id() {
                if &option_id == id {
                    return true;
                }
            }
        }
        false
    }

    /// java: public boolean saveAll()
    ///
    /// Saves both the config and Minecraft's built-in options.
    pub fn save_all(&self) -> bool {
        // Ensure key updated mappings are saved
        // java: Minecraft.getInstance().options.save(); - rust: port gap (GAPS_D.md D3): the port's `Options.save`
        // writes the options map, which a `KeyMapping.setKey` from mod code does not update.
        if let Err(e) = mcrust::net::minecraft::client::options::save(&compat::minecraft().options.borrow()) {
            logger().error(&format!("Failed to save options: {e}"));
        }
        self.save()
    }
}

impl<T: 'static> AnyConfigManagerImpl for ConfigManagerImpl<T> {
    fn is_option_patched(&self, id: Option<&Identifier>) -> bool {
        ConfigManagerImpl::is_option_patched(self, id)
    }

    fn save_all(&self) -> bool {
        ConfigManagerImpl::save_all(self)
    }
}

impl<T: 'static> AnyConfigManager for ConfigManagerImpl<T> {
    fn as_config_manager_impl(&self) -> Option<&dyn AnyConfigManagerImpl> {
        Some(self)
    }
}

impl<T: 'static> ConfigManager<T> for ConfigManagerImpl<T> {
    /// java: @Override public Class<T> configClass()
    fn config_class(&self) -> compat::ClassOf<T> {
        self.config_class
    }

    /// java: @Override public T instance()
    fn instance(&self) -> compat::Shared<T> {
        self.instance.borrow().clone()
    }

    fn instance_epoch(&self) -> Option<u64> {
        Some(self.instance_epoch.load(std::sync::atomic::Ordering::Acquire))
    }

    /// java: @Override public T unpatchedInstance()
    fn unpatched_instance(&self) -> compat::Shared<T> {
        self.unpatched_instance.borrow().clone()
    }

    /// java: @Override public T defaults()
    fn defaults(&self) -> compat::Shared<T> {
        compat::shared(self.create_new_config_instance())
    }

    /// java: @Override public boolean save()
    fn save(&self) -> bool {
        let result = self.serializer.save();
        self.update_patched_instance();
        result
    }

    /// java: @Override public boolean load()
    fn load(&self) -> bool {
        // No need to update the patched instance since that will be done automatically by the serializer's call to
        // setUnpatchedInstance
        self.serializer.load()
    }

    /// java: @Override public void updatePatchedInstance()
    fn update_patched_instance(&self) {
        let unpatched = self.unpatched_instance.borrow().clone();
        let copy = compat::read(&unpatched, |source| self.serializer.copy_object(source));
        let instance = compat::shared(copy);
        *self.instance.borrow_mut() = instance.clone();
        // (published at the same point the object changes, so an epoch read before `instance()` never vouches
        // for an object newer than itself)
        self.instance_epoch.fetch_add(1, std::sync::atomic::Ordering::Release);
        let patches = self.patches.borrow().clone();
        ConfigPatcher::apply_patches(&instance, &self.config_class, &patches);
    }

    /// java: @Override public void setPatches(List<ConfigPatch> patches)
    ///
    /// Replaces the existing set of config patches (if any) and then applies them.
    fn set_patches(&self, patches: Vec<Arc<dyn ConfigPatch>>) {
        *self.patches.borrow_mut() = patches;
        self.update_patched_instance();
    }
}
