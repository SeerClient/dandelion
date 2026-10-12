//! Ported from net.azureaaron.dandelion.impl.ListOptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::list_option::Builder;
use crate::net::azureaaron::dandelion::api::list_option::ListOption;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use crate::net::azureaaron::dandelion::r#impl::option_binding_impl::OptionBindingImpl;
use crate::net::azureaaron::dandelion::r#impl::option_impl::OptionImpl;
use crate::net::azureaaron::dandelion::r#impl::utils::reflection_utils::ReflectionUtils;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class ListOptionImpl<T> implements ListOption<T>
pub struct ListOptionImpl<T> {
    /// java: private final @Nullable Identifier id;
    pub id: Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
    /// java: private final OptionBinding<List<T>> binding;
    pub binding: compat::Shared<dyn OptionBinding<Vec<T>>>,
    /// java: private final Controller<T> controller;
    pub controller: compat::Shared<dyn Controller<T>>,
    /// java: private final Supplier<T> initialValue;
    pub initial_value: Arc<dyn Fn() -> T>,
    /// java: private final boolean collapsed;
    pub collapsed: bool,
    /// java: private final boolean modifiable;
    pub modifiable: bool,
    /// java: private final List<OptionFlag> flags;
    pub flags: Vec<compat::Shared<dyn OptionFlag>>,
    /// java: private final List<OptionListener<List<T>>> listeners;
    pub listeners: Vec<compat::Shared<dyn OptionListener<Vec<T>>>>,
    /// java: private final Class<List<T>> type;
    pub r#type: Option<compat::ClassOf<Vec<T>>>,
    /// java: private final Class<T> entryType;
    pub entry_type: Option<compat::ClassOf<T>>,
    /// java: private final List<Option<T>> entries;
    pub entries: Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl<T: Clone + 'static> ListOptionImpl<T> {
    /// java: @SuppressWarnings("unchecked") public ListOptionImpl(@Nullable Identifier id, Component name, List<Component> description, List<Component> tags, OptionBinding<List<T>> binding, Controller<T> controlle
    pub fn new(mut id: Option<Identifier>, mut name: MutableComponent, mut description: Vec<MutableComponent>, mut tags: Vec<MutableComponent>, mut binding: compat::Shared<dyn OptionBinding<Vec<T>>>, mut controller: compat::Shared<dyn Controller<T>>, initial_value: Arc<dyn Fn() -> T>, mut collapsed: bool, mut modifiable: bool, mut flags: Vec<compat::Shared<dyn OptionFlag>>, mut listeners: Vec<compat::Shared<dyn OptionListener<Vec<T>>>>) -> compat::Shared<Self> {
        let __init_id = id.clone();
        let __init_name = name.clone();
        let __init_description = description.clone();
        let __init_tags = tags.clone();
        let __init_binding = binding.clone();
        let __init_controller = controller.clone();
        let __init_initial_value = initial_value.clone();
        let __init_collapsed = collapsed;
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
            initial_value: __init_initial_value,
            collapsed: __init_collapsed,
            modifiable: __init_modifiable,
            flags: __init_flags,
            listeners: __init_listeners,
            r#type: None,
            entry_type: None,
            entries: Default::default(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        { let __v = Some(ReflectionUtils::get_actual_class(&__this.lock().borrow().binding().lock().borrow().default_value())); compat::write(&__this, |__t| __t.r#type = __v); }
        { let __v = Some(ReflectionUtils::get_actual_class(&((compat::read(&__this, |__t| __t.initial_value.clone())))())); compat::write(&__this, |__t| __t.entry_type = __v); }
        { let __v = ListOptionImpl::create_entries(&__this, &mut __this.lock().borrow().binding().lock().borrow().get()); compat::write(&__this, |__t| __t.entries = __v); }
        __this
    }

    /// java: private List<Option<T>> createEntries(List<T> values)
    pub fn create_entries(__this: &compat::Shared<Self>, values: &mut Vec<T>) -> Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>> {
        let mut entries: Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>> = Vec::new();
        for i in 0..(values.len() as i32) {
            entries.push(ListOptionImpl::create_entry(__this, values, i));
        }
        return entries;
    }

    /// java: private Option<T> createEntry(List<T> values, int i)
    pub fn create_entry(__this: &compat::Shared<Self>, values: &mut Vec<T>, mut i: i32) -> compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>> {
        // TODO(fabric2seer): List.forEach(..)
        //   java: return new OptionImpl<>(
        //   java: 				id,
        //   java: 				name(),
        //   java: 				description(),
        //   java: 				tags(),
        //   java: 				new OptionBindingImpl<>(initialValue().get(), () -> values.get(i), newValue -> values.set(i, newValue)),
        //   java: 				entryController(),
        //   java: 				modifiable(),
        //   java: 				flags(),
        //   java: 				List.of((_, updateType) -> listeners().forEach(l -> l.onUpdate(this, updateType)))
        //   java: 		);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl<T: Clone + 'static> ListOption<T> for ListOptionImpl<T> {
    /// java: @Override public Controller<T> entryController()
    fn entry_controller(&self) -> compat::Shared<dyn Controller<T>> {
        return self.controller.clone();
    }

    /// java: @Override public Supplier<T> initialValue()
    fn initial_value(&self) -> Arc<dyn Fn() -> T> {
        return self.initial_value.clone();
    }

    /// java: @Override public Class<T> entryType()
    fn entry_type(&self) -> compat::ClassOf<T> {
        return self.entry_type.clone().unwrap();
    }

}

impl<T: Clone + 'static> crate::net::azureaaron::dandelion::api::option::Option<compat::Object> for ListOptionImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public OptionBinding<List<T>> binding()
    fn binding(&self) -> compat::Shared<dyn OptionBinding<Vec<T>>> {
        return self.binding.clone();
    }

    /// java: @Override public Controller<List<T>> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<Vec<T>>> {
        panic!("UnsupportedOperationException: {}", "list options do not have a controller, see entryController() instead");
    }

    /// java: @Override public boolean modifiable()
    fn modifiable(&self) -> bool {
        return self.modifiable;
    }

    /// java: @Override public List<OptionFlag> flags()
    fn flags(&self) -> Vec<compat::Shared<dyn OptionFlag>> {
        return self.flags.clone();
    }

    /// java: @Override public List<OptionListener<List<T>>> listeners()
    fn listeners(&self) -> Vec<compat::Shared<dyn OptionListener<Vec<T>>>> {
        return self.listeners.clone();
    }

    /// java: @Override public Class<List<T>> type()
    fn r#type(&self) -> compat::ClassOf<Vec<T>> {
        return self.r#type.clone().unwrap();
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

}

impl<T: Clone + 'static> OptionGroup for ListOptionImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable Identifier id() (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Option<Identifier> {
        ListOptionImpl::id_dyn
    }

