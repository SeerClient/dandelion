//! Ported from net.azureaaron.dandelion.impl.yacl.YACLOptionAdapter.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::option::{AnyOption, Option};
use crate::net::azureaaron::dandelion::api::option_listener::UpdateType;
use crate::net::azureaaron::dandelion::r#impl::yacl::key_mapping_controller::KeyMappingController;
use crate::mixins::key_mapping_accessor;
use ::yacl::dev::isxander::yacl3::api::button_option as ybutton_option;
use ::yacl::dev::isxander::yacl3::api::controller::Controller as YController;
use ::yacl::dev::isxander::yacl3::api::label_option as ylabel_option;
use ::yacl::dev::isxander::yacl3::api::option as yoption;
use ::yacl::dev::isxander::yacl3::api::option::{AnyOption as YAnyOption, Option as YOption};
use ::yacl::dev::isxander::yacl3::api::option_description;
use ::yacl::dev::isxander::yacl3::api::option_event_listener::{Event, OptionEventListener};
use ::yacl::dev::isxander::yacl3::api::option_flag::OptionFlag as YOptionFlag;
use mcrust::com::mojang::blaze3d::platform::input_constants::Key;
use mcrust::net::minecraft::client::key_mapping;
use mcrust::net::minecraft::client::minecraft::Minecraft;

/// java: public class YACLOptionAdapter
pub struct YACLOptionAdapter {}

impl YACLOptionAdapter {
    /// java: protected static List<dev.isxander.yacl3.api.Option<?>> toYaclOptions(ConfigManager<?> manager,
    /// List<? extends net.azureaaron.dandelion.api.Option<?>> options)
    pub fn to_yacl_options(manager: &Arc<dyn AnyConfigManager>, options: Vec<Arc<dyn AnyOption>>) -> Vec<Arc<dyn YAnyOption>> {
        let mut yacl_options: Vec<Arc<dyn YAnyOption>> = Vec::new();
        for option in options {
            // java: toYaclOption(manager, option) - the generic method with the option's own `T`
            yacl_options.push(option.to_yacl_option(manager));
        }
        yacl_options
    }

    /// java: @SuppressWarnings("unchecked") private static <T> dev.isxander.yacl3.api.Option<T> toYaclOption(
    /// ConfigManager<?> manager, net.azureaaron.dandelion.api.Option<T> option)
    pub fn to_yacl_option<T: Clone + PartialEq + 'static>(manager: &Arc<dyn AnyConfigManager>, option: Arc<dyn Option<T>>) -> Arc<dyn YAnyOption> {
        let as_any: Arc<dyn AnyOption> = option.clone();
        // case net.azureaaron.dandelion.api.ButtonOption button
        if let Some(button) = as_any.clone().as_button_option() {
            let action_button = button.clone();
            let yacl: Arc<dyn ybutton_option::ButtonOption> = ybutton_option::create_builder()
                .name(button.name())
                .description(option_description::create_builder().text(button.description()).build())
                .text(button.prompt())
                .action(move |_screen, _| action_button.action().accept(compat::ScreenRef::Current))
                .build();
            return yacl;
        }
        // case net.azureaaron.dandelion.api.KeyMappingOption keyMapping
        if let Some(key_mapping) = as_any.clone().as_key_mapping_option() {
            let mapping = key_mapping.key_mapping();
            let get_mapping = mapping.clone();
            let set_mapping = mapping.clone();
            let yacl: Arc<dyn YOption<Key>> = yoption::create_builder::<Key>()
                .name(key_mapping.name())
                .description(option_description::create_builder().text(key_mapping.description()).build())
                .binding_3(
                    mapping.get_default_key().clone(),
                    move || key_mapping_accessor::get_key(&get_mapping),
                    move |new_value: Key| {
                        // java: keyMapping.keyMapping().setKey(newValue); - on the live mapping in `ALL`
                        key_mapping::set_key(set_mapping.get_name(), new_value);
                        // java: KeyMapping.resetMapping(); - the port derives MAP from ALL, so the new key is indexed
                        // the moment it is written
                    },
                )
                .custom_controller(|yacl_option: Arc<dyn YOption<Key>>| -> Arc<dyn YController<Key>> { Arc::new(KeyMappingController::new(yacl_option)) })
                .build();
            return yacl;
        }
        // case net.azureaaron.dandelion.api.LabelOption label
        if let Some(label) = as_any.clone().as_label_option() {
            let yacl: Arc<dyn ylabel_option::LabelOption> = ylabel_option::create_builder().line(label.label()).build();
            return yacl;
        }
        // case net.azureaaron.dandelion.api.Option<T> _
        let binding = option.binding();
        let get_binding = binding.clone();
        let set_binding = binding.clone();
        let controller_option = option.clone();
        let patched = manager
            .as_config_manager_impl()
            .expect("ClassCastException: the manager is not a ConfigManagerImpl")
            .is_option_patched(option.id().as_ref());
        let yacl: Arc<dyn YOption<T>> = yoption::create_builder::<T>()
            .name(option.name())
            .description(option_description::create_builder().text(option.description()).build())
            .binding_3(binding.default_value(), move || get_binding.get(), move |value: T| set_binding.set(value))
            .add_listeners(YACLOptionAdapter::to_yacl_option_event_listener(&option))
            .controller(move |yacl_option: Arc<dyn YOption<T>>| controller_option.controller().controller_yacl(yacl_option, controller_option.r#type()))
            .available(option.modifiable() && !patched)
            .flags(YACLOptionAdapter::to_yacl_option_flags(&*option))
            .build();
        yacl
    }

    /// java: protected static <T> List<OptionEventListener<T>> toYaclOptionEventListener(net.azureaaron.dandelion.api.Option<T> option)
    pub fn to_yacl_option_event_listener<T: Clone + PartialEq + 'static>(option: &Arc<dyn Option<T>>) -> Vec<Arc<dyn OptionEventListener<T>>> {
        let mut yacl_option_event_listeners: Vec<Arc<dyn OptionEventListener<T>>> = Vec::new();
        for listener in option.listeners() {
            let option = option.clone();
            let yacl_listener: Arc<dyn OptionEventListener<T>> = Arc::new(move |_yacl_option: &dyn YOption<T>, event: Event| {
                listener.on_update(&*option, YACLOptionAdapter::convert_update_type(event));
            });
            yacl_option_event_listeners.push(yacl_listener);
        }
        yacl_option_event_listeners
    }

    /// java: private static OptionListener.UpdateType convertUpdateType(OptionEventListener.Event event)
    fn convert_update_type(event: Event) -> UpdateType {
        match event {
            Event::StateChange => UpdateType::ValueChange,
            _ => UpdateType::YaclUnsupported,
        }
    }

    /// java: protected static List<dev.isxander.yacl3.api.OptionFlag> toYaclOptionFlags(net.azureaaron.dandelion.api.Option<?> option)
    pub fn to_yacl_option_flags(option: &dyn AnyOption) -> Vec<Arc<dyn YOptionFlag>> {
        let mut yacl_option_flags: Vec<Arc<dyn YOptionFlag>> = Vec::new();
        for flag in option.flags() {
            // Convert Dandelion's OptionFlag to YACL's OptionFlag
            let yacl_flag: Arc<dyn YOptionFlag> = Arc::new(move |client: &mut Minecraft| flag.accept(client));
            yacl_option_flags.push(yacl_flag);
        }
        yacl_option_flags
    }
}
