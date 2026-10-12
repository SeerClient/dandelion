//! Ported from net.azureaaron.dandelion.impl.GsonConfigSerializer by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_serializer::ConfigSerializer;
use crate::net::azureaaron::dandelion::api::config_serializer::ConfigSerializerBase;
use crate::net::azureaaron::dandelion::r#impl::config_manager_impl::ConfigManagerImpl;

/// java: public class GsonConfigSerializer<T> extends ConfigSerializer<T>
pub struct GsonConfigSerializer<T> {
    /// java: extends ConfigSerializer
    pub base: ConfigSerializerBase<T>,
    /// java: private final Gson gson;
    pub gson: compat::Gson,
}

impl<T: 'static> std::ops::Deref for GsonConfigSerializer<T> {
    type Target = ConfigSerializerBase<T>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for GsonConfigSerializer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> GsonConfigSerializer<T> {
    /// java: protected GsonConfigSerializer(ConfigManagerImpl<T> configManager, Path path, Gson gson)
    pub fn new(mut config_manager: compat::Shared<ConfigManagerImpl<T>>, mut path: std::path::PathBuf, mut gson: compat::Gson) -> Self {
        let __init_gson = gson.clone();
        let mut this = Self {
            base: ConfigSerializerBase::<T>::new(config_manager.clone(), path.clone()),
            gson: __init_gson,
        };
        this
    }

    /// java: protected static GsonBuilder createDefaultGsonBuilder()
    pub fn create_default_gson_builder() -> compat::GsonBuilder {
        return compat::GsonBuilder::new().set_field_naming_policy(compat::FieldNamingPolicy::Identity).serialize_nulls().register_type_hierarchy_adapter().register_type_hierarchy_adapter().register_type_hierarchy_adapter().register_type_hierarchy_adapter().register_type_hierarchy_adapter().set_pretty_printing();
    }

}

impl<T: Clone + 'static> ConfigSerializer<T> for GsonConfigSerializer<T> {
    fn config_serializer_base(&self) -> &ConfigSerializerBase<T> {
        &self.base
    }
    fn config_serializer_base_mut(&mut self) -> &mut ConfigSerializerBase<T> {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean save()
    fn save(&self) -> bool {
        let mut unpatched_instance: Option<T> = Some(self.config_manager.lock().borrow().unpatched_instance());
        let Some(mut unpatched_instance) = unpatched_instance.clone() else {
            logger().warn(&format!("[Dandelion] Cannot save a null instance of {} to {}! Skipping save.", compat::jstring(&self.config_manager.lock().borrow().config_class()), self.path.display()));
            return false;
        };
        let mut temporary_file: Option<std::path::PathBuf> = None;
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut json: String = self.gson.to_json(&unpatched_instance, &self.config_manager.lock().borrow().config_class());
            temporary_file = Some(compat::create_temp_file("dandelion_config", None).expect("IOException"));
            std::fs::write(temporary_file.as_ref().unwrap(), &json).expect("IOException");
            std::fs::create_dir_all(&self.path.parent().map(|x| x.to_path_buf()).unwrap()).expect("IOException");
            // java: try {..} catch (AtomicMoveNotSupportedException _) - an exception here is a panic, caught by `catch_unwind`
            let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                std::fs::rename(temporary_file.as_ref().unwrap(), &self.path).expect("AtomicMoveNotSupportedException");
            }));
            if let Err(_) = __result {
                compat::files_move(temporary_file.as_ref().unwrap(), &self.path).expect("IOException");
            }
            logger().info(&format!("[Dandelion] Successfully saved {} to {}.", compat::jstring(&self.config_manager.lock().borrow().config_class()), self.path.display()));
            return true;
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                // TODO(fabric2seer): a log call with a non-literal format
                //   java: LOGGER.error(LogUtils.FATAL_MARKER, "[Dandelion] Failed to save {} to {}!", this.configManager.configClass(), this.path, e);
                // java: try {..} catch (Exception _) - an exception here is a panic, caught by `catch_unwind`
                let __result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    if let Some(temporary_file) = &temporary_file {
                        std::fs::remove_file(temporary_file).is_ok();
                    }
                }));
                if let Err(_) = __result {
                }
            }
        }
        return false;
    }

    /// java: @Override public boolean load()
    fn load(&mut self) -> bool {
        if !self.path.exists() {
            logger().info(&format!("[Dandelion] No config found for {} at {}. Initializing and saving default config.", compat::jstring(&self.config_manager.lock().borrow().config_class()), self.path.display()));
            ConfigManagerImpl::set_unpatched_instance(&self.config_manager, ConfigManagerImpl::create_new_config_instance(&self.config_manager));
            self.save();
            return true;
        }
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut config: String = std::fs::read_to_string(&self.path).expect("IOException");
            let mut instance: T = self.gson.from_json_tree(&serde_json::from_str::<serde_json::Value>(&config).expect("JsonSyntaxException"), &self.config_manager.lock().borrow().config_class()).unwrap();
            ConfigManagerImpl::set_unpatched_instance(&self.config_manager, instance.clone());
            logger().info(&format!("[Dandelion] Successfully loaded {} from {}.", compat::jstring(&self.config_manager.lock().borrow().config_class()), self.path.display()));
            return true;
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                // TODO(fabric2seer): a log call with a non-literal format
                //   java: LOGGER.error(LogUtils.FATAL_MARKER, "[Dandelion] Failed to load the {} from {}!", this.configManager.configClass(), this.path, e);
            }
        }
        return false;
    }

    /// java: @Override public T copyObject(T source)
    fn copy_object(&self, source: &T) -> T {
        return self.gson.from_json(&self.gson.to_json(source, &self.config_manager.lock().borrow().config_class()), &self.config_manager.lock().borrow().config_class()).unwrap();
    }

}
