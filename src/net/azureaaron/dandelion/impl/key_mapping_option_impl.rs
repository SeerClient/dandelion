//! Ported from net.azureaaron.dandelion.impl.KeyMappingOptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::key_mapping_option::{self, KeyMappingOption};
use crate::net::azureaaron::dandelion::api::option::{AnyOption, Option};
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_editable_option_adapter::{EditableOptionFactory, MoulConfigEditableOptionAdapter};
use crate::net::azureaaron::dandelion::r#impl::utils::reflection_utils::ReflectionUtils;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_adapter::YACLOptionAdapter;
use ::yacl::dev::isxander::yacl3::api::option::AnyOption as YAnyOption;
use mcrust::net::minecraft::client::key_mapping::KeyMapping;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class KeyMappingOptionImpl implements KeyMappingOption
pub struct KeyMappingOptionImpl {
    /// java: private final @Nullable Identifier id;
    pub id: std::option::Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
    /// java: private final KeyMapping keyMapping;
    pub key_mapping: KeyMapping,
}

impl KeyMappingOptionImpl {
    /// java: protected KeyMappingOptionImpl(@Nullable Identifier id, Component name, List<Component> description,
    /// List<Component> tags, KeyMapping keyMapping)
    pub fn new(
        id: std::option::Option<Identifier>,
        name: MutableComponent,
        description: Vec<MutableComponent>,
        tags: Vec<MutableComponent>,
        key_mapping: std::option::Option<KeyMapping>,
    ) -> Self {
        let key_mapping = key_mapping.expect("NullPointerException: keyMapping must not be null");
        Self { id, name, description, tags, key_mapping }
    }
}

impl AnyOption for KeyMappingOptionImpl {
    /// java: @Override public @Nullable Identifier id()
    fn id(&self) -> std::option::Option<Identifier> {
        self.id.clone()
    }

    /// java: @Override public Component name()
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: @Override public List<Component> description()
    fn description(&self) -> Vec<MutableComponent> {
        self.description.clone()
    }

    /// java: @Override public List<Component> tags()
    fn tags(&self) -> Vec<MutableComponent> {
        self.tags.clone()
    }

    /// java: @Override public boolean modifiable()
    fn modifiable(&self) -> bool {
        // NYI
        true
    }

    /// java: @Override public List<OptionFlag> flags()
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>> {
        panic!("UnsupportedOperationException")
    }

    fn as_key_mapping_option(self: Arc<Self>) -> std::option::Option<Arc<dyn KeyMappingOption>> {
        Some(self)
    }

    fn to_yacl_option(self: Arc<Self>, manager: &Arc<dyn AnyConfigManager>) -> Arc<dyn YAnyOption> {
        YACLOptionAdapter::to_yacl_option::<KeyMapping>(manager, self)
    }

    fn create_editable_option_factory(self: Arc<Self>) -> EditableOptionFactory {
        MoulConfigEditableOptionAdapter::create_editable_option_factory::<KeyMapping>(self)
    }
}

impl Option<KeyMapping> for KeyMappingOptionImpl {
    /// java: @Override public OptionBinding<KeyMapping> binding()
    fn binding(&self) -> Arc<dyn OptionBinding<KeyMapping>> {
        panic!("UnsupportedOperationException")
    }

    /// java: @Override public Controller<KeyMapping> controller()
    fn controller(&self) -> Arc<dyn Controller<KeyMapping>> {
        panic!("UnsupportedOperationException")
    }

    /// java: @Override public List<OptionListener<KeyMapping>> listeners()
    fn listeners(&self) -> Vec<Arc<dyn OptionListener<KeyMapping>>> {
        panic!("UnsupportedOperationException")
    }

    /// java: @Override public Class<KeyMapping> type()
    fn r#type(&self) -> compat::ClassOf<KeyMapping> {
        // java: KeyMapping.class
        ReflectionUtils::get_actual_class(&self.key_mapping)
    }
}

impl KeyMappingOption for KeyMappingOptionImpl {
    /// java: @Override public KeyMapping keyMapping()
    fn key_mapping(&self) -> KeyMapping {
        self.key_mapping.clone()
    }
}

/// java: public static class KeyMappingOptionBuilder implements KeyMappingOption.Builder
pub struct KeyMappingOptionBuilder {
    /// java: private @Nullable Identifier id = null;
    pub id: std::option::Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private @Nullable KeyMapping keyMapping = null;
    pub key_mapping: std::option::Option<KeyMapping>,
}

impl KeyMappingOptionBuilder {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { id: None, name: component::empty(), description: Vec::new(), tags: Vec::new(), key_mapping: None }
    }
}

impl key_mapping_option::Builder for KeyMappingOptionBuilder {
    /// java: @Override public KeyMappingOption.Builder id(Identifier id)
    fn id(mut self, id: Identifier) -> Self {
        self.id = Some(id);
        self
    }

    /// java: @Override public KeyMappingOption.Builder name(Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = name;
        self
    }

    /// java: @Override public KeyMappingOption.Builder description(Component... texts)
    fn description(mut self, texts: Vec<MutableComponent>) -> Self {
        self.description = texts;
        self
    }

    /// java: @Override public KeyMappingOption.Builder tags(Component... tags)
    fn tags(mut self, tags: Vec<MutableComponent>) -> Self {
        self.tags = tags;
        self
    }

    /// java: @Override public KeyMappingOption.Builder keyMapping(KeyMapping keyMapping)
    fn key_mapping(mut self, key_mapping: KeyMapping) -> Self {
        self.key_mapping = Some(key_mapping);
        self
    }

    /// java: @Override public KeyMappingOption build()
    fn build(self) -> Arc<dyn KeyMappingOption> {
        Arc::new(KeyMappingOptionImpl::new(self.id, self.name, self.description, self.tags, self.key_mapping))
    }
}
