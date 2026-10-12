//! Ported from net.azureaaron.dandelion.api.KeyMappingOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::key_mapping_option_impl::KeyMappingOptionBuilder;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface KeyMappingOption extends Option<KeyMapping>
pub trait KeyMappingOption: crate::net::azureaaron::dandelion::api::option::Option + compat::DynAny {
    /// java: KeyMapping keyMapping(); (dispatch: the body for this class)
    fn key_mapping(&self) -> fn(&compat::Shared<dyn KeyMappingOption>) -> mcrust::net::minecraft::client::key_mapping::KeyMapping;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (KeyMappingOptionBuilder::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder
pub trait Builder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder id(Identifier id); (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn Builder>, Identifier) -> compat::Shared<dyn Builder>;

    /// java: Builder name(Component name); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder description(Component... texts); (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: Builder tags(Component... tags); (dispatch: the body for this class)
    fn tags(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: Builder keyMapping(KeyMapping keyMapping); (dispatch: the body for this class)
    fn key_mapping(&self) -> fn(&compat::Shared<dyn Builder>, mcrust::net::minecraft::client::key_mapping::KeyMapping) -> compat::Shared<dyn Builder>;

    /// java: KeyMappingOption build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn KeyMappingOption>;

}
