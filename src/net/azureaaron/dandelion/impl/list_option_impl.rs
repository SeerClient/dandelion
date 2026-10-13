//! Ported from net.azureaaron.dandelion.impl.ListOptionImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::list_option::{self, ListOption};
use crate::net::azureaaron::dandelion::api::option::{AnyOption, Option};
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::api::option_listener::{OptionListener, UpdateType};
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_editable_option_adapter::{EditableOptionFactory, MoulConfigEditableOptionAdapter};
use crate::net::azureaaron::dandelion::r#impl::option_binding_impl::OptionBindingImpl;
use crate::net::azureaaron::dandelion::r#impl::option_impl::OptionImpl;
use crate::net::azureaaron::dandelion::r#impl::utils::reflection_utils::ReflectionUtils;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_adapter::YACLOptionAdapter;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_group_adapter::YACLOptionGroupAdapter;
use ::yacl::dev::isxander::yacl3::api::option::AnyOption as YAnyOption;
use ::yacl::dev::isxander::yacl3::api::option_group::OptionGroup as YOptionGroup;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use std::sync::Weak;

/// java: public class ListOptionImpl<T> implements ListOption<T>
pub struct ListOptionImpl<T: Clone + PartialEq + 'static> {
    /// java: private final @Nullable Identifier id;
    pub id: std::option::Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
    /// java: private final OptionBinding<List<T>> binding;
    pub binding: Arc<dyn OptionBinding<Vec<T>>>,
    /// java: private final Controller<T> controller;
    pub controller: Arc<dyn Controller<T>>,
    /// java: private final Supplier<T> initialValue;
    pub initial_value: Arc<dyn Fn() -> T>,
    /// java: private final boolean collapsed;
    pub collapsed: bool,
    /// java: private final boolean modifiable;
    pub modifiable: bool,
    /// java: private final List<OptionFlag> flags;
    pub flags: Vec<Arc<dyn OptionFlag>>,
    /// java: private final List<OptionListener<List<T>>> listeners;
    pub listeners: Vec<Arc<dyn OptionListener<Vec<T>>>>,
    /// java: private final Class<List<T>> type;
    pub r#type: compat::ClassOf<Vec<T>>,
    /// java: private final Class<T> entryType;
    pub entry_type: compat::ClassOf<T>,
    /// java: private final List<Option<T>> entries;
    pub entries: Vec<Arc<dyn Option<T>>>,
}

impl<T: Clone + PartialEq + 'static> ListOptionImpl<T> {
    /// java: @SuppressWarnings("unchecked") public ListOptionImpl(@Nullable Identifier id, Component name,
    /// List<Component> description, List<Component> tags, OptionBinding<List<T>> binding, Controller<T> controller,
    /// Supplier<T> initialValue, boolean collapsed, boolean modifiable, List<OptionFlag> flags,
    /// List<OptionListener<List<T>>> listeners)
    ///
    /// The entries' listener hands `this` to the list's listeners, so the object is built with `Arc::new_cyclic`
    /// (PATTERNS_Y2 P5).
    pub fn new(
        id: std::option::Option<Identifier>,
        name: MutableComponent,
        description: Vec<MutableComponent>,
        tags: Vec<MutableComponent>,
        binding: Arc<dyn OptionBinding<Vec<T>>>,
        controller: Arc<dyn Controller<T>>,
        initial_value: Arc<dyn Fn() -> T>,
        collapsed: bool,
        modifiable: bool,
        flags: Vec<Arc<dyn OptionFlag>>,
        listeners: Vec<Arc<dyn OptionListener<Vec<T>>>>,
    ) -> Arc<ListOptionImpl<T>> {
        Arc::new_cyclic(|this: &Weak<ListOptionImpl<T>>| {
            // java: ReflectionUtils.getActualClass(Objects.requireNonNull(this.binding().defaultValue(), ..))
            let r#type = ReflectionUtils::get_actual_class(&binding.default_value());
            // java: ReflectionUtils.getActualClass(Objects.requireNonNull(this.initialValue.get(), ..))
            let entry_type = ReflectionUtils::get_actual_class(&initial_value());
            let mut list = ListOptionImpl {
                id,
                name,
                description,
                tags,
                binding,
                controller,
                initial_value,
                collapsed,
                modifiable,
                flags,
                listeners,
                r#type,
                entry_type,
                entries: Vec::new(),
            };
            // java: this.entries = createEntries(this.binding().get());
            list.entries = list.create_entries(this, list.binding.get());
            list
        })
    }

    /// java: private List<Option<T>> createEntries(List<T> values)
    fn create_entries(&self, this: &Weak<ListOptionImpl<T>>, values: Vec<T>) -> Vec<Arc<dyn Option<T>>> {
        let mut entries: Vec<Arc<dyn Option<T>>> = Vec::new();
        for i in 0..values.len() {
            entries.push(self.create_entry(this, &values, i));
        }
        entries
    }

    /// java: private Option<T> createEntry(List<T> values, int i)
    ///
    /// Java's getter / setter read and write element `i` of the list the binding's getter returned - the config's own
    /// list object. A Rust getter answers the list by value, so the entry reads the binding's list again and writes
    /// element `i` back through the binding's setter.
    fn create_entry(&self, this: &Weak<ListOptionImpl<T>>, values: &Vec<T>, i: usize) -> Arc<dyn Option<T>> {
        let _ = values;
        let get_binding = self.binding.clone();
        let set_binding = self.binding.clone();
        let entry_binding: Arc<dyn OptionBinding<T>> = Arc::new(OptionBindingImpl::new(
            (self.initial_value)(),
            Arc::new(move || get_binding.get()[i].clone()),
            Arc::new(move |new_value: T| {
                let mut values = set_binding.get();
                values[i] = new_value;
                set_binding.set(values);
            }),
        ));
        // java: List.of((_, updateType) -> listeners().forEach(l -> l.onUpdate(this, updateType)))
        let list = this.clone();
        let entry_listener: Arc<dyn OptionListener<T>> = Arc::new(move |_option: &dyn Option<T>, update_type: UpdateType| {
            if let Some(list) = list.upgrade() {
                for l in Option::listeners(&*list) {
                    l.on_update(&*list, update_type);
                }
            }
        });
        Arc::new(OptionImpl::new(
            self.id.clone(),
            self.name.clone(),
            self.description.clone(),
            self.tags.clone(),
            entry_binding,
            self.controller.clone(),
            self.modifiable,
            self.flags.clone(),
            vec![entry_listener],
        ))
    }
}

