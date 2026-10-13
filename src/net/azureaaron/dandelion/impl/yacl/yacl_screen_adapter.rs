//! Ported from net.azureaaron.dandelion.impl.yacl.YACLScreenAdapter.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_config_category_adapter::YACLConfigCategoryAdapter;
use ::yacl::dev::isxander::yacl3::api::yet_another_config_lib;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class YACLScreenAdapter
pub struct YACLScreenAdapter {}

impl YACLScreenAdapter {
    /// java: public static Screen generateYaclScreen(ConfigManager<?> manager, Component title,
    /// List<ConfigCategory> categories, @Nullable Screen parent)
    pub fn generate_yacl_screen(
        manager: &Arc<dyn AnyConfigManager>,
        title: MutableComponent,
        categories: &[Arc<dyn ConfigCategory>],
        parent: Option<compat::ScreenRef>,
    ) -> compat::ScreenRef {
        let save_manager = manager.clone();
        let mut yacl_screen_builder = yet_another_config_lib::create_builder().title(title).save(move || {
            // java: ((ConfigManagerImpl<?>) manager)::saveAll
            save_manager.as_config_manager_impl().expect("ClassCastException: the manager is not a ConfigManagerImpl").save_all();
        });
        if !categories.is_empty() {
            yacl_screen_builder = yacl_screen_builder.categories(YACLConfigCategoryAdapter::to_yacl_config_categories(manager, categories));
        }
        yacl_screen_builder.build().generate_screen(parent)
    }
}
