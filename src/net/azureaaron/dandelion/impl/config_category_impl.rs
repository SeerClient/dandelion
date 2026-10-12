//! Ported from net.azureaaron.dandelion.impl.ConfigCategoryImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::Builder;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::option_adder::OptionAdder;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::r#impl::option_group_impl::OptionGroupImpl;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class ConfigCategoryImpl implements ConfigCategory
#[derive(Clone)]
pub struct ConfigCategoryImpl {
    /// java: private final Identifier id;
    pub id: Identifier,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final Component description;
    pub description: MutableComponent,
    /// java: private final OptionGroup rootGroup;
    pub root_group: compat::Shared<dyn OptionGroup>,
    /// java: private final List<OptionGroup> groups;
    pub groups: Vec<compat::Shared<dyn OptionGroup>>,
}

impl ConfigCategoryImpl {
    /// java: protected ConfigCategoryImpl(Identifier id, Component name, Component description, OptionGroup rootGroup, List<OptionGroup> groups)
    pub fn new(mut id: Identifier, mut name: MutableComponent, mut description: MutableComponent, mut root_group: compat::Shared<dyn OptionGroup>, mut groups: Vec<compat::Shared<dyn OptionGroup>>) -> compat::Shared<Self> {
        let __init_id = id.clone();
        let __init_name = name.clone();
        let __init_description = description.clone();
        let __init_root_group = root_group.clone();
        let __init_groups = groups.clone();
        let __this = compat::shared(Self {
            id: __init_id,
            name: __init_name,
            description: __init_description,
            root_group: __init_root_group,
            groups: __init_groups,
        });
        __this
    }

}

impl ConfigCategory for ConfigCategoryImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Identifier id() (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> Identifier {
        ConfigCategoryImpl::id_dyn
    }

    /// java: @Override public Component name() (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        ConfigCategoryImpl::name_dyn
    }

    /// java: @Override public Component description() (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        ConfigCategoryImpl::description_dyn
    }

    /// java: @Override public @Nullable OptionGroup rootGroup() (dispatch: the body for this class)
    fn root_group(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> Option<compat::Shared<dyn OptionGroup>> {
        ConfigCategoryImpl::root_group_dyn
    }

    /// java: @Override public List<OptionGroup> groups() (dispatch: the body for this class)
    fn groups(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> Vec<compat::Shared<dyn OptionGroup>> {
        ConfigCategoryImpl::groups_dyn
    }

}

impl ConfigCategoryImpl {
    /// java: @Override public Identifier id()
    pub fn id_impl(__this: &compat::Shared<Self>) -> Identifier {
        return compat::read(__this, |__t| __t.id.clone());
    }

    /// ConfigCategoryImpl.id as the dispatch of ConfigCategory.id
    pub fn id_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> Identifier {
        Self::id_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Component name()
    pub fn name_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.name.clone());
    }

    /// ConfigCategoryImpl.name as the dispatch of ConfigCategory.name
    pub fn name_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Component description()
    pub fn description_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.description.clone());
    }

    /// ConfigCategoryImpl.description as the dispatch of ConfigCategory.description
    pub fn description_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> MutableComponent {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public @Nullable OptionGroup rootGroup()
    pub fn root_group_impl(__this: &compat::Shared<Self>) -> Option<compat::Shared<dyn OptionGroup>> {
        return Some(compat::read(__this, |__t| __t.root_group.clone()));
    }

    /// ConfigCategoryImpl.rootGroup as the dispatch of ConfigCategory.rootGroup
    pub fn root_group_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> Option<compat::Shared<dyn OptionGroup>> {
        Self::root_group_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<OptionGroup> groups()
    pub fn groups_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<dyn OptionGroup>> {
        return compat::read(__this, |__t| __t.groups.clone());
    }

    /// ConfigCategoryImpl.groups as the dispatch of ConfigCategory.groups
    pub fn groups_dyn(__this: &compat::Shared<dyn ConfigCategory>) -> Vec<compat::Shared<dyn OptionGroup>> {
        Self::groups_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class ConfigCategoryBuilderImpl implements ConfigCategory.Builder
#[derive(Clone)]
pub struct ConfigCategoryBuilderImpl {
    /// java: private @Nullable Identifier id = null;
    pub id: Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private Component description = Component.empty();
    pub description: MutableComponent,
    /// java: private List<Option<?>> rootOptions = new ArrayList<>();
    pub root_options: Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>,
    /// java: private List<OptionGroup> groups = new ArrayList<>();
    pub groups: Vec<compat::Shared<dyn OptionGroup>>,
}

impl ConfigCategoryBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            id: None,
            name: component::empty(),
            description: component::empty(),
            root_options: Vec::new(),
            groups: Vec::new(),
        });
        __this
    }

}

