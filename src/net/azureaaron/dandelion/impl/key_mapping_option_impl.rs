//! Ported from net.azureaaron.dandelion.impl.KeyMappingOptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::key_mapping_option::Builder;
use crate::net::azureaaron::dandelion::api::key_mapping_option::KeyMappingOption;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class KeyMappingOptionImpl implements KeyMappingOption
#[derive(Clone)]
pub struct KeyMappingOptionImpl {
    /// java: private final @Nullable Identifier id;
    pub id: Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
    /// java: private final KeyMapping keyMapping;
    pub key_mapping: mcrust::net::minecraft::client::key_mapping::KeyMapping,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl KeyMappingOptionImpl {
    /// java: protected KeyMappingOptionImpl(@Nullable Identifier id, Component name, List<Component> description, List<Component> tags, KeyMapping keyMapping)
    pub fn new(mut id: Option<Identifier>, mut name: MutableComponent, mut description: Vec<MutableComponent>, mut tags: Vec<MutableComponent>, mut key_mapping: mcrust::net::minecraft::client::key_mapping::KeyMapping) -> compat::Shared<Self> {
        let __init_id = id.clone();
        let __init_name = name.clone();
        let __init_description = description.clone();
        let __init_tags = tags.clone();
        let __init_key_mapping = key_mapping.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            id: __init_id,
            name: __init_name,
            description: __init_description,
            tags: __init_tags,
            key_mapping: __init_key_mapping,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl KeyMappingOption for KeyMappingOptionImpl {
    /// java: @Override public KeyMapping keyMapping() (dispatch: the body for this class)
    fn key_mapping(&self) -> fn(&compat::Shared<dyn KeyMappingOption>) -> mcrust::net::minecraft::client::key_mapping::KeyMapping {
        KeyMappingOptionImpl::key_mapping_dyn
    }

}

impl crate::net::azureaaron::dandelion::api::option::Option<compat::Object> for KeyMappingOptionImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable Identifier id()
    fn id(&self) -> Option<Identifier> {
        return self.id.clone();
    }

    /// java: @Override public Component name()
    fn name(&self) -> MutableComponent {
        return self.name.clone();
    }

    /// java: @Override public List<Component> description()
    fn description(&self) -> Vec<MutableComponent> {
        return self.description.clone();
    }

    /// java: @Override public List<Component> tags()
    fn tags(&self) -> Vec<MutableComponent> {
        return self.tags.clone();
    }

    /// java: @Override public OptionBinding<KeyMapping> binding()
    fn binding(&self) -> compat::Shared<dyn OptionBinding<mcrust::net::minecraft::client::key_mapping::KeyMapping>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public Controller<KeyMapping> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<mcrust::net::minecraft::client::key_mapping::KeyMapping>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public boolean modifiable()
    fn modifiable(&self) -> bool {
        return true;
    }

    /// java: @Override public List<OptionFlag> flags()
    fn flags(&self) -> Vec<compat::Shared<dyn OptionFlag>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public List<OptionListener<KeyMapping>> listeners()
    fn listeners(&self) -> Vec<compat::Shared<dyn OptionListener<mcrust::net::minecraft::client::key_mapping::KeyMapping>>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public Class<KeyMapping> type()
    fn r#type(&self) -> compat::ClassOf<mcrust::net::minecraft::client::key_mapping::KeyMapping> {
        return std::any::TypeId::of::<mcrust::net::minecraft::client::key_mapping::KeyMapping>();
    }

}

impl KeyMappingOptionImpl {
    /// java: @Override public KeyMapping keyMapping()
    pub fn key_mapping_impl(__this: &compat::Shared<Self>) -> mcrust::net::minecraft::client::key_mapping::KeyMapping {
        return compat::read(__this, |__t| __t.key_mapping.clone());
    }

    /// KeyMappingOptionImpl.keyMapping as the dispatch of KeyMappingOption.keyMapping
    pub fn key_mapping_dyn(__this: &compat::Shared<dyn KeyMappingOption>) -> mcrust::net::minecraft::client::key_mapping::KeyMapping {
        Self::key_mapping_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class KeyMappingOptionBuilder implements KeyMappingOption.Builder
#[derive(Clone)]
pub struct KeyMappingOptionBuilder {
    /// java: private @Nullable Identifier id = null;
    pub id: Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private @Nullable KeyMapping keyMapping = null;
    pub key_mapping: Option<mcrust::net::minecraft::client::key_mapping::KeyMapping>,
}

impl KeyMappingOptionBuilder {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            id: None,
            name: component::empty(),
            description: vec![],
            tags: vec![],
            key_mapping: None,
        });
        __this
    }

}

