//! Ported from net.azureaaron.dandelion.impl.yacl.YACLOptionGroupAdapter.
//!
//! `toYaclOptionGroup`'s `switch (group)` dispatches through `OptionGroup::to_yacl_option_group`: a list option answers
//! `to_yacl_list_option_group` (the `case ListOption listOption` arm, with its `T`), every other group
//! `to_yacl_plain_option_group` (the `case OptionGroup optionGroup` arm).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::list_option::ListOption;
use crate::net::azureaaron::dandelion::api::option::{AnyOption, Option};
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_adapter::YACLOptionAdapter;
use ::yacl::dev::isxander::yacl3::api::list_option as ylist_option;
use ::yacl::dev::isxander::yacl3::api::option::Option as YOption;
use ::yacl::dev::isxander::yacl3::api::option_description;
use ::yacl::dev::isxander::yacl3::api::option_group as yoption_group;
use ::yacl::dev::isxander::yacl3::api::option_group::OptionGroup as YOptionGroup;

/// java: public class YACLOptionGroupAdapter
pub struct YACLOptionGroupAdapter {}

impl YACLOptionGroupAdapter {
    /// java: protected static List<dev.isxander.yacl3.api.OptionGroup> toYaclOptionGroups(ConfigManager<?> manager,
    /// List<net.azureaaron.dandelion.api.OptionGroup> groups)
    pub fn to_yacl_option_groups(manager: &Arc<dyn AnyConfigManager>, groups: Vec<Arc<dyn OptionGroup>>) -> Vec<Arc<dyn YOptionGroup>> {
        let mut yacl_groups: Vec<Arc<dyn YOptionGroup>> = Vec::new();
        for group in groups {
            yacl_groups.push(group.to_yacl_option_group(manager));
        }
        yacl_groups
    }

    /// java: the `case ListOption listOption` arm of `toYaclOptionGroup`
    pub fn to_yacl_list_option_group<T: Clone + PartialEq + 'static>(manager: &Arc<dyn AnyConfigManager>, list_option: Arc<dyn ListOption<T>>) -> Arc<dyn YOptionGroup> {
        let binding = list_option.binding();
        let get_binding = binding.clone();
        let set_binding = binding.clone();
        let controller_option = list_option.clone();
        let initial = list_option.initial_value();
        let patched = manager
            .as_config_manager_impl()
            .expect("ClassCastException: the manager is not a ConfigManagerImpl")
            .is_option_patched(AnyOption::id(&*list_option).as_ref());
        let as_option: Arc<dyn Option<Vec<T>>> = list_option.clone();
        let yacl: Arc<dyn ylist_option::ListOption<T>> = ylist_option::create_builder::<T>()
            .name(AnyOption::name(&*list_option))
            .description(option_description::create_builder().text(AnyOption::description(&*list_option)).build())
            .binding_3(binding.default_value(), move || get_binding.get(), move |value: Vec<T>| set_binding.set(value))
            .controller(move |yacl_option: Arc<dyn YOption<T>>| controller_option.entry_controller().controller_yacl(yacl_option, controller_option.entry_type()))
            .initial(move || initial())
            .collapsed(OptionGroup::collapsed(&*list_option))
            .available(list_option.modifiable() && !patched)
            .flags(YACLOptionAdapter::to_yacl_option_flags(&*list_option))
            .add_listeners(YACLOptionAdapter::to_yacl_option_event_listener(&as_option))
            .build();
        yacl
    }

    /// java: the `case OptionGroup optionGroup` arm of `toYaclOptionGroup`
    pub fn to_yacl_plain_option_group(manager: &Arc<dyn AnyConfigManager>, option_group: Arc<dyn OptionGroup>) -> Arc<dyn YOptionGroup> {
        let mut yacl_group = yoption_group::create_builder()
            .name(option_group.name())
            .description(option_description::create_builder().text(option_group.description()).build())
            .collapsed(option_group.collapsed());
        if !option_group.options().is_empty() {
            yacl_group = yacl_group.options(YACLOptionAdapter::to_yacl_options(manager, option_group.options()));
        }
        yacl_group.build()
    }
}