    /// java: @Override public Component name() (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> MutableComponent {
        ListOptionImpl::name_dyn
    }

    /// java: @Override public List<Component> description() (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent> {
        ListOptionImpl::description_dyn
    }

    /// java: @Override public List<Component> tags() (dispatch: the body for this class)
    fn tags(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent> {
        ListOptionImpl::tags_dyn
    }

    /// java: @Override public boolean collapsed() (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> bool {
        ListOptionImpl::collapsed_dyn
    }

    /// java: @Override public List<Option<T>> options() (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>> {
        ListOptionImpl::options_dyn
    }

}

impl<T: Clone + 'static> ListOptionImpl<T> {
    /// java: @Override public @Nullable Identifier id()
    pub fn id_impl(__this: &compat::Shared<Self>) -> Option<Identifier> {
        return compat::read(__this, |__t| __t.id.clone());
    }

    /// ListOptionImpl.id as the dispatch of OptionGroup.id
    pub fn id_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Option<Identifier> {
        Self::id_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Component name()
    pub fn name_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.name.clone());
    }

    /// ListOptionImpl.name as the dispatch of OptionGroup.name
    pub fn name_dyn(__this: &compat::Shared<dyn OptionGroup>) -> MutableComponent {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<Component> description()
    pub fn description_impl(__this: &compat::Shared<Self>) -> Vec<MutableComponent> {
        return compat::read(__this, |__t| __t.description.clone());
    }

    /// ListOptionImpl.description as the dispatch of OptionGroup.description
    pub fn description_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<Component> tags()
    pub fn tags_impl(__this: &compat::Shared<Self>) -> Vec<MutableComponent> {
        return compat::read(__this, |__t| __t.tags.clone());
    }

    /// ListOptionImpl.tags as the dispatch of OptionGroup.tags
    pub fn tags_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent> {
        Self::tags_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean collapsed()
    pub fn collapsed_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.collapsed);
    }

    /// ListOptionImpl.collapsed as the dispatch of OptionGroup.collapsed
    pub fn collapsed_dyn(__this: &compat::Shared<dyn OptionGroup>) -> bool {
        Self::collapsed_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<Option<T>> options()
    pub fn options_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>> {
        return compat::read(__this, |__t| __t.entries.clone());
    }

    /// ListOptionImpl.options as the dispatch of OptionGroup.options
    pub fn options_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>> {
        Self::options_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class BuilderImpl<T> implements ListOption.Builder<T>
#[derive(Clone)]
pub struct BuilderImpl<T> {
    /// java: private @Nullable Identifier id = null;
    pub id: Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = java.util.List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private @Nullable OptionBinding<List<T>> binding = null;
    pub binding: Option<compat::Shared<dyn OptionBinding<Vec<T>>>>,
    /// java: private @Nullable Controller<T> controller = null;
    pub controller: Option<compat::Shared<dyn Controller<T>>>,
    /// java: private @Nullable Supplier<T> initialValue = null;
    pub initial_value: Option<Arc<dyn Fn() -> T>>,
    /// java: private boolean collapsed = false;
    pub collapsed: bool,
    /// java: private boolean modifiable = true;
    pub modifiable: bool,
    /// java: private List<OptionFlag> flags = List.of();
    pub flags: Vec<compat::Shared<dyn OptionFlag>>,
    /// java: private final List<OptionListener<List<T>>> listeners = new ArrayList<>();
    pub listeners: Vec<compat::Shared<dyn OptionListener<Vec<T>>>>,
}

impl<T: Clone + 'static> BuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            id: None,
            name: component::empty(),
            description: vec![],
            tags: vec![],
            binding: None,
            controller: None,
            initial_value: None,
            collapsed: false,
            modifiable: true,
            flags: vec![],
            listeners: Vec::new(),
        };
        this
    }

}

