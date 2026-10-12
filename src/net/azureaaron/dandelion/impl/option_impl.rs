//! Ported from net.azureaaron.dandelion.impl.OptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::option::Builder;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use crate::net::azureaaron::dandelion::r#impl::option_binding_impl::OptionBindingImpl;
use crate::net::azureaaron::dandelion::r#impl::utils::reflection_utils::ReflectionUtils;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use crate::net::azureaaron::dandelion::api::option::Option as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class OptionImpl<T> implements Option<T>
pub struct OptionImpl<T> {
    /// java: private final @Nullable Identifier id;
    pub id: Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
    /// java: private final OptionBinding<T> binding;
    pub binding: compat::Shared<dyn OptionBinding<T>>,
    /// java: private final Controller<T> controller;
    pub controller: compat::Shared<dyn Controller<T>>,
    /// java: private final boolean modifiable;
    pub modifiable: bool,
    /// java: private final List<OptionFlag> flags;
    pub flags: Vec<compat::Shared<dyn OptionFlag>>,
    /// java: private final List<OptionListener<T>> listeners;
    pub listeners: Vec<compat::Shared<dyn OptionListener<T>>>,
    /// java: private final Class<T> type;
    pub r#type: Option<compat::ClassOf<T>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: Clone + 'static> OptionImpl<T> {
    /// java: @SuppressWarnings("unchecked") protected OptionImpl(@Nullable Identifier id, Component name, List<Component> description, List<Component> tags, OptionBinding<T> binding, Controller<T> controller, bool
    pub fn new(mut id: Option<Identifier>, mut name: MutableComponent, mut description: Vec<MutableComponent>, mut tags: Vec<MutableComponent>, mut binding: compat::Shared<dyn OptionBinding<T>>, mut controller: compat::Shared<dyn Controller<T>>, mut modifiable: bool, mut flags: Vec<compat::Shared<dyn OptionFlag>>, mut listeners: Vec<compat::Shared<dyn OptionListener<T>>>) -> compat::Shared<Self> {
        let __init_id = id.clone();
        let __init_name = name.clone();
        let __init_description = description.clone();
        let __init_tags = tags.clone();
        let __init_binding = binding.clone();
        let __init_controller = controller.clone();
        let __init_modifiable = modifiable;
        let __init_flags = flags.clone();
        let __init_listeners = listeners.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            id: __init_id,
            name: __init_name,
            description: __init_description,
            tags: __init_tags,
            binding: __init_binding,
            controller: __init_controller,
            modifiable: __init_modifiable,
            flags: __init_flags,
            listeners: __init_listeners,
            r#type: None,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(ReflectionUtils::get_actual_class(&__this.lock().borrow().binding.default_value())); compat::write(&__this, |__t| __t.r#type = __v); }
        OptionImpl::check_type(&(__this.clone() as compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>));
        __this
    }

    /// java: protected static <T> void checkType(Option<T> option)
    pub fn check_type<T: 'static>(option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>) {
        // TODO(fabric2seer): switch expression (not parsed)
        //   java: boolean hasCorrectType = switch (option.controller()) {
        //   java: 			case BooleanController _ -> option.type() == boolean.class || option.type() == Boolean.class;
        //   java: 			case ColourController _ -> option.type() == Color.class;
        //   java: 			case EnumController<?> _ -> option.type().isEnum();
        //   java: 			case FloatController _ -> option.type() == float.class || option.type() == Float.class;
        //   java: 			case IntegerController _ -> option.type() == int.class || option.type() == Integer.class;
        //   java: 			case ItemController _ -> Item.class.isAssignableFrom(option.type());
        //   java: 			case StringController _ -> option.type() == String.class;
        //   java: 		};
        let mut has_correct_type: bool = todo!("fabric2seer: the initialiser of `hasCorrectType` (TODO above)");
        if !has_correct_type {
            // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
            // rust: let mut name: String = (if option.lock().borrow().id().is_some() { option.lock().borrow().id().unwrap().fmt() } else { option.lock().borrow().name().get_string() });
            let mut name: String = todo!("fabric2seer: the initialiser of `name` (TODO above)");
            // TODO(fabric2seer): `.getInterfaces(..)` on a Class
            //   java: Class<?> controllerInterfaceType = option.controller().getClass().getInterfaces()[0];
            let mut controller_interface_type: compat::ClassOf<compat::Object> = todo!("fabric2seer: the initialiser of `controllerInterfaceType` (TODO above)");
            // TODO(fabric2seer): a pattern switch expression
            //   java: String expected = switch (option.controller()) {
            //   java: 				case BooleanController _ -> "boolean";
            //   java: 				case ColourController _ -> "Color";
            //   java: 				case EnumController<?> _ -> "Enum";
            //   java: 				case FloatController _ -> "float";
            //   java: 				case IntegerController _ -> "int";
            //   java: 				case ItemController _ -> "Item";
            //   java: 				case StringController _ -> "String";
            //   java: 			};
            let mut expected: String = todo!("fabric2seer: the initialiser of `expected` (TODO above)");
            let mut message: String = format!("[Dandelion] Option {} has mismatched type with controller ({})! Expected a {} but got {}!", name, compat::jstring(&controller_interface_type), expected, compat::jstring(&option.lock().borrow().r#type()));
            panic!("RuntimeException: {}", message.to_string());
        }
    }

}

impl<T: Clone + 'static> crate::net::azureaaron::dandelion::api::option::Option<T> for OptionImpl<T> {
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

