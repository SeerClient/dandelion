//! Ported from net.azureaaron.dandelion.impl.ButtonOptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::button_option::{self, ButtonOption, ScreenConsumer};
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
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

/// java: public class ButtonOptionImpl implements ButtonOption
pub struct ButtonOptionImpl {
    /// java: private final @Nullable Identifier id;
    pub id: std::option::Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final Component prompt;
    pub prompt: MutableComponent,
    /// java: private final Consumer<Screen> action;
    pub action: ScreenConsumer,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
}

impl ButtonOptionImpl {
    /// java: protected ButtonOptionImpl(@Nullable Identifier id, Component name, List<Component> description,
    /// List<Component> tags, Component prompt, Consumer<Screen> action)
    pub fn new(
        id: std::option::Option<Identifier>,
        name: MutableComponent,
        description: Vec<MutableComponent>,
        tags: Vec<MutableComponent>,
        prompt: MutableComponent,
        action: ScreenConsumer,
    ) -> Self {
        // java: Objects.requireNonNull(..) - none of these can be null here
        Self { id, name, description, prompt, action, tags }
    }
}

impl AnyOption for ButtonOptionImpl {
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

    fn as_button_option(self: Arc<Self>) -> std::option::Option<Arc<dyn ButtonOption>> {
        Some(self)
    }

    fn to_yacl_option(self: Arc<Self>, manager: &Arc<dyn AnyConfigManager>) -> Arc<dyn YAnyOption> {
        YACLOptionAdapter::to_yacl_option::<ScreenConsumer>(manager, self)
    }

    fn create_editable_option_factory(self: Arc<Self>) -> EditableOptionFactory {
        MoulConfigEditableOptionAdapter::create_editable_option_factory::<ScreenConsumer>(self)
    }
}

impl Option<ScreenConsumer> for ButtonOptionImpl {
    /// java: @Override public OptionBinding<Consumer<Screen>> binding()
    fn binding(&self) -> Arc<dyn OptionBinding<ScreenConsumer>> {
        panic!("UnsupportedOperationException")
    }

    /// java: @Override public Controller<Consumer<Screen>> controller()
    fn controller(&self) -> Arc<dyn Controller<ScreenConsumer>> {
        panic!("UnsupportedOperationException")
    }

    /// java: @Override public List<OptionListener<Consumer<Screen>>> listeners()
    fn listeners(&self) -> Vec<Arc<dyn OptionListener<ScreenConsumer>>> {
        panic!("UnsupportedOperationException")
    }

    /// java: @SuppressWarnings("unchecked") @Override public Class<Consumer<Screen>> type()
    fn r#type(&self) -> compat::ClassOf<ScreenConsumer> {
        // java: (Class<Consumer<Screen>>) this.action.getClass()
        ReflectionUtils::get_actual_class(&self.action)
    }
}

impl ButtonOption for ButtonOptionImpl {
    /// java: @Override public Component prompt()
    fn prompt(&self) -> MutableComponent {
        self.prompt.clone()
    }

    /// java: @Override public Consumer<Screen> action()
    fn action(&self) -> ScreenConsumer {
        self.action.clone()
    }
}

/// java: public static class ButtonOptionBuilderImpl implements ButtonOption.Builder
pub struct ButtonOptionBuilderImpl {
    /// java: private @Nullable Identifier id = null;
    pub id: std::option::Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private Component prompt = Component.nullToEmpty("Execute");
    pub prompt: MutableComponent,
    /// java: private Consumer<Screen> action = Consumers.nop();
    pub action: ScreenConsumer,
}

impl ButtonOptionBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {
            id: None,
            name: component::empty(),
            description: Vec::new(),
            tags: Vec::new(),
            prompt: component::null_to_empty(Some("Execute")),
            action: ScreenConsumer::nop(),
        }
    }
}

impl button_option::Builder for ButtonOptionBuilderImpl {
    /// java: @Override public ButtonOption.Builder id(Identifier id)
    fn id(mut self, id: Identifier) -> Self {
        self.id = Some(id);
        self
    }

    /// java: @Override public ButtonOption.Builder name(Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = name;
        self
    }

    /// java: @Override public ButtonOption.Builder description(Component... texts)
    fn description(mut self, texts: Vec<MutableComponent>) -> Self {
        self.description = texts;
        self
    }

    /// java: @Override public ButtonOption.Builder tags(Component... tags)
    fn tags(mut self, tags: Vec<MutableComponent>) -> Self {
        self.tags = tags;
        self
    }

    /// java: @Override public ButtonOption.Builder prompt(Component prompt)
    fn prompt(mut self, prompt: MutableComponent) -> Self {
        self.prompt = prompt;
        self
    }

    /// java: @Override public ButtonOption.Builder action(Consumer<Screen> action)
    fn action(mut self, action: impl Fn(compat::ScreenRef) + 'static) -> Self {
        self.action = ScreenConsumer(Arc::new(action));
        self
    }

    /// java: @Override public ButtonOption build()
    fn build(self) -> Arc<dyn ButtonOption> {
        Arc::new(ButtonOptionImpl::new(self.id, self.name, self.description, self.tags, self.prompt, self.action))
    }
}
