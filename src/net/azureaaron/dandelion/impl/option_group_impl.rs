//! Ported from net.azureaaron.dandelion.impl.OptionGroupImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option_adder::OptionAdder;
use crate::net::azureaaron::dandelion::api::option_group::Builder;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class OptionGroupImpl implements OptionGroup
#[derive(Clone)]
pub struct OptionGroupImpl {
    /// java: private final @Nullable Identifier id;
    pub id: Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
    /// java: private final boolean collapsed;
    pub collapsed: bool,
    /// java: private final List<Option<?>> options;
    pub options: Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>,
}

impl OptionGroupImpl {
    /// java: protected OptionGroupImpl(@Nullable Identifier id, Component name, List<Component> description, List<Component> tags, boolean collapsed, List<Option<?>> options)
    pub fn new(mut id: Option<Identifier>, mut name: MutableComponent, mut description: Vec<MutableComponent>, mut tags: Vec<MutableComponent>, mut collapsed: bool, mut options: Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<Self> {
        let __init_id = id.clone();
        let __init_name = name.clone();
        let __init_description = description.clone();
        let __init_tags = tags.clone();
        let __init_collapsed = collapsed;
        let __init_options = options.clone();
        let __this = compat::shared(Self {
            id: __init_id,
            name: __init_name,
            description: __init_description,
            tags: __init_tags,
            collapsed: __init_collapsed,
            options: __init_options,
        });
        __this
    }

}

impl OptionGroup for OptionGroupImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable Identifier id() (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Option<Identifier> {
        OptionGroupImpl::id_dyn
    }

    /// java: @Override public Component name() (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> MutableComponent {
        OptionGroupImpl::name_dyn
    }

    /// java: @Override public List<Component> description() (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent> {
        OptionGroupImpl::description_dyn
    }

    /// java: @Override public List<Component> tags() (dispatch: the body for this class)
    fn tags(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent> {
        OptionGroupImpl::tags_dyn
    }

    /// java: @Override public boolean collapsed() (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> bool {
        OptionGroupImpl::collapsed_dyn
    }

    /// java: @Override public List<Option<?>> options() (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>> {
        OptionGroupImpl::options_dyn
    }

}

impl OptionGroupImpl {
    /// java: @Override public @Nullable Identifier id()
    pub fn id_impl(__this: &compat::Shared<Self>) -> Option<Identifier> {
        return compat::read(__this, |__t| __t.id.clone());
    }

    /// OptionGroupImpl.id as the dispatch of OptionGroup.id
    pub fn id_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Option<Identifier> {
        Self::id_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Component name()
    pub fn name_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.name.clone());
    }

    /// OptionGroupImpl.name as the dispatch of OptionGroup.name
    pub fn name_dyn(__this: &compat::Shared<dyn OptionGroup>) -> MutableComponent {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<Component> description()
    pub fn description_impl(__this: &compat::Shared<Self>) -> Vec<MutableComponent> {
        return compat::read(__this, |__t| __t.description.clone());
    }

    /// OptionGroupImpl.description as the dispatch of OptionGroup.description
    pub fn description_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<Component> tags()
    pub fn tags_impl(__this: &compat::Shared<Self>) -> Vec<MutableComponent> {
        return compat::read(__this, |__t| __t.tags.clone());
    }

    /// OptionGroupImpl.tags as the dispatch of OptionGroup.tags
    pub fn tags_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent> {
        Self::tags_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean collapsed()
    pub fn collapsed_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.collapsed);
    }

    /// OptionGroupImpl.collapsed as the dispatch of OptionGroup.collapsed
    pub fn collapsed_dyn(__this: &compat::Shared<dyn OptionGroup>) -> bool {
        Self::collapsed_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public List<Option<?>> options()
    pub fn options_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>> {
        return compat::read(__this, |__t| __t.options.clone());
    }

    /// OptionGroupImpl.options as the dispatch of OptionGroup.options
    pub fn options_dyn(__this: &compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>> {
        Self::options_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class OptionGroupBuilderImpl implements OptionGroup.Builder
#[derive(Clone)]
pub struct OptionGroupBuilderImpl {
    /// java: private @Nullable Identifier id = null;
    pub id: Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private boolean collapsed = false;
    pub collapsed: bool,
    /// java: private List<Option<?>> options = new ArrayList<>();
    pub options: Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>,
}

impl OptionGroupBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            id: None,
            name: component::empty(),
            description: vec![],
            tags: vec![],
            collapsed: false,
            options: Vec::new(),
        });
        __this
    }

}

