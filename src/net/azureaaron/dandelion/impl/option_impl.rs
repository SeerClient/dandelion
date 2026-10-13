//! Ported from net.azureaaron.dandelion.impl.OptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::controllers::controller::{Controller, ControllerKind, NumberControllerKind};
use crate::net::azureaaron::dandelion::api::option::{self, AnyOption, Option};
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_editable_option_adapter::{EditableOptionFactory, MoulConfigEditableOptionAdapter};
use crate::net::azureaaron::dandelion::r#impl::option_binding_impl::OptionBindingImpl;
use crate::net::azureaaron::dandelion::r#impl::utils::reflection_utils::ReflectionUtils;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_adapter::YACLOptionAdapter;
use ::yacl::dev::isxander::yacl3::api::option::AnyOption as YAnyOption;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::item::Item;
use std::any::TypeId;

/// java: public class OptionImpl<T> implements Option<T>
pub struct OptionImpl<T: Clone + PartialEq + 'static> {
    /// java: private final @Nullable Identifier id;
    pub id: std::option::Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
    /// java: private final OptionBinding<T> binding;
    pub binding: Arc<dyn OptionBinding<T>>,
    /// java: private final Controller<T> controller;
    pub controller: Arc<dyn Controller<T>>,
    /// java: private final boolean modifiable;
    pub modifiable: bool,
    /// java: private final List<OptionFlag> flags;
    pub flags: Vec<Arc<dyn OptionFlag>>,
    /// java: private final List<OptionListener<T>> listeners;
    pub listeners: Vec<Arc<dyn OptionListener<T>>>,
    /// java: private final Class<T> type;
    pub r#type: compat::ClassOf<T>,
}

impl<T: Clone + PartialEq + 'static> OptionImpl<T> {
    /// java: @SuppressWarnings("unchecked") protected OptionImpl(@Nullable Identifier id, Component name,
    /// List<Component> description, List<Component> tags, OptionBinding<T> binding, Controller<T> controller,
    /// boolean modifiable, List<OptionFlag> flags, List<OptionListener<T>> listeners)
    pub fn new(
        id: std::option::Option<Identifier>,
        name: MutableComponent,
        description: Vec<MutableComponent>,
        tags: Vec<MutableComponent>,
        binding: Arc<dyn OptionBinding<T>>,
        controller: Arc<dyn Controller<T>>,
        modifiable: bool,
        flags: Vec<Arc<dyn OptionFlag>>,
        listeners: Vec<Arc<dyn OptionListener<T>>>,
    ) -> Self {
        // java: this.type = (Class<T>) ReflectionUtils.getActualClass(Objects.requireNonNull(this.binding.defaultValue(), ..));
        let r#type = ReflectionUtils::get_actual_class(&binding.default_value());
        let this = Self { id, name, description, tags, binding, controller, modifiable, flags, listeners, r#type };
        // Require that we receive the correct type, otherwise the library will likely crash later!
        OptionImpl::check_type(&this);
        this
    }

    /// java: protected static <T> void checkType(Option<T> option)
    ///
    /// Checks that the `type` of the option matches the type required by the `controller`.
    pub fn check_type(option: &dyn Option<T>) {
        let r#type = option.r#type();
        let ty = TypeId::of::<T>();
        let controller = option.controller();
        let has_correct_type = match controller.kind() {
            ControllerKind::Boolean(_) => ty == TypeId::of::<bool>(),
            ControllerKind::Colour(_) => ty == TypeId::of::<Color>(),
            // `option.type().isEnum()`: an `EnumController<T>` exists only for `T: JavaEnum`
            ControllerKind::Enum(_) => true,
            ControllerKind::Number(NumberControllerKind::Float(_)) => ty == TypeId::of::<f32>(),
            ControllerKind::Number(NumberControllerKind::Integer(_)) => ty == TypeId::of::<i32>(),
            // `Item.class.isAssignableFrom(option.type())`
            ControllerKind::Item(_) => ty == TypeId::of::<compat::JavaRef<Item>>(),
            ControllerKind::String(_) => ty == TypeId::of::<String>(),
        };
        if !has_correct_type {
            let name = match option.id() {
                Some(id) => id.to_string(),
                None => option.name().get_string(),
            };
            // All the controller interface classes implement their respective interface so this *should* always yield
            // one (and the correct one at that!)
            let (controller_interface_type, expected) = match controller.kind() {
                ControllerKind::Boolean(_) => ("BooleanController", "boolean"),
                ControllerKind::Colour(_) => ("ColourController", "Color"),
                ControllerKind::Enum(_) => ("EnumController", "Enum"),
                ControllerKind::Number(NumberControllerKind::Float(_)) => ("FloatController", "float"),
                ControllerKind::Number(NumberControllerKind::Integer(_)) => ("IntegerController", "int"),
                ControllerKind::Item(_) => ("ItemController", "Item"),
                ControllerKind::String(_) => ("StringController", "String"),
            };
            let message = format!(
                "[Dandelion] Option {} has mismatched type with controller (interface net.azureaaron.dandelion.api.controllers.{})! Expected a {} but got {}!",
                name,
                controller_interface_type,
                expected,
                compat::jstring(&r#type)
            );
            panic!("RuntimeException: {message}");
        }
    }
}

impl<T: Clone + PartialEq + 'static> AnyOption for OptionImpl<T> {
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
        self.modifiable
    }

    /// java: @Override public List<OptionFlag> flags()
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>> {
        self.flags.clone()
    }

    fn to_yacl_option(self: Arc<Self>, manager: &Arc<dyn AnyConfigManager>) -> Arc<dyn YAnyOption> {
        YACLOptionAdapter::to_yacl_option::<T>(manager, self)
    }

    fn create_editable_option_factory(self: Arc<Self>) -> EditableOptionFactory {
        MoulConfigEditableOptionAdapter::create_editable_option_factory::<T>(self)
    }
}