    /// java: @Override public OptionBinding<T> binding()
    fn binding(&self) -> compat::Shared<dyn OptionBinding<T>> {
        return self.binding.clone();
    }

    /// java: @Override public Controller<T> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<T>> {
        return self.controller.clone();
    }

    /// java: @Override public boolean modifiable()
    fn modifiable(&self) -> bool {
        return self.modifiable;
    }

    /// java: @Override public List<OptionFlag> flags()
    fn flags(&self) -> Vec<compat::Shared<dyn OptionFlag>> {
        return self.flags.clone();
    }

    /// java: @Override public List<OptionListener<T>> listeners()
    fn listeners(&self) -> Vec<compat::Shared<dyn OptionListener<T>>> {
        return self.listeners.clone();
    }

    /// java: @Override public Class<T> type()
    fn r#type(&self) -> compat::ClassOf<T> {
        return self.r#type.clone().unwrap();
    }

}

/// java: public static class OptionBuilderImpl<T> implements Option.Builder<T>
#[derive(Clone)]
pub struct OptionBuilderImpl<T> {
    /// java: private @Nullable Identifier id = null;
    pub id: Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private @Nullable OptionBinding<T> binding = null;
    pub binding: Option<compat::Shared<dyn OptionBinding<T>>>,
    /// java: private @Nullable Controller<T> controller = null;
    pub controller: Option<compat::Shared<dyn Controller<T>>>,
    /// java: private boolean modifiable = true;
    pub modifiable: bool,
    /// java: private List<OptionFlag> flags = List.of();
    pub flags: Vec<compat::Shared<dyn OptionFlag>>,
    /// java: private List<OptionListener<T>> listeners = new ArrayList<>();
    pub listeners: Vec<compat::Shared<dyn OptionListener<T>>>,
}

impl<T: Clone + 'static> OptionBuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            id: None,
            name: component::empty(),
            description: vec![],
            tags: vec![],
            binding: None,
            controller: None,
            modifiable: true,
            flags: vec![],
            listeners: Vec::new(),
        };
        this
    }

}

impl<T: Clone + 'static> Builder<T> for OptionBuilderImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder<T> id(Identifier id)
    fn id(&mut self, mut id: Identifier) -> compat::Shared<dyn Builder<T>> {
        self.id = Some(id.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> name(Component name)
    fn name(&mut self, mut name: MutableComponent) -> compat::Shared<dyn Builder<T>> {
        self.name = name.clone();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> description(Component... texts)
    fn description(&mut self, texts: &Vec<MutableComponent>) -> compat::Shared<dyn Builder<T>> {
        self.description = texts.to_vec();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> tags(Component... tags)
    fn tags(&mut self, tags: &Vec<MutableComponent>) -> compat::Shared<dyn Builder<T>> {
        self.tags = tags.to_vec();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> binding(T defaultValue, Supplier<T> getter, Consumer<T> setter)
    fn binding(&mut self, default_value: &T, getter: Arc<dyn Fn() -> T>, setter: Arc<dyn Fn(T)>) -> compat::Shared<dyn Builder<T>> {
        self.binding = Some((compat::shared(OptionBindingImpl::new(default_value.clone(), getter.clone(), setter.clone())) as compat::Shared<dyn OptionBinding<compat::Object>>));
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> controller(Controller<T> controller)
    fn controller(&mut self, mut controller: compat::Shared<dyn Controller<T>>) -> compat::Shared<dyn Builder<T>> {
        self.controller = Some(controller.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> modifiable(boolean modifiable)
    fn modifiable(&mut self, mut modifiable: bool) -> compat::Shared<dyn Builder<T>> {
        self.modifiable = modifiable;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> flags(OptionFlag... flags)
    fn flags(&mut self, flags: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>> {
        self.flags = flags.to_vec();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Builder<T> listener(OptionListener<T> listener)
    fn listener(&mut self, mut listener: compat::Shared<dyn OptionListener<T>>) -> compat::Shared<dyn Builder<T>> {
        self.listeners.push(listener.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public Option<T> build()
    fn build(&self) -> compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>> {
        self.binding;
        self.controller;
        return (OptionImpl::new(self.id.clone(), self.name.clone(), self.description.clone(), self.tags.clone(), self.binding.clone().unwrap(), self.controller.clone().unwrap(), self.modifiable, self.flags.clone(), self.listeners.iter().cloned().collect::<Vec<_>>()) as compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>);
    }

}
