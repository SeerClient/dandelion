//! Ported from net.azureaaron.dandelion.impl.yacl.YACLOptionAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use ::yacl::dev::isxander::yacl3::api::option_event_listener::Event;
use ::yacl::dev::isxander::yacl3::api::option_event_listener::OptionEventListener;
use ::yacl::dev::isxander::yacl3::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::option_listener::UpdateType;
use crate::net::azureaaron::dandelion::api::option::Option as _;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener as _;

/// java: public class YACLOptionAdapter
#[derive(Clone)]
pub struct YACLOptionAdapter {}

impl YACLOptionAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: protected static List<dev.isxander.yacl3.api.Option<?>> toYaclOptions(ConfigManager<?> manager, List<? extends net.azureaaron.dandelion.api.Option<?>> options)
    pub fn to_yacl_options(manager: &compat::Shared<dyn ConfigManager<compat::Object>>, options: &Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> Vec<compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option::Option<compat::Object>>> {
        let mut yacl_options: Vec<compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option::Option<compat::Object>>> = Vec::new();
        for option in options.iter() {
            yacl_options.push(YACLOptionAdapter::to_yacl_option(manager, option));
        }
        return yacl_options;
    }

    /// java: @SuppressWarnings("unchecked") private static <T> dev.isxander.yacl3.api.Option<T> toYaclOption(ConfigManager<?> manager, net.azureaaron.dandelion.api.Option<T> option)
    pub fn to_yacl_option<T: 'static>(manager: &compat::Shared<dyn ConfigManager<compat::Object>>, option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>) -> compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option::Option<T>> {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (option) {
        //   java: 			case net.azureaaron.dandelion.api.ButtonOption button -> {
        //   java: 				yield (dev.isxander.yacl3.api.Option<T>) dev.isxander.yacl3.api.ButtonOption.createBuilder()
        //   java: 				.name(button.name())
        //   java: 				.description(OptionDescription.createBuilder()
        //   java: 						.text(button.description())
        //   java: 						.build())
        //   java: 				.text(button.prompt())
        //   java: 				.action((screen, _) -> button.action().accept(screen))
        //   java: 				.build();
        //   java: 			}
        //   java:
        //   java: 			case net.azureaaron.dandelion.api.KeyMappingOption keyMapping -> {
        //   java: 				yield (dev.isxander.yacl3.api.Option<T>) dev.isxander.yacl3.api.Option.<InputConstants.Key>createBuilder()
        //   java: 				.name(keyMapping.name())
        //   java: 				.description(OptionDescription.createBuilder()
        //   java: 						.text(keyMapping.description())
        //   java: 						.build())
        //   java: 				.binding(keyMapping.keyMapping().getDefaultKey(),
        //   java: 						() -> ((KeyMappingAccessor) keyMapping.keyMapping()).getKey(),
        //   java: 						newValue -> {
        //   java: 							keyMapping.keyMapping().setKey(newValue);
        //   java: 							KeyMapping.resetMapping();
        //   java: 						})
        //   java: 				.customController(KeyMappingController::new)
        //   java: 				.build();
        //   java: 			}
        //   java:
        //   java: 			case net.azureaaron.dandelion.api.LabelOption label -> {
        //   java: 				yield (dev.isxander.yacl3.api.Option<T>) dev.isxander.yacl3.api.LabelOption.createBuilder()
        //   java: 				.line(label.label())
        //   java: 				.build();
        //   java: 			}
        //   java:
        //   java: 			case net.azureaaron.dandelion.api.Option<T> _ -> {
        //   java: 				yield dev.isxander.yacl3.api.Option.<T>createBuilder()
        //   java: 				.name(option.name())
        //   java: 				.description(OptionDescription.createBuilder()
        //   java: 						.text(option.description())
        //   java: 						.build())
        //   java: 				.binding(option.binding().defaultValue(),
        //   java: 						option.binding()::get,
        //   java: 						option.binding()::set)
        //   java: 				.addListeners(toYaclOptionEventListener(option))
        //   java: 				.controller(yaclOption -> option.controller().controllerYACL(yaclOption, option.type()))
        //   java: 				.available(option.modifiable() && !((ConfigManagerImpl<?>) manager).isOptionPatched(option.id()))
        //   java: 				.flags(toYaclOptionFlags(option))
        //   java: 				.build();
        //   java: 			}
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: protected static <T> List<OptionEventListener<T>> toYaclOptionEventListener(net.azureaaron.dandelion.api.Option<T> option)
    pub fn to_yacl_option_event_listener<T: 'static>(option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>) -> Vec<compat::Shared<dyn OptionEventListener<T>>> {
        let mut yacl_option_event_listeners: Vec<compat::Shared<dyn OptionEventListener<T>>> = Vec::new();
        for listener in option.lock().borrow().listeners() {
            let mut yacl_listener: compat::Shared<dyn OptionEventListener<T>> = Arc::new(move |_, event| {
                { let (__a0, __a1,) = (option, YACLOptionAdapter::convert_update_type(event.clone()),); listener.lock().borrow().on_update(__a0, __a1) };
            });
            yacl_option_event_listeners.push(yacl_listener.clone());
        }
        return yacl_option_event_listeners;
    }

    /// java: private static OptionListener.UpdateType convertUpdateType(OptionEventListener.Event event)
    pub fn convert_update_type(mut event: Event) -> UpdateType {
        return match event {
            Event::StateChange => UpdateType::ValueChange,
            _ => UpdateType::YaclUnsupported,
        };
    }

    /// java: protected static List<dev.isxander.yacl3.api.OptionFlag> toYaclOptionFlags(net.azureaaron.dandelion.api.Option<?> option)
    pub fn to_yacl_option_flags(option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> Vec<compat::Shared<dyn OptionFlag>> {
        let mut yacl_option_flags: Vec<compat::Shared<dyn OptionFlag>> = Vec::new();
        for flag in option.lock().borrow().flags() {
            // TODO(fabric2seer): `accept` is not a method of OptionFlag
            //   java: dev.isxander.yacl3.api.OptionFlag yaclFlag = flag::accept;
            let mut yacl_flag: compat::Shared<dyn OptionFlag> = todo!("fabric2seer: the initialiser of `yaclFlag` (TODO above)");
            yacl_option_flags.push(yacl_flag.clone());
        }
        return yacl_option_flags;
    }

}
