//! Ported from net.azureaaron.dandelion.impl.yacl.YACLConfigCategoryAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use ::yacl::dev::isxander::yacl3::api::config_category;
use ::yacl::dev::isxander::yacl3::api::config_category::Builder;
use ::yacl::dev::isxander::yacl3::api::option_addable::OptionAddable;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_adapter::YACLOptionAdapter;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_group_adapter::YACLOptionGroupAdapter;

/// java: public class YACLConfigCategoryAdapter
#[derive(Clone)]
pub struct YACLConfigCategoryAdapter {}

impl YACLConfigCategoryAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static List<dev.isxander.yacl3.api.ConfigCategory> toYaclConfigCategories(ConfigManager<?> manager, List<net.azureaaron.dandelion.api.ConfigCategory> categories)
    pub fn to_yacl_config_categories(manager: &compat::Shared<dyn ConfigManager<compat::Object>>, categories: &Vec<compat::Shared<dyn ConfigCategory>>) -> Vec<compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::config_category::ConfigCategory>> {
        let mut yacl_config_categories: Vec<compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::config_category::ConfigCategory>> = Vec::new();
        for category in categories.iter() {
            yacl_config_categories.push(YACLConfigCategoryAdapter::to_yacl_config_category(manager, category));
        }
        return yacl_config_categories;
    }

    /// java: private static dev.isxander.yacl3.api.ConfigCategory toYaclConfigCategory(ConfigManager<?> manager, net.azureaaron.dandelion.api.ConfigCategory category)
    pub fn to_yacl_config_category(manager: &compat::Shared<dyn ConfigManager<compat::Object>>, category: &compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::config_category::ConfigCategory> {
        let mut yacl_config_category = ({ let __r = &config_category::create_builder(); let __f = compat::dispatch(__r, |__t| __t.name()); __f(__r, ({ let __f = compat::dispatch(category, |__t| __t.name()); __f(category) })) });
        let mut group: Option<compat::Shared<dyn OptionGroup>> = ({ let __f = compat::dispatch(category, |__t| __t.root_group()); __f(category) });
        if group.is_some() && !(({ let __r = group.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.options()); __f(__r) }).is_empty()) {
            yacl_config_category = ({ let __f = compat::dispatch(&yacl_config_category, |__t| __t.options()); __f(&compat::view(&yacl_config_category, |a| a as compat::Shared<dyn OptionAddable>), &YACLOptionAdapter::to_yacl_options(manager, &({ let __r = group.as_ref().unwrap(); let __f = compat::dispatch(__r, |__t| __t.options()); __f(__r) }))) });
        }
        if !(({ let __f = compat::dispatch(category, |__t| __t.groups()); __f(category) }).is_empty()) {
            yacl_config_category = ({ let __f = compat::dispatch(&yacl_config_category, |__t| __t.groups()); __f(&yacl_config_category, &YACLOptionGroupAdapter::to_yacl_option_groups(manager, &({ let __f = compat::dispatch(category, |__t| __t.groups()); __f(category) }))) });
        }
        return ({ let __f = compat::dispatch(&yacl_config_category, |__t| __t.build()); __f(&yacl_config_category) });
    }

}
