//! Ported from net.azureaaron.dandelion.impl.yacl.YACLConfigCategoryAdapter.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_adapter::YACLOptionAdapter;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_group_adapter::YACLOptionGroupAdapter;
use ::yacl::dev::isxander::yacl3::api::config_category as yconfig_category;
use ::yacl::dev::isxander::yacl3::api::config_category::ConfigCategory as YConfigCategory;

/// java: public class YACLConfigCategoryAdapter
pub struct YACLConfigCategoryAdapter {}

impl YACLConfigCategoryAdapter {
    /// java: protected static List<dev.isxander.yacl3.api.ConfigCategory> toYaclConfigCategories(ConfigManager<?> manager,
    /// List<net.azureaaron.dandelion.api.ConfigCategory> categories)
    pub fn to_yacl_config_categories(manager: &Arc<dyn AnyConfigManager>, categories: &[Arc<dyn ConfigCategory>]) -> Vec<Arc<dyn YConfigCategory>> {
        let mut yacl_config_categories: Vec<Arc<dyn YConfigCategory>> = Vec::new();
        for category in categories {
            yacl_config_categories.push(YACLConfigCategoryAdapter::to_yacl_config_category(manager, category));
        }
        yacl_config_categories
    }

    /// java: private static dev.isxander.yacl3.api.ConfigCategory toYaclConfigCategory(ConfigManager<?> manager,
    /// net.azureaaron.dandelion.api.ConfigCategory category)
    fn to_yacl_config_category(manager: &Arc<dyn AnyConfigManager>, category: &Arc<dyn ConfigCategory>) -> Arc<dyn YConfigCategory> {
        let mut yacl_config_category = yconfig_category::create_builder().name(category.name());
        if let Some(group) = category.root_group() {
            if !group.options().is_empty() {
                yacl_config_category = yacl_config_category.options(YACLOptionAdapter::to_yacl_options(manager, group.options()));
            }
        }
        if !category.groups().is_empty() {
            yacl_config_category = yacl_config_category.groups(YACLOptionGroupAdapter::to_yacl_option_groups(manager, category.groups()));
        }
        yacl_config_category.build()
    }
}
