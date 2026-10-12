//! Ported from net.azureaaron.dandelion.impl.ConfigManagerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::config_serializer::ConfigSerializer;
use crate::net::azureaaron::dandelion::api::patching::config_patch::ConfigPatch;
use crate::net::azureaaron::dandelion::r#impl::gson_config_serializer::GsonConfigSerializer;
use crate::net::azureaaron::dandelion::r#impl::patching::config_patcher::ConfigPatcher;
use crate::net::azureaaron::dandelion::r#impl::utils::reflection_utils::ReflectionUtils;
use mcrust::net::minecraft::client::options;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class ConfigManagerImpl<T> implements ConfigManager<T>
pub struct ConfigManagerImpl<T> {
    /// java: private final Class<T> configClass;
    pub config_class: compat::ClassOf<T>,
    /// java: private final Path path;
    pub path: std::path::PathBuf,
    /// java: private final Gson gson;
    pub gson: compat::Gson,
    /// java: private final ConfigSerializer<T> serializer;
    pub serializer: Option<compat::Shared<dyn ConfigSerializer<T>>>,
    /// java: private List<ConfigPatch> patches = List.of();
    pub patches: Vec<compat::Shared<dyn ConfigPatch>>,
    /// java: private T instance;
    pub instance: Option<T>,
    /// java: private T unpatchedInstance;
    pub unpatched_instance: Option<T>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: Clone + 'static> ConfigManagerImpl<T> {
    /// java: public ConfigManagerImpl(Class<T> configClass, Path path, UnaryOperator<GsonBuilder> gsonBuilder)
    pub fn new(mut config_class: compat::ClassOf<T>, mut path: std::path::PathBuf, gson_builder: Arc<dyn Fn(compat::GsonBuilder) -> compat::GsonBuilder>) -> compat::Shared<Self> {
        let __init_config_class = config_class.clone();
        let __init_path = path.clone();
        let __init_gson = (gson_builder)(GsonConfigSerializer::create_default_gson_builder()).create();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            config_class: __init_config_class,
            path: __init_path,
            gson: __init_gson,
            serializer: None,
            patches: vec![],
            instance: None,
            unpatched_instance: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some((compat::shared(GsonConfigSerializer::new(__this.clone(), compat::read(&__this, |__t| __t.path.clone()), compat::read(&__this, |__t| __t.gson.clone()))) as compat::Shared<dyn ConfigSerializer<compat::Object>>)); compat::write(&__this, |__t| __t.serializer = __v); }
        { let __v = Some(ConfigManagerImpl::create_new_config_instance(&__this)); compat::write(&__this, |__t| __t.instance = __v); }
        { let __v = Some(ConfigManagerImpl::create_new_config_instance(&__this)); compat::write(&__this, |__t| __t.unpatched_instance = __v); }
        __this
    }

    /// java: protected T createNewConfigInstance()
    pub fn create_new_config_instance(__this: &compat::Shared<Self>) -> T {
        return ReflectionUtils::create_new_default_instance(&compat::read(__this, |__t| __t.config_class.clone()));
    }

    /// java: protected void setUnpatchedInstance(T newInstance)
    pub fn set_unpatched_instance(__this: &compat::Shared<Self>, mut new_instance: T) {
        { let __v = Some(new_instance.clone()); compat::write(__this, |__t| __t.unpatched_instance = __v); }
        __this.lock().borrow_mut().update_patched_instance();
    }

    /// java: public boolean isOptionPatched(@Nullable Identifier id)
    pub fn is_option_patched(__this: &compat::Shared<Self>, mut id: Option<Identifier>) -> bool {
        let Some(mut id) = id.clone() else {
            return false;
        };
        for patch in compat::read(__this, |__t| __t.patches.clone()) {
            if ({ let __f = compat::dispatch(&patch, |__t| __t.option_id()); __f(&patch) }).is_some() && ({ let __f = compat::dispatch(&patch, |__t| __t.option_id()); __f(&patch) }).unwrap() == id {
                return true;
            }
        }
        return false;
    }

    /// java: public boolean saveAll()
    pub fn save_all(__this: &compat::Shared<Self>) -> bool {
        // TODO(fabric2seer): `save` takes 1 argument(s) in the port (`data: &OptionsData`), Java passes 0
        // rust: options::save();
        return __this.lock().borrow_mut().save();
    }

}

impl<T: Clone + 'static> ConfigManager<T> for ConfigManagerImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public void updatePatchedInstance()
    fn update_patched_instance(&mut self) {
        self.instance = Some({ let (__a0,) = (self.unpatched_instance.as_ref().unwrap(),); self.serializer.as_ref().unwrap().lock().borrow().copy_object(__a0) });
        ConfigPatcher::apply_patches(self.instance.as_ref().unwrap(), &self.patches);
    }

    /// java: @Override public void setPatches(List<ConfigPatch> patches)
    fn set_patches(&mut self, patches: &Vec<compat::Shared<dyn ConfigPatch>>) {
        self.patches = patches.clone();
        self.update_patched_instance();
    }

    /// java: @Override public Class<T> configClass()
    fn config_class(&self) -> compat::ClassOf<T> {
        return self.config_class.clone();
    }

    /// java: @Override public T instance()
    fn instance(&self) -> T {
        return self.instance.clone().unwrap();
    }

    /// java: @Override public T unpatchedInstance()
    fn unpatched_instance(&self) -> T {
        return self.unpatched_instance.clone().unwrap();
    }

    /// java: @Override public T defaults()
    fn defaults(&self) -> T {
        return ConfigManagerImpl::create_new_config_instance(&self.self_ref.upgrade().unwrap());
    }

    /// java: @Override public boolean save()
    fn save(&mut self) -> bool {
        let mut result: bool = self.serializer.as_ref().unwrap().lock().borrow().save();
        self.update_patched_instance();
        return result;
    }

    /// java: @Override public boolean load()
    fn load(&mut self) -> bool {
        return self.serializer.as_mut().unwrap().lock().borrow_mut().load();
    }

}
