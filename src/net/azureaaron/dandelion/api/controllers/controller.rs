//! Ported from net.azureaaron.dandelion.api.controllers.Controller.
//!
//! `sealed interface Controller<T> permits BooleanController, ColourController, EnumController, NumberController,
//! ItemController, StringController`: the backends `switch` over the six (`OptionImpl.checkType`,
//! `YACLControllerAdapter`, `MoulConfigOptionEditorAdapter`). `kind()` is that switch's subject: each permitted
//! interface's implementations answer their own arm. An `EnumController<T>` needs `T: JavaEnum`, which a `Controller<T>`
//! does not know, so its arm carries the erased `AnyEnumController` (the enum arms' bodies run where `T` is known).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::boolean_controller::BooleanController;
use crate::net::azureaaron::dandelion::api::controllers::colour_controller::ColourController;
use crate::net::azureaaron::dandelion::api::controllers::enum_controller::AnyEnumController;
use crate::net::azureaaron::dandelion::api::controllers::float_controller::FloatController;
use crate::net::azureaaron::dandelion::api::controllers::integer_controller::IntegerController;
use crate::net::azureaaron::dandelion::api::controllers::item_controller::ItemController;
use crate::net::azureaaron::dandelion::api::controllers::string_controller::StringController;
use crate::net::azureaaron::dandelion::api::option::Option;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_option_editor_adapter::MoulConfigOptionEditorAdapter;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_controller_adapter::YACLControllerAdapter;
use ::yacl::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use ::yacl::dev::isxander::yacl3::api::option::Option as YOption;

/// The `permits` list of `Controller<T>`: which of the six sealed sub-interfaces a controller is (Java's pattern
/// `switch (controller) { case BooleanController b -> ..; .. }`).
pub enum ControllerKind<'a> {
    Boolean(&'a dyn BooleanController),
    Colour(&'a dyn ColourController),
    Enum(&'a dyn AnyEnumController),
    /// `NumberController<?>` - itself `sealed .. permits IntegerController, FloatController`
    Number(NumberControllerKind<'a>),
    Item(&'a dyn ItemController),
    String(&'a dyn StringController),
}

/// `sealed interface NumberController<T extends Number> extends Controller<T> permits IntegerController,
/// FloatController`
pub enum NumberControllerKind<'a> {
    Integer(&'a dyn IntegerController),
    Float(&'a dyn FloatController),
}

/// java: public sealed interface Controller<T> permits ..
pub trait Controller<T: Clone + PartialEq + 'static>: compat::DynAny {
    /// The sealed hierarchy's arm this controller is (see `ControllerKind`).
    fn kind(&self) -> ControllerKind<'_>;

    /// java: @ApiStatus.Experimental default GuiOptionEditor controllerMoulConfig(Option<T> option, ProcessedOption
    /// moulConfigOption, MoulConfigDefinition configDefinition)
    fn controller_moul_config(
        &self,
        option: Arc<dyn Option<T>>,
        moul_config_option: Arc<dyn ::moulconfig::io::github::notenoughupdates::moulconfig::processor::processed_option::ProcessedOption>,
        config_definition: &Arc<MoulConfigDefinition>,
    ) -> Arc<dyn ::moulconfig::io::github::notenoughupdates::moulconfig::gui::gui_option_editor::GuiOptionEditor> {
        MoulConfigOptionEditorAdapter::create_moul_config_editor(option, moul_config_option, config_definition)
    }

    /// java: @ApiStatus.Experimental default ControllerBuilder<T> controllerYACL(dev.isxander.yacl3.api.Option<T>
    /// yaclOption, Class<T> type)
    fn controller_yacl(&self, yacl_option: Arc<dyn YOption<T>>, r#type: compat::ClassOf<T>) -> Box<dyn ControllerBuilder<T>> {
        YACLControllerAdapter::to_yacl_controller_builder(yacl_option, r#type, self)
    }
}

/// java: interface Builder<T, I extends Controller<T>>
pub trait Builder<T: Clone + PartialEq + 'static, I: Controller<T> + ?Sized>: Sized {
    /// java: I build();
    fn build(self) -> Arc<I>;
}
