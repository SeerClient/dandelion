//! Ported from net.azureaaron.dandelion.api.ConfigCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option_adder::OptionAdder;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::r#impl::config_category_impl::ConfigCategoryBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface ConfigCategory
pub trait ConfigCategory: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Identifier id(); (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> Identifier;

    /// java: Component name(); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent;

    /// java: Component description(); (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> MutableComponent;

    /// java: @Nullable OptionGroup rootGroup(); (dispatch: the body for this class)
    fn root_group(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> Option<compat::Shared<dyn OptionGroup>>;

    /// java: List<OptionGroup> groups(); (dispatch: the body for this class)
    fn groups(&self) -> fn(&compat::Shared<dyn ConfigCategory>) -> Vec<compat::Shared<dyn OptionGroup>>;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (ConfigCategoryBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: non-sealed interface Builder extends OptionAdder
pub trait Builder: OptionAdder {
    /// java: Builder id(Identifier id); (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn Builder>, Identifier) -> compat::Shared<dyn Builder>;

    /// java: Builder name(Component name); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder description(Component description); (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder group(OptionGroup group); (dispatch: the body for this class)
    fn group(&self) -> fn(&compat::Shared<dyn Builder>, compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder>;

    /// java: default Builder groupIf(boolean condition, OptionGroup group) (dispatch: the body for this class)
    fn group_if(&self) -> fn(&compat::Shared<dyn Builder>, bool, &compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::group_if_body
    }

    /// java: default Builder groups(List<OptionGroup> groups) (dispatch: the body for this class)
    fn groups(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::groups_body
    }

    /// java: ConfigCategory build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn ConfigCategory>;

}

impl dyn Builder {
    /// java: @Override default Builder optionIf(boolean condition, Option<?> option)
    pub fn option_if_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static field OptionAdder.super
        //   java: OptionAdder.super.optionIf(condition, option);
        return __this.clone();
    }

    /// Builder.optionIf as the dispatch of OptionAdder.optionIf for the concrete class `__C`
    pub fn option_if_from<__C: Builder + 'static>(__this: &compat::Shared<dyn OptionAdder>, __p0: bool, __p1: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAdder> {
        <dyn Builder>::option_if_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Builder>), __p0, __p1)
    }

    /// java: @Override default Builder optionIf(boolean condition, Supplier<Option<?>> option)
    pub fn option_if_boolean_supplier_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, option: Arc<dyn Fn() -> compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static field OptionAdder.super
        //   java: OptionAdder.super.optionIf(condition, option);
        return __this.clone();
    }

    /// Builder.optionIf as the dispatch of OptionAdder.optionIf for the concrete class `__C`
    pub fn option_if_boolean_supplier_from<__C: Builder + 'static>(__this: &compat::Shared<dyn OptionAdder>, __p0: bool, __p1: Arc<dyn Fn() -> compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAdder> {
        <dyn Builder>::option_if_boolean_supplier_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Builder>), __p0, __p1)
    }

    /// java: @Override default Builder options(List<? extends Option<?>> options)
    pub fn options_body(__this: &compat::Shared<dyn Builder>, options: &Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn Builder> {
        // TODO(fabric2seer): static field OptionAdder.super
        //   java: OptionAdder.super.options(options);
        return __this.clone();
    }

    /// Builder.options as the dispatch of OptionAdder.options for the concrete class `__C`
    pub fn options_from<__C: Builder + 'static>(__this: &compat::Shared<dyn OptionAdder>, __p0: &Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAdder> {
        <dyn Builder>::options_body(&compat::view_as::<__C, _, _>(__this, |a| a as compat::Shared<dyn Builder>), __p0)
    }

    /// java: default Builder groupIf(boolean condition, OptionGroup group)
    pub fn group_if_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, group: &compat::Shared<dyn OptionGroup>) -> compat::Shared<dyn Builder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.group()); __f(__this, group.clone()) }) } else { __this.clone() });
    }

    /// java: default Builder groups(List<OptionGroup> groups)
    pub fn groups_body(__this: &compat::Shared<dyn Builder>, groups: &Vec<compat::Shared<dyn OptionGroup>>) -> compat::Shared<dyn Builder> {
        for group in groups.iter() {
            ({ let __f = compat::dispatch(__this, |__t| __t.group()); __f(__this, group.clone()) });
        }
        return __this.clone();
    }

}
