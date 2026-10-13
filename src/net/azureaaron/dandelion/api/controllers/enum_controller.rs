//! Ported from net.azureaaron.dandelion.api.controllers.EnumController.
//!
//! `T extends Enum<T>` is `T: compat::JavaEnum` (YACL's enum bound). A backend that switches over a `Controller<T>`
//! does not know that bound, so the enum arm of `Controller.kind()` is the erased `AnyEnumController`: `dropdown()`
//! plus the enum arms of the two adapters, run by the implementation where `T: JavaEnum` is known.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::r#impl::controllers::enum_controller_impl::EnumControllerBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public non-sealed interface EnumController<T extends Enum<T>> extends Controller<T>
pub trait EnumController<T: compat::JavaEnum>: Controller<T> {
    /// java: boolean dropdown();
    fn dropdown(&self) -> bool;

    /// java: Function<T, Component> formatter();
    fn formatter(&self) -> Arc<dyn Fn(T) -> MutableComponent>;
}

/// java: `EnumController<?>` - what a `switch (controller)` over a `Controller<T>` sees of its enum arm.
pub trait AnyEnumController {
    /// java: boolean dropdown();
    fn dropdown(&self) -> bool;

    /// java: the `case EnumController<?> enumController` arm of `YACLControllerAdapter.toYaclControllerBuilder`, with
    /// Java's unchecked casts: `yacl_option` is a `Box` of the `Arc<dyn dev.isxander.yacl3.api.Option<T>>`, `type` a
    /// `Box` of the `compat::ClassOf<T>`, and the answer a `Box` of the `Box<dyn ControllerBuilder<T>>` (a mismatched
    /// `T` is the `ClassCastException`).
    fn to_yacl_controller_builder_erased(&self, yacl_option: Box<dyn std::any::Any>, r#type: Box<dyn std::any::Any>) -> Box<dyn std::any::Any>;

    /// java: the `String[] displayValues` of the `case EnumController enumController` arm of
    /// `MoulConfigOptionEditorAdapter.createMoulConfigEditor`: every constant of `option.type()` through `formatter()`,
    /// as its `getString()`.
    fn display_values(&self) -> Vec<String>;
}

/// java: static <T extends Enum<T>> Builder<T> createBuilder()
pub fn create_builder<T: compat::JavaEnum>() -> EnumControllerBuilderImpl<T> {
    EnumControllerBuilderImpl::new()
}

/// java: interface Builder<T extends Enum<T>> extends Controller.Builder<T, EnumController<T>>
pub trait Builder<T: compat::JavaEnum>: controller::Builder<T, dyn EnumController<T>> {
    /// java: Builder<T> dropdown(boolean dropdown);
    fn dropdown(self, dropdown: bool) -> Self;

    /// java: Builder<T> formatter(Function<T, Component> formatter);
    fn formatter(self, formatter: impl Fn(T) -> MutableComponent + 'static) -> Self;
}
