//! Ported from net.azureaaron.dandelion.impl.LabelOptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::label_option::{self, LabelOption};
use crate::net::azureaaron::dandelion::api::option::{AnyOption, Option};
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_editable_option_adapter::{EditableOptionFactory, MoulConfigEditableOptionAdapter};
use crate::net::azureaaron::dandelion::r#impl::utils::reflection_utils::ReflectionUtils;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_adapter::YACLOptionAdapter;
use ::yacl::dev::isxander::yacl3::api::option::AnyOption as YAnyOption;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class LabelOptionImpl implements LabelOption
pub struct LabelOptionImpl {
    /// java: private final Component label;
    pub label: MutableComponent,
}

impl LabelOptionImpl {
    /// java: protected LabelOptionImpl(Component label)
    pub fn new(label: MutableComponent) -> Self {
        Self { label }
    }
}

impl AnyOption for LabelOptionImpl {
    /// java: @Override public @Nullable Identifier id()
    fn id(&self) -> std::option::Option<Identifier> {
        panic!("UnsupportedOperationException")
    }

    /// java: @Override public Component name()
    fn name(&self) -> MutableComponent {
        component::empty()
    }

    /// java: @Override public List<Component> description()
    fn description(&self) -> Vec<MutableComponent> {
        Vec::new()
    }

    /// java: @Override public List<Component> tags()
    fn tags(&self) -> Vec<MutableComponent> {
        Vec::new()
    }

    /// java: @Override public boolean modifiable()
    fn modifiable(&self) -> bool {
        true
    }

    /// java: @Override public List<OptionFlag> flags()
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>> {
        panic!("UnsupportedOperationException")
    }

    fn as_label_option(self: Arc<Self>) -> std::option::Option<Arc<dyn LabelOption>> {
        Some(self)
    }

    fn to_yacl_option(self: Arc<Self>, manager: &Arc<dyn AnyConfigManager>) -> Arc<dyn YAnyOption> {
        YACLOptionAdapter::to_yacl_option::<MutableComponent>(manager, self)
    }

    fn create_editable_option_factory(self: Arc<Self>) -> EditableOptionFactory {
        MoulConfigEditableOptionAdapter::create_editable_option_factory::<MutableComponent>(self)
    }
}

impl Option<MutableComponent> for LabelOptionImpl {
    /// java: @Override public OptionBinding<Component> binding()
    fn binding(&self) -> Arc<dyn OptionBinding<MutableComponent>> {
        panic!("UnsupportedOperationException")
    }

    /// java: @Override public Controller<Component> controller()
    fn controller(&self) -> Arc<dyn Controller<MutableComponent>> {
        panic!("UnsupportedOperationException")
    }

    /// java: @Override public List<OptionListener<Component>> listeners()
    fn listeners(&self) -> Vec<Arc<dyn OptionListener<MutableComponent>>> {
        panic!("UnsupportedOperationException")
    }

    /// java: @Override public Class<Component> type()
    fn r#type(&self) -> compat::ClassOf<MutableComponent> {
        // java: Component.class
        ReflectionUtils::get_actual_class(&self.label)
    }
}

impl LabelOption for LabelOptionImpl {
    /// java: @Override public Component label()
    fn label(&self) -> MutableComponent {
        self.label.clone()
    }
}

/// java: public static class LabelOptionBuilderImpl implements LabelOption.Builder
pub struct LabelOptionBuilderImpl {
    /// java: private List<Component> label = List.of();
    pub label: Vec<MutableComponent>,
}

impl LabelOptionBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { label: Vec::new() }
    }
}

impl label_option::Builder for LabelOptionBuilderImpl {
    /// java: @Override public LabelOption.Builder label(Component... texts)
    fn label(mut self, texts: Vec<MutableComponent>) -> Self {
        self.label = texts;
        self
    }

    /// java: @Override public LabelOption build()
    fn build(self) -> Arc<dyn LabelOption> {
        // java: Component concatted = MutableComponent.create(PlainTextContents.EMPTY); - `Component.empty()`
        let mut concatted = component::empty();
        for text in self.label {
            // java: concatted.getSiblings().add(text);
            concatted = concatted.append(text);
        }
        Arc::new(LabelOptionImpl::new(concatted))
    }
}