impl<T: Clone + PartialEq + 'static> Option<T> for OptionImpl<T> {
    /// java: @Override public OptionBinding<T> binding()
    fn binding(&self) -> Arc<dyn OptionBinding<T>> {
        self.binding.clone()
    }

    /// java: @Override public Controller<T> controller()
    fn controller(&self) -> Arc<dyn Controller<T>> {
        self.controller.clone()
    }

    /// java: @Override public List<OptionListener<T>> listeners()
    fn listeners(&self) -> Vec<Arc<dyn OptionListener<T>>> {
        self.listeners.clone()
    }

    /// java: @Override public Class<T> type()
    fn r#type(&self) -> compat::ClassOf<T> {
        self.r#type
    }
}

/// java: public static class OptionBuilderImpl<T> implements Option.Builder<T>
pub struct OptionBuilderImpl<T: Clone + PartialEq + 'static> {
    /// java: private @Nullable Identifier id = null;
    pub id: std::option::Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private @Nullable OptionBinding<T> binding = null;
    pub binding: std::option::Option<Arc<dyn OptionBinding<T>>>,
    /// java: private @Nullable Controller<T> controller = null;
    pub controller: std::option::Option<Arc<dyn Controller<T>>>,
    /// java: private boolean modifiable = true;
    pub modifiable: bool,
    /// java: private List<OptionFlag> flags = List.of();
    pub flags: Vec<Arc<dyn OptionFlag>>,
    /// java: private List<OptionListener<T>> listeners = new ArrayList<>();
    pub listeners: Vec<Arc<dyn OptionListener<T>>>,
}

impl<T: Clone + PartialEq + 'static> OptionBuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {
            id: None,
            name: component::empty(),
            description: Vec::new(),
            tags: Vec::new(),
            binding: None,
            controller: None,
            modifiable: true,
            flags: Vec::new(),
            listeners: Vec::new(),
        }
    }
}

impl<T: Clone + PartialEq + 'static> option::Builder<T> for OptionBuilderImpl<T> {
    /// java: @Override public Builder<T> id(Identifier id)
    fn id(mut self, id: Identifier) -> Self {
        self.id = Some(id);
        self
    }

    /// java: @Override public Builder<T> name(Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = name;
        self
    }

    /// java: @Override public Builder<T> description(Component... texts)
    fn description(mut self, texts: Vec<MutableComponent>) -> Self {
        self.description = texts;
        self
    }

    /// java: @Override public Builder<T> tags(Component... tags)
    fn tags(mut self, tags: Vec<MutableComponent>) -> Self {
        self.tags = tags;
        self
    }

    /// java: @Override public Builder<T> binding(T defaultValue, Supplier<T> getter, Consumer<T> setter)
    fn binding(mut self, default_value: T, getter: impl Fn() -> T + 'static, setter: impl Fn(T) + 'static) -> Self {
        self.binding = Some(Arc::new(OptionBindingImpl::new(default_value, Arc::new(getter), Arc::new(setter))));
        self
    }

    /// java: @Override public Builder<T> controller(Controller<T> controller)
    fn controller(mut self, controller: Arc<dyn Controller<T>>) -> Self {
        self.controller = Some(controller);
        self
    }

    /// java: @Override public Builder<T> modifiable(boolean modifiable)
    fn modifiable(mut self, modifiable: bool) -> Self {
        self.modifiable = modifiable;
        self
    }

    /// java: @Override public Builder<T> flags(OptionFlag... flags)
    fn flags(mut self, flags: Vec<Arc<dyn OptionFlag>>) -> Self {
        self.flags = flags;
        self
    }

    /// java: @Override public Builder<T> listener(OptionListener<T> listener)
    fn listener(mut self, listener: Arc<dyn OptionListener<T>>) -> Self {
        self.listeners.push(listener);
        self
    }

    /// java: @Override public Option<T> build()
    fn build(self) -> Arc<dyn Option<T>> {
        let binding = self.binding.expect("NullPointerException: a binding is required.");
        let controller = self.controller.expect("NullPointerException: a controller is required.");
        Arc::new(OptionImpl::new(self.id, self.name, self.description, self.tags, binding, controller, self.modifiable, self.flags, self.listeners))
    }
}