impl Builder for OptionGroupBuilderImpl {
    /// java: @Override public Builder id(Identifier id) (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn Builder>, Identifier) -> compat::Shared<dyn Builder> {
        OptionGroupBuilderImpl::id_dyn
    }

    /// java: @Override public Builder name(Component name) (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        OptionGroupBuilderImpl::name_dyn
    }

    /// java: @Override public Builder description(Component... texts) (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        OptionGroupBuilderImpl::description_dyn
    }

    /// java: @Override public Builder tags(Component... tags) (dispatch: the body for this class)
    fn tags(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        OptionGroupBuilderImpl::tags_dyn
    }

    /// java: @Override public Builder collapsed(boolean collapsed) (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn Builder>, bool) -> compat::Shared<dyn Builder> {
        OptionGroupBuilderImpl::collapsed_dyn
    }

    /// java: @Override public OptionGroup build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn OptionGroup> {
        OptionGroupBuilderImpl::build_dyn
    }

}

impl OptionAdder for OptionGroupBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder option(Option<?> option) (dispatch: the body for this class)
    fn option(&self) -> fn(&compat::Shared<dyn OptionAdder>, &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAdder> {
        OptionGroupBuilderImpl::option_dyn
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

impl OptionGroupBuilderImpl {
    /// java: @Override public Builder id(Identifier id)
    pub fn id_impl(__this: &compat::Shared<Self>, mut id: Identifier) -> compat::Shared<dyn Builder> {
        { let __v = Some(id.clone()); compat::write(__this, |__t| __t.id = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// OptionGroupBuilderImpl.id as the dispatch of Builder.id
    pub fn id_dyn(__this: &compat::Shared<dyn Builder>, __p0: Identifier) -> compat::Shared<dyn Builder> {
        Self::id_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder name(Component name)
    pub fn name_impl(__this: &compat::Shared<Self>, mut name: MutableComponent) -> compat::Shared<dyn Builder> {
        { let __v = name.clone(); compat::write(__this, |__t| __t.name = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// OptionGroupBuilderImpl.name as the dispatch of Builder.name
    pub fn name_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder description(Component... texts)
    pub fn description_impl(__this: &compat::Shared<Self>, texts: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        { let __v = texts.to_vec(); compat::write(__this, |__t| __t.description = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// OptionGroupBuilderImpl.description as the dispatch of Builder.description
    pub fn description_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder tags(Component... tags)
    pub fn tags_impl(__this: &compat::Shared<Self>, tags: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        { let __v = tags.to_vec(); compat::write(__this, |__t| __t.tags = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// OptionGroupBuilderImpl.tags as the dispatch of Builder.tags
    pub fn tags_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::tags_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder collapsed(boolean collapsed)
    pub fn collapsed_impl(__this: &compat::Shared<Self>, mut collapsed: bool) -> compat::Shared<dyn Builder> {
        { let __v = collapsed; compat::write(__this, |__t| __t.collapsed = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// OptionGroupBuilderImpl.collapsed as the dispatch of Builder.collapsed
    pub fn collapsed_dyn(__this: &compat::Shared<dyn Builder>, __p0: bool) -> compat::Shared<dyn Builder> {
        Self::collapsed_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public OptionGroup build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn OptionGroup> {
        return (OptionGroupImpl::new(compat::read(__this, |__t| __t.id.clone()), compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.description.clone()), compat::read(__this, |__t| __t.tags.clone()), compat::read(__this, |__t| __t.collapsed), __this.lock().borrow().options.iter().cloned().collect::<Vec<_>>()) as compat::Shared<dyn OptionGroup>);
    }

    /// OptionGroupBuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn OptionGroup> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Builder option(Option<?> option)
    pub fn option_impl(__this: &compat::Shared<Self>, option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn Builder> {
        let __h0 = option.clone();
        __this.lock().borrow_mut().options.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// OptionGroupBuilderImpl.option as the dispatch of OptionAdder.option
    pub fn option_dyn(__this: &compat::Shared<dyn OptionAdder>, __p0: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAdder> {
        Self::option_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