impl<T: Clone + 'static> Builder<T> for BuilderImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ListOption.Builder<T> id(Identifier id)
    fn id(&mut self, mut id: Identifier) -> compat::Shared<dyn Builder<T>> {
        self.id = Some(id.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> name(Component name)
    fn name(&mut self, mut name: MutableComponent) -> compat::Shared<dyn Builder<T>> {
        self.name = name.clone();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> description(Component... texts)
    fn description(&mut self, texts: &Vec<MutableComponent>) -> compat::Shared<dyn Builder<T>> {
        self.description = texts.to_vec();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> tags(Component... tags)
    fn tags(&mut self, tags: &Vec<MutableComponent>) -> compat::Shared<dyn Builder<T>> {
        self.tags = tags.to_vec();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> binding(List<T> defaultValue, Supplier<List<T>> getter, Consumer<List<T>> setter)
    fn binding(&mut self, default_value: &Vec<T>, getter: Arc<dyn Fn() -> Vec<T>>, setter: Arc<dyn Fn(Vec<T>)>) -> compat::Shared<dyn Builder<T>> {
        self.binding = Some((compat::shared(OptionBindingImpl::new(default_value.clone(), getter.clone(), setter.clone())) as compat::Shared<dyn OptionBinding<compat::Object>>));
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> controller(Controller<T> controller)
    fn controller(&mut self, mut controller: compat::Shared<dyn Controller<T>>) -> compat::Shared<dyn Builder<T>> {
        self.controller = Some(controller.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> initial(Supplier<T> initialValue)
    fn initial(&mut self, initial_value: Arc<dyn Fn() -> T>) -> compat::Shared<dyn Builder<T>> {
        self.initial_value = Some(initial_value.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> initial(T initialValue)
    fn initial_t(&mut self, initial_value: &T) -> compat::Shared<dyn Builder<T>> {
        self.initial_value = Some(Arc::new(move || initial_value.clone()));
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> collapsed(boolean collapsed)
    fn collapsed(&mut self, mut collapsed: bool) -> compat::Shared<dyn Builder<T>> {
        self.collapsed = collapsed;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> modifiable(boolean modifiable)
    fn modifiable(&mut self, mut modifiable: bool) -> compat::Shared<dyn Builder<T>> {
        self.modifiable = modifiable;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> flags(OptionFlag... flags)
    fn flags(&mut self, flags: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>> {
        self.flags = flags.to_vec();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption.Builder<T> listener(OptionListener<List<T>> listener)
    fn listener(&mut self, mut listener: compat::Shared<dyn OptionListener<Vec<T>>>) -> compat::Shared<dyn Builder<T>> {
        self.listeners.push(listener.clone());
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public ListOption<T> build()
    fn build(&self) -> compat::Shared<dyn ListOption<T>> {
        self.binding;
        self.controller;
        self.initial_value;
        return (ListOptionImpl::new(self.id.clone(), self.name.clone(), self.description.clone(), self.tags.clone(), self.binding.clone().unwrap(), self.controller.clone().unwrap(), self.initial_value.clone().unwrap(), self.collapsed, self.modifiable, self.flags.clone(), self.listeners.iter().cloned().collect::<Vec<_>>()) as compat::Shared<dyn ListOption<compat::Object>>);
    }

}
