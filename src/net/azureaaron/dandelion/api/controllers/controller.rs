//! Ported from net.azureaaron.dandelion.api.controllers.Controller by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use ::yacl::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_option_editor_adapter::MoulConfigOptionEditorAdapter;

/// java: public sealed interface Controller<T> permits BooleanController, ColourController, EnumController, NumberController, ItemController, StringController
pub trait Controller<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @ApiStatus.Experimental default GuiOptionEditor controllerMoulConfig(Option<T> option, ProcessedOption moulConfigOption, MoulConfigDefinition configDefinition)
    fn controller_moul_config(&self, option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>, moul_config_option: &crate::unresolved::ProcessedOption, config_definition: &compat::Shared<MoulConfigDefinition>) -> crate::unresolved::GuiOptionEditor {
        return MoulConfigOptionEditorAdapter::create_moul_config_editor(option, moul_config_option, config_definition);
    }

    /// java: @ApiStatus.Experimental @SuppressWarnings("unchecked") default ControllerBuilder<T> controllerYACL(dev.isxander.yacl3.api.Option<T> yaclOption, Class<T> type)
    fn controller_yacl(&self, yacl_option: &compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option::Option<T>>, r#type: &compat::ClassOf<T>) -> compat::Shared<dyn ControllerBuilder<T>> {
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return YACLControllerAdapter.toYaclControllerBuilder(yaclOption, type, this);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

/// java: interface Builder<T, I extends Controller<T>>
pub trait Builder<T: Clone + 'static, I: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: I build();
    fn build(&self) -> I;

}