impl Builder for KeyMappingOptionBuilder {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public KeyMappingOption.Builder id(Identifier id) (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn Builder>, Identifier) -> compat::Shared<dyn Builder> {
        KeyMappingOptionBuilder::id_dyn
    }

    /// java: @Override public KeyMappingOption.Builder name(Component name) (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        KeyMappingOptionBuilder::name_dyn
    }

    /// java: @Override public KeyMappingOption.Builder description(Component... texts) (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        KeyMappingOptionBuilder::description_dyn
    }

    /// java: @Override public KeyMappingOption.Builder tags(Component... tags) (dispatch: the body for this class)
    fn tags(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        KeyMappingOptionBuilder::tags_dyn
    }

    /// java: @Override public KeyMappingOption.Builder keyMapping(KeyMapping keyMapping) (dispatch: the body for this class)
    fn key_mapping(&self) -> fn(&compat::Shared<dyn Builder>, mcrust::net::minecraft::client::key_mapping::KeyMapping) -> compat::Shared<dyn Builder> {
        KeyMappingOptionBuilder::key_mapping_dyn
    }

    /// java: @Override public KeyMappingOption build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn KeyMappingOption> {
        KeyMappingOptionBuilder::build_dyn
    }

}

impl KeyMappingOptionBuilder {
    /// java: @Override public KeyMappingOption.Builder id(Identifier id)
    pub fn id_impl(__this: &compat::Shared<Self>, mut id: Identifier) -> compat::Shared<dyn Builder> {
        { let __v = Some(id.clone()); compat::write(__this, |__t| __t.id = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// KeyMappingOptionBuilder.id as the dispatch of Builder.id
    pub fn id_dyn(__this: &compat::Shared<dyn Builder>, __p0: Identifier) -> compat::Shared<dyn Builder> {
        Self::id_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public KeyMappingOption.Builder name(Component name)
    pub fn name_impl(__this: &compat::Shared<Self>, mut name: MutableComponent) -> compat::Shared<dyn Builder> {
        { let __v = name.clone(); compat::write(__this, |__t| __t.name = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// KeyMappingOptionBuilder.name as the dispatch of Builder.name
    pub fn name_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public KeyMappingOption.Builder description(Component... texts)
    pub fn description_impl(__this: &compat::Shared<Self>, texts: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        { let __v = texts.to_vec(); compat::write(__this, |__t| __t.description = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// KeyMappingOptionBuilder.description as the dispatch of Builder.description
    pub fn description_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public KeyMappingOption.Builder tags(Component... tags)
    pub fn tags_impl(__this: &compat::Shared<Self>, tags: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        { let __v = tags.to_vec(); compat::write(__this, |__t| __t.tags = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// KeyMappingOptionBuilder.tags as the dispatch of Builder.tags
    pub fn tags_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::tags_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public KeyMappingOption.Builder keyMapping(KeyMapping keyMapping)
    pub fn key_mapping_impl(__this: &compat::Shared<Self>, mut key_mapping: mcrust::net::minecraft::client::key_mapping::KeyMapping) -> compat::Shared<dyn Builder> {
        { let __v = Some(key_mapping.clone()); compat::write(__this, |__t| __t.key_mapping = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// KeyMappingOptionBuilder.keyMapping as the dispatch of Builder.keyMapping
    pub fn key_mapping_dyn(__this: &compat::Shared<dyn Builder>, __p0: mcrust::net::minecraft::client::key_mapping::KeyMapping) -> compat::Shared<dyn Builder> {
        Self::key_mapping_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public KeyMappingOption build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn KeyMappingOption> {
        return (KeyMappingOptionImpl::new(compat::read(__this, |__t| __t.id.clone()), compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.description.clone()), compat::read(__this, |__t| __t.tags.clone()), compat::read(__this, |__t| __t.key_mapping.clone()).unwrap()) as compat::Shared<dyn KeyMappingOption>);
    }

    /// KeyMappingOptionBuilder.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn KeyMappingOption> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
