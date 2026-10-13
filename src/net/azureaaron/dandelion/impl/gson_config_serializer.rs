//! Ported from net.azureaaron.dandelion.impl.GsonConfigSerializer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::config_serializer::{ConfigSerializer, ConfigSerializerBase};
use crate::net::azureaaron::dandelion::r#impl::config_manager_impl::ConfigManagerImpl;
use std::sync::Weak;

/// java: public class GsonConfigSerializer<T> extends ConfigSerializer<T>
pub struct GsonConfigSerializer<T: 'static> {
    /// java: extends ConfigSerializer
    pub base: ConfigSerializerBase<T>,
    /// java: private final Gson gson;
    pub gson: compat::Gson,
}

impl<T: 'static> GsonConfigSerializer<T> {
    /// java: protected GsonConfigSerializer(ConfigManagerImpl<T> configManager, Path path, Gson gson)
    pub fn new(config_manager: Weak<ConfigManagerImpl<T>>, path: std::path::PathBuf, gson: compat::Gson) -> Self {
        Self { base: ConfigSerializerBase::new(config_manager, path), gson }
    }

    /// java: protected static GsonBuilder createDefaultGsonBuilder()
    ///
    /// The five type hierarchy adapters (Color, Component, Style, Identifier, Item) are carried by the generated
    /// per-class Gson forms (`compat::GsonBuilder::register_type_hierarchy_adapter`, GAPS_Y2 G2).
    pub fn create_default_gson_builder() -> compat::GsonBuilder {
        compat::GsonBuilder::new()
            .set_field_naming_policy(compat::FieldNamingPolicy::Identity)
            .serialize_nulls()
            // .registerTypeHierarchyAdapter(Color.class, new CodecTypeAdapter<>(CodecUtils.COLOUR_CODEC))
            .register_type_hierarchy_adapter()
            // .registerTypeHierarchyAdapter(Component.class, new CodecTypeAdapter<>(ComponentSerialization.CODEC))
            .register_type_hierarchy_adapter()
            // .registerTypeHierarchyAdapter(Style.class, new StyleTypeAdapter())
            .register_type_hierarchy_adapter()
            // .registerTypeHierarchyAdapter(Identifier.class, new CodecTypeAdapter<>(Identifier.CODEC))
            .register_type_hierarchy_adapter()
            // .registerTypeHierarchyAdapter(Item.class, new ItemTypeAdapter())
            .register_type_hierarchy_adapter()
            .set_pretty_printing()
    }

    fn path(&self) -> &std::path::PathBuf {
        &self.base.path
    }
}

impl<T: 'static> ConfigSerializer<T> for GsonConfigSerializer<T> {
    fn config_serializer_base(&self) -> &ConfigSerializerBase<T> {
        &self.base
    }

    /// java: @Override public boolean save()
    ///
    /// Saves the config file. Uses the atomic move pattern (when available) to reduce the chance of corrupting the
    /// config file.
    fn save(&self) -> bool {
        let config_manager = self.base.config_manager();
        let config_class = config_manager.config_class();
        // java: `if (unpatchedInstance == null)` - the shared instance is never null here
        let unpatched_instance = config_manager.unpatched_instance();
        let mut temporary_file: Option<std::path::PathBuf> = None;
        // java: try {..} catch (Exception e)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> std::io::Result<()> {
            let json: String = compat::read(&unpatched_instance, |instance| self.gson.to_json(instance, &config_class));
            // Write to temporary file
            let file = compat::create_temp_file("dandelion_config", None)?;
            temporary_file = Some(file.clone());
            std::fs::write(&file, &json)?;
            // Create the directories that the config file needs if necessary
            if let Some(parent) = self.path().parent() {
                std::fs::create_dir_all(parent)?;
            }
            // Atomically move the file to the proper config location
            if std::fs::rename(&file, self.path()).is_err() {
                // java: catch (AtomicMoveNotSupportedException _) - fall back to non-atomic move if it is not supported
                // (moving across partitions/drives)
                compat::files_move(&file, self.path())?;
            }
            Ok(())
        }));
        match result {
            Ok(Ok(())) => {
                logger().info(&format!("[Dandelion] Successfully saved {} to {}.", compat::jstring(&config_class), self.path().display()));
                return true;
            }
            Ok(Err(e)) => {
                logger().error(&format!("[Dandelion] Failed to save {} to {}! {e}", compat::jstring(&config_class), self.path().display()));
            }
            Err(e) => {
                logger().error(&format!("[Dandelion] Failed to save {} to {}! {}", compat::jstring(&config_class), self.path().display(), compat::panic_message(&e)));
            }
        }
        // Delete temporary file so it doesn't persist (not needed normally since it gets moved)
        if let Some(temporary_file) = &temporary_file {
            let _ = std::fs::remove_file(temporary_file);
        }
        false
    }

    /// java: @Override public boolean load()
    fn load(&self) -> bool {
        let config_manager = self.base.config_manager();
        let config_class = config_manager.config_class();
        if !self.path().exists() {
            logger().info(&format!(
                "[Dandelion] No config found for {} at {}. Initializing and saving default config.",
                compat::jstring(&config_class),
                self.path().display()
            ));
            config_manager.set_unpatched_instance(config_manager.create_new_config_instance());
            self.save();
            return true;
        }
        // java: try {..} catch (Exception e)
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<T, String> {
            let config: String = std::fs::read_to_string(self.path()).map_err(|e| e.to_string())?;
            let json: serde_json::Value = serde_json::from_str(&config).map_err(|e| format!("JsonSyntaxException: {e}"))?;
            // java: `gson.fromJson(..)` answers null for a JSON null, and the null instance fails later; here it is
            // the failure
            self.gson.from_json_tree(&json, &config_class).ok_or_else(|| "the config is null".to_string())
        }));
        match result {
            Ok(Ok(instance)) => {
                config_manager.set_unpatched_instance(instance);
                logger().info(&format!("[Dandelion] Successfully loaded {} from {}.", compat::jstring(&config_class), self.path().display()));
                return true;
            }
            Ok(Err(e)) => {
                logger().error(&format!("[Dandelion] Failed to load the {} from {}! {e}", compat::jstring(&config_class), self.path().display()));
            }
            Err(e) => {
                logger().error(&format!("[Dandelion] Failed to load the {} from {}! {}", compat::jstring(&config_class), self.path().display(), compat::panic_message(&e)));
            }
        }
        false
    }

    /// java: @Override public T copyObject(T source)
    fn copy_object(&self, source: &T) -> T {
        // Copies the object by converting it to JSON and back
        let config_class = self.base.config_manager().config_class();
        self.gson
            .from_json(&self.gson.to_json(source, &config_class), &config_class)
            .expect("NullPointerException: the copied config is null")
    }
}
