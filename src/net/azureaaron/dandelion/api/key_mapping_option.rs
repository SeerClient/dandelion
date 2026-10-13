//! Ported from net.azureaaron.dandelion.api.KeyMappingOption.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option::Option;
use crate::net::azureaaron::dandelion::r#impl::key_mapping_option_impl::KeyMappingOptionBuilder;
use mcrust::net::minecraft::client::key_mapping::KeyMapping;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface KeyMappingOption extends Option<KeyMapping>
///
/// A `KeyMapping` is a value here (the port keeps the LIVE binding in the registry `KeyMapping.ALL`, reached by the
/// mapping's name: `key_mapping::key_of` / `key_mapping::set_key`).
pub trait KeyMappingOption: Option<KeyMapping> {
    /// java: KeyMapping keyMapping();
    fn key_mapping(&self) -> KeyMapping;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> KeyMappingOptionBuilder {
    KeyMappingOptionBuilder::new()
}

/// java: interface Builder
pub trait Builder: Sized {
    /// java: Builder id(Identifier id);
    fn id(self, id: Identifier) -> Self;

    /// java: Builder name(Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder description(Component... texts);
    fn description(self, texts: Vec<MutableComponent>) -> Self;

    /// java: Builder tags(Component... tags);
    fn tags(self, tags: Vec<MutableComponent>) -> Self;

    /// java: Builder keyMapping(KeyMapping keyMapping);
    fn key_mapping(self, key_mapping: KeyMapping) -> Self;

    /// java: KeyMappingOption build();
    fn build(self) -> Arc<dyn KeyMappingOption>;
}
