//! Ported from net.azureaaron.dandelion.impl.yacl.YACLOptionGroupAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;

/// java: public class YACLOptionGroupAdapter
#[derive(Clone)]
pub struct YACLOptionGroupAdapter {}

impl YACLOptionGroupAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static List<dev.isxander.yacl3.api.OptionGroup> toYaclOptionGroups(ConfigManager<?> manager, List<net.azureaaron.dandelion.api.OptionGroup> groups)
    pub fn to_yacl_option_groups(manager: &compat::Shared<dyn ConfigManager<compat::Object>>, groups: &Vec<compat::Shared<dyn OptionGroup>>) -> Vec<compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option_group::OptionGroup>> {
        let mut yacl_groups: Vec<compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option_group::OptionGroup>> = Vec::new();
        for group in groups.iter() {
            yacl_groups.push(YACLOptionGroupAdapter::to_yacl_option_group(manager, group));
        }
        return yacl_groups;
    }

    /// java: @SuppressWarnings(
    pub fn to_yacl_option_group(manager: &compat::Shared<dyn ConfigManager<compat::Object>>, group: &compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option_group::OptionGroup> {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (group) {
        //   java: 			case ListOption listOption -> dev.isxander.yacl3.api.ListOption.createBuilder()
        //   java: 					.name(listOption.name())
        //   java: 					.description(OptionDescription.createBuilder()
        //   java: 							.text(listOption.description())
        //   java: 							.build())
        //   java: 					.binding((List<Object>) listOption.binding().defaultValue(),
        //   java: 							() -> (List<Object>) listOption.binding().get(),
        //   java: 							listOption.binding()::set)
        //   java: 					.controller(yaclOption -> listOption.entryController().controllerYACL(yaclOption, listOption.entryType()))
        //   java: 					.initial(listOption.initialValue())
        //   java: 					.collapsed(listOption.collapsed())
        //   java: 					.available(listOption.modifiable() && !((ConfigManagerImpl<?>) manager).isOptionPatched(listOption.id()))
        //   java: 					.flags(YACLOptionAdapter.toYaclOptionFlags(listOption))
        //   java: 					.addListeners(YACLOptionAdapter.toYaclOptionEventListener(listOption))
        //   java: 					.build();
        //   java:
        //   java: 			case OptionGroup optionGroup -> {
        //   java: 				var yaclGroup = dev.isxander.yacl3.api.OptionGroup.createBuilder()
        //   java: 						.name(optionGroup.name())
        //   java: 						.description(OptionDescription.createBuilder()
        //   java: 								.text(optionGroup.description())
        //   java: 								.build())
        //   java: 						.collapsed(optionGroup.collapsed());
        //   java:
        //   java: 				if (!optionGroup.options().isEmpty()) {
        //   java: 					yaclGroup = yaclGroup.options(YACLOptionAdapter.toYaclOptions(manager, optionGroup.options()));
        //   java: 				}
        //   java:
        //   java: 				yield yaclGroup.build();
        //   java: 			}
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