impl<T: Clone + PartialEq + 'static> AnyOption for ListOptionImpl<T> {
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
        YACLOptionAdapter::to_yacl_option::<Vec<T>>(manager, self)
    }

    fn create_editable_option_factory(self: Arc<Self>) -> EditableOptionFactory {
        MoulConfigEditableOptionAdapter::create_editable_option_factory::<Vec<T>>(self)
    }
}

impl<T: Clone + PartialEq + 'static> Option<Vec<T>> for ListOptionImpl<T> {
    /// java: @Override public OptionBinding<List<T>> binding()
    fn binding(&self) -> Arc<dyn OptionBinding<Vec<T>>> {
        self.binding.clone()
    }

    /// java: @Override public Controller<List<T>> controller()
    fn controller(&self) -> Arc<dyn Controller<Vec<T>>> {
        panic!("UnsupportedOperationException: list options do not have a controller, see entryController() instead")
    }

    /// java: @Override public List<OptionListener<List<T>>> listeners()
    fn listeners(&self) -> Vec<Arc<dyn OptionListener<Vec<T>>>> {
        self.listeners.clone()
    }

    /// java: @Override public Class<List<T>> type()
    fn r#type(&self) -> compat::ClassOf<Vec<T>> {
        self.r#type
    }
}

impl<T: Clone + PartialEq + 'static> OptionGroup for ListOptionImpl<T> {
    fn id(&self) -> std::option::Option<Identifier> {
        self.id.clone()
    }

    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    fn description(&self) -> Vec<MutableComponent> {
        self.description.clone()
    }

    fn tags(&self) -> Vec<MutableComponent> {
        self.tags.clone()
    }

    /// java: @Override public boolean collapsed()
    fn collapsed(&self) -> bool {
        self.collapsed
    }

    /// java: @Override public List<Option<T>> options()
    fn options(&self) -> Vec<Arc<dyn AnyOption>> {
        self.entries.iter().map(|entry| entry.clone() as Arc<dyn AnyOption>).collect()
    }

    fn to_yacl_option_group(self: Arc<Self>, manager: &Arc<dyn AnyConfigManager>) -> Arc<dyn YOptionGroup> {
        YACLOptionGroupAdapter::to_yacl_list_option_group::<T>(manager, self)
    }
}

impl<T: Clone + PartialEq + 'static> ListOption<T> for ListOptionImpl<T> {
    /// java: @Override public Supplier<T> initialValue()
    fn initial_value(&self) -> Arc<dyn Fn() -> T> {
        self.initial_value.clone()
    }

    /// java: @Override public Controller<T> entryController()
    fn entry_controller(&self) -> Arc<dyn Controller<T>> {
        self.controller.clone()
    }

    /// java: @Override public Class<T> entryType()
    fn entry_type(&self) -> compat::ClassOf<T> {
        self.entry_type
    }
}