impl Builder for ConfigCategoryBuilderImpl {
    /// java: @Override public Builder id(Identifier id) (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn Builder>, Identifier) -> compat::Shared<dyn Builder> {
        ConfigCategoryBuilderImpl::id_dyn
    }

    /// java: @Override public Builder name(Component name) (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        ConfigCategoryBuilderImpl::name_dyn
    }

    /// java: @Override public Builder description(Component description) (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        ConfigCategoryBuilderImpl::description_dyn
    }

    /// java: @Override public Builder group(OptionGroup group) (dispatch: the body for this class)
    fn group(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        ConfigCategoryBuilderImpl::group_dyn
    }

    /// java: @Override public ConfigCategory build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn ConfigCategory> {
        ConfigCategoryBuilderImpl::build_dyn
    }

}

impl OptionAdder for ConfigCategoryBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder option(Option<?> option) (dispatch: the body for this class)
    fn option(&self) -> fn(&compat::Shared<dyn OptionAdder>, &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAdder> {
        ConfigCategoryBuilderImpl::option_dyn
    }

    /// java: @Override default Builder optionIf(boolean condition, Option<?> option) (dispatch: the body for this class)
    fn option_if(&self) -> fn(&compat::Shared<dyn OptionAdder>, bool, &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAdder> {
        <dyn Builder>::option_if_from::<Self>
    }

    /// java: @Override default Builder optionIf(boolean condition, Supplier<Option<?>> option) (dispatch: the body for this class)
    fn option_if_boolean_supplier(&self) -> fn(&compat::Shared<dyn OptionAdder>, bool, Arc<dyn Fn() -> compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAdder> {
        <dyn Builder>::option_if_boolean_supplier_from::<Self>
    }

    /// java: @Override default Builder options(List<? extends Option<?>> options) (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionAdder>, &Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAdder> {
        <dyn Builder>::options_from::<Self>
    }

}

impl ConfigCategoryBuilderImpl {
    /// java: @Override public Builder id(Identifier id)
    pub fn id_impl(__this: &compat::Shared<Self>, mut id: Identifier) -> compat::Shared<dyn Builder> {
        { let __v = Some(id.clone()); compat::write(__this, |__t| __t.id = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ConfigCategoryBuilderImpl.id as the dispatch of Builder.id
    pub fn id_dyn(__this: &compat::Shared<dyn Builder>, __p0: Identifier) -> compat::Shared<dyn Builder> {
        Self::id_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder name(Component name)
    pub fn name_impl(__this: &compat::Shared<Self>, mut name: MutableComponent) -> compat::Shared<dyn Builder> {
        { let __v = name.clone(); compat::write(__this, |__t| __t.name = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ConfigCategoryBuilderImpl.name as the dispatch of Builder.name
    pub fn name_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder description(Component description)
    pub fn description_impl(__this: &compat::Shared<Self>, mut description: MutableComponent) -> compat::Shared<dyn Builder> {
        { let __v = description.clone(); compat::write(__this, |__t| __t.description = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ConfigCategoryBuilderImpl.description as the dispatch of Builder.description
    pub fn description_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder group(OptionGroup group)
    pub fn group_impl(__this: &compat::Shared<Self>, mut group: compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        let __h0 = group.clone();
        __this.lock().borrow_mut().groups.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ConfigCategoryBuilderImpl.group as the dispatch of Builder.group
    pub fn group_dyn(__this: &compat::Shared<dyn Builder>, __p0: compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        Self::group_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public ConfigCategory build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn ConfigCategory> {
        compat::read(__this, |__t| __t.id.clone());
        return (ConfigCategoryImpl::new(compat::read(__this, |__t| __t.id.clone()).unwrap(), compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.description.clone()), (OptionGroupImpl::new(None, component::empty(), vec![], vec![], false, __this.lock().borrow().root_options.iter().cloned().collect::<Vec<_>>()) as compat::Shared<dyn OptionGroup>), __this.lock().borrow().groups.iter().cloned().collect::<Vec<_>>()) as compat::Shared<dyn ConfigCategory>);
    }

    /// ConfigCategoryBuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn ConfigCategory> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Builder option(Option<?> option)
    pub fn option_impl(__this: &compat::Shared<Self>, option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn Builder> {
        let __h0 = option.clone();
        __this.lock().borrow_mut().root_options.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ConfigCategoryBuilderImpl.option as the dispatch of OptionAdder.option
    pub fn option_dyn(__this: &compat::Shared<dyn OptionAdder>, __p0: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAdder> {
        Self::option_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
