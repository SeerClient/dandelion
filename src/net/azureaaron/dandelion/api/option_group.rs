//! Ported from net.azureaaron.dandelion.api.OptionGroup by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option_adder::OptionAdder;
use crate::net::azureaaron::dandelion::r#impl::option_group_impl::OptionGroupBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface OptionGroup
pub trait OptionGroup: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @Nullable Identifier id(); (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Option<Identifier>;

    /// java: Component name(); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> MutableComponent;

    /// java: List<Component> description(); (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent>;

    /// java: List<Component> tags(); (dispatch: the body for this class)
    fn tags(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<MutableComponent>;

    /// java: boolean collapsed(); (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> bool;

    /// java: List<? extends Option<?>> options(); (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionGroup>) -> Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (OptionGroupBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: non-sealed interface Builder extends OptionAdder
pub trait Builder: OptionAdder {
    /// java: Builder id(Identifier id); (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn Builder>, Identifier) -> compat::Shared<dyn Builder>;

    /// java: Builder name(Component name); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder description(Component... texts); (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: Builder tags(Component... tags); (dispatch: the body for this class)
    fn tags(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: Builder collapsed(boolean collapsed); (dispatch: the body for this class)
    fn collapsed(&self) -> fn(&compat::Shared<dyn Builder>, bool) -> compat::Shared<dyn Builder>;

    /// java: OptionGroup build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn OptionGroup>;

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

}