/// java: public static class BuilderImpl<T> implements ListOption.Builder<T>
pub struct BuilderImpl<T: Clone + PartialEq + 'static> {
    /// java: private @Nullable Identifier id = null;
    pub id: std::option::Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = java.util.List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private @Nullable OptionBinding<List<T>> binding = null;
    pub binding: std::option::Option<Arc<dyn OptionBinding<Vec<T>>>>,
    /// java: private @Nullable Controller<T> controller = null;
    pub controller: std::option::Option<Arc<dyn Controller<T>>>,
    /// java: private @Nullable Supplier<T> initialValue = null;
    pub initial_value: std::option::Option<Arc<dyn Fn() -> T>>,
    /// java: private boolean collapsed = false;
    pub collapsed: bool,
    /// java: private boolean modifiable = true;
    pub modifiable: bool,
    /// java: private List<OptionFlag> flags = List.of();
    pub flags: Vec<Arc<dyn OptionFlag>>,
    /// java: private final List<OptionListener<List<T>>> listeners = new ArrayList<>();
    pub listeners: Vec<Arc<dyn OptionListener<Vec<T>>>>,
}

impl<T: Clone + PartialEq + 'static> BuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self {
            id: None,
            name: component::empty(),
            description: Vec::new(),
            tags: Vec::new(),
            binding: None,
            controller: None,
            initial_value: None,
            collapsed: false,
            modifiable: true,
            flags: Vec::new(),
            listeners: Vec::new(),
        }
    }
}

impl<T: Clone + PartialEq + 'static> list_option::Builder<T> for BuilderImpl<T> {
    /// java: @Override public ListOption.Builder<T> id(Identifier id)
    fn id(mut self, id: Identifier) -> Self {
        self.id = Some(id);
        self
    }

    /// java: @Override public ListOption.Builder<T> name(Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = name;
        self
    }

    /// java: @Override public ListOption.Builder<T> description(Component... texts)
    fn description(mut self, texts: Vec<MutableComponent>) -> Self {
        self.description = texts;
        self
    }

    /// java: @Override public ListOption.Builder<T> tags(Component... tags)
    fn tags(mut self, tags: Vec<MutableComponent>) -> Self {
        self.tags = tags;
        self
    }

    /// java: @Override public ListOption.Builder<T> binding(List<T> defaultValue, Supplier<List<T>> getter,
    /// Consumer<List<T>> setter)
    fn binding(mut self, default_value: Vec<T>, getter: impl Fn() -> Vec<T> + 'static, setter: impl Fn(Vec<T>) + 'static) -> Self {
        self.binding = Some(Arc::new(OptionBindingImpl::new(default_value, Arc::new(getter), Arc::new(setter))));
        self
    }

    /// java: @Override public ListOption.Builder<T> controller(Controller<T> controller)
    fn controller(mut self, controller: Arc<dyn Controller<T>>) -> Self {
        self.controller = Some(controller);
        self
    }

    /// java: @Override public ListOption.Builder<T> initial(Supplier<T> initialValue)
    fn initial(mut self, initial_value: impl Fn() -> T + 'static) -> Self {
        self.initial_value = Some(Arc::new(initial_value));
        self
    }

    /// java: @Override public ListOption.Builder<T> initial(T initialValue)
    fn initial_t(mut self, initial_value: T) -> Self {
        self.initial_value = Some(Arc::new(move || initial_value.clone()));
        self
    }

    /// java: @Override public ListOption.Builder<T> collapsed(boolean collapsed)
    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// java: @Override public ListOption.Builder<T> modifiable(boolean modifiable)
    fn modifiable(mut self, modifiable: bool) -> Self {
        self.modifiable = modifiable;
        self
    }

    /// java: @Override public ListOption.Builder<T> flags(OptionFlag... flags)
    fn flags(mut self, flags: Vec<Arc<dyn OptionFlag>>) -> Self {
        self.flags = flags;
        self
    }

    /// java: @Override public ListOption.Builder<T> listener(OptionListener<List<T>> listener)
    fn listener(mut self, listener: Arc<dyn OptionListener<Vec<T>>>) -> Self {
        self.listeners.push(listener);
        self
    }

    /// java: @Override public ListOption<T> build()
    fn build(self) -> Arc<dyn ListOption<T>> {
        let binding = self.binding.expect("NullPointerException: a binding is required.");
        let controller = self.controller.expect("NullPointerException: a controller is required.");
        let initial_value = self.initial_value.expect("NullPointerException: an initial value is required.");
        ListOptionImpl::new(
            self.id,
            self.name,
            self.description,
            self.tags,
            binding,
            controller,
            initial_value,
            self.collapsed,
            self.modifiable,
            self.flags,
            self.listeners,
        )
    }
}
