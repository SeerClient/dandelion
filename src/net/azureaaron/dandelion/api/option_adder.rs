//! Ported from net.azureaaron.dandelion.api.OptionAdder by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public sealed interface OptionAdder permits ConfigCategory.Builder, OptionGroup.Builder
pub trait OptionAdder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: OptionAdder option(Option<?> option); (dispatch: the body for this class)
    fn option(&self) -> fn(&compat::Shared<dyn OptionAdder>, &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAdder>;

    /// java: default OptionAdder optionIf(boolean condition, Option<?> option) (dispatch: the body for this class)
    fn option_if(&self) -> fn(&compat::Shared<dyn OptionAdder>, bool, &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAdder> {
        <dyn OptionAdder>::option_if_body
    }

    /// java: default OptionAdder optionIf(boolean condition, Supplier<Option<?>> option) (dispatch: the body for this class)
    fn option_if_boolean_supplier(&self) -> fn(&compat::Shared<dyn OptionAdder>, bool, Arc<dyn Fn() -> compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAdder> {
        <dyn OptionAdder>::option_if_boolean_supplier_body
    }

    /// java: default OptionAdder options(List<? extends Option<?>> options) (dispatch: the body for this class)
    fn options(&self) -> fn(&compat::Shared<dyn OptionAdder>, &Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAdder> {
        <dyn OptionAdder>::options_body
    }

}

impl dyn OptionAdder {
    /// java: default OptionAdder optionIf(boolean condition, Option<?> option)
    pub fn option_if_body(__this: &compat::Shared<dyn OptionAdder>, mut condition: bool, option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> compat::Shared<dyn OptionAdder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.option()); __f(__this, option) }) } else { __this.clone() });
    }

    /// java: default OptionAdder optionIf(boolean condition, Supplier<Option<?>> option)
    pub fn option_if_boolean_supplier_body(__this: &compat::Shared<dyn OptionAdder>, mut condition: bool, option: Arc<dyn Fn() -> compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAdder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.option()); __f(__this, &(option)()) }) } else { __this.clone() });
    }

    /// java: default OptionAdder options(List<? extends Option<?>> options)
    pub fn options_body(__this: &compat::Shared<dyn OptionAdder>, options: &Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>) -> compat::Shared<dyn OptionAdder> {
        for option in options.iter() {
            ({ let __f = compat::dispatch(__this, |__t| __t.option()); __f(__this, option) });
        }
        return __this.clone();
    }

}
