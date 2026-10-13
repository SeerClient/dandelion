//! Ported from net.azureaaron.dandelion.impl.yacl.YACLControllerAdapter.
//!
//! Java's `(Option<Boolean>) yaclOption` casts are `compat::java_cast`: the arm of the sealed switch proves `T`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::boolean_controller::BooleanStyle;
use crate::net::azureaaron::dandelion::api::controllers::controller::{Controller, ControllerKind, NumberControllerKind};
use crate::net::azureaaron::dandelion::api::controllers::enum_controller::EnumController;
use ::yacl::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use ::yacl::dev::isxander::yacl3::api::controller::{
    boolean_controller_builder, color_controller_builder, enum_controller_builder, enum_dropdown_controller_builder,
    float_field_controller_builder, float_slider_controller_builder, integer_field_controller_builder,
    integer_slider_controller_builder, item_controller_builder, string_controller_builder,
};
use ::yacl::dev::isxander::yacl3::api::option::Option as YOption;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::world::item::item::Item;

/// java: public class YACLControllerAdapter
pub struct YACLControllerAdapter {}

impl YACLControllerAdapter {
    /// java: @SuppressWarnings({ "unchecked", "rawtypes" }) public static <T> ControllerBuilder toYaclControllerBuilder(
    /// dev.isxander.yacl3.api.Option<T> yaclOption, Class<T> type, Controller<T> controller)
    pub fn to_yacl_controller_builder<T: Clone + PartialEq + 'static, C: Controller<T> + ?Sized>(
        yacl_option: Arc<dyn YOption<T>>,
        r#type: compat::ClassOf<T>,
        controller: &C,
    ) -> Box<dyn ControllerBuilder<T>> {
        match controller.kind() {
            ControllerKind::Boolean(boolean_controller) => {
                let yacl_controller = boolean_controller_builder::create(compat::java_cast::<_, Arc<dyn YOption<bool>>>(yacl_option))
                    .coloured(boolean_controller.coloured());
                let yacl_controller = match boolean_controller.style() {
                    BooleanStyle::OnOff => yacl_controller.on_off_formatter(),
                    BooleanStyle::YesNo => yacl_controller.yes_no_formatter(),
                    BooleanStyle::TrueFalse => yacl_controller.true_false_formatter(),
                };
                compat::java_cast::<Box<dyn ControllerBuilder<bool>>, _>(Box::new(yacl_controller))
            }
            ControllerKind::Colour(colour_controller) => {
                let builder = color_controller_builder::create(compat::java_cast::<_, Arc<dyn YOption<Color>>>(yacl_option))
                    .allow_alpha(colour_controller.has_alpha());
                compat::java_cast::<Box<dyn ControllerBuilder<Color>>, _>(Box::new(builder))
            }
            ControllerKind::Enum(enum_controller) => {
                // the arm needs `T extends Enum<T>`: it runs where the controller knows its `T`
                let erased = enum_controller.to_yacl_controller_builder_erased(Box::new(yacl_option), Box::new(r#type));
                *erased
                    .downcast::<Box<dyn ControllerBuilder<T>>>()
                    .unwrap_or_else(|_| panic!("ClassCastException: the enum controller's builder is not of the option's type"))
            }
            ControllerKind::Number(NumberControllerKind::Integer(i)) => {
                let option = compat::java_cast::<_, Arc<dyn YOption<i32>>>(yacl_option);
                let builder: Box<dyn ControllerBuilder<i32>> = if i.slider() {
                    Box::new(integer_slider_controller_builder::create(option).range(i.min(), i.max()).step(i.step()))
                } else {
                    Box::new(integer_field_controller_builder::create(option).range(i.min(), i.max()))
                };
                compat::java_cast(builder)
            }
            ControllerKind::Number(NumberControllerKind::Float(f)) => {
                let option = compat::java_cast::<_, Arc<dyn YOption<f32>>>(yacl_option);
                let builder: Box<dyn ControllerBuilder<f32>> = if f.slider() {
                    Box::new(float_slider_controller_builder::create(option).range(f.min(), f.max()).step(f.step()))
                } else {
                    Box::new(float_field_controller_builder::create(option).range(f.min(), f.max()))
                };
                compat::java_cast(builder)
            }
            ControllerKind::Item(_) => {
                let builder = item_controller_builder::create(compat::java_cast::<_, Arc<dyn YOption<compat::JavaRef<Item>>>>(yacl_option));
                compat::java_cast::<Box<dyn ControllerBuilder<compat::JavaRef<Item>>>, _>(Box::new(builder))
            }
            ControllerKind::String(_) => {
                let builder = string_controller_builder::create(compat::java_cast::<_, Arc<dyn YOption<String>>>(yacl_option));
                compat::java_cast::<Box<dyn ControllerBuilder<String>>, _>(Box::new(builder))
            }
        }
    }

    /// The `case EnumController<?> enumController` arm of `toYaclControllerBuilder`, with `T` known to be an enum.
    pub fn to_yacl_enum_controller_builder<T: compat::JavaEnum>(
        yacl_option: Arc<dyn YOption<T>>,
        r#type: compat::ClassOf<T>,
        enum_controller: &dyn EnumController<T>,
    ) -> Box<dyn ControllerBuilder<T>> {
        // java: ValueFormatter<T> yaclFormatter = ((Function<T, Component>) enumController.formatter())::apply;
        let formatter = enum_controller.formatter();
        if !enum_controller.dropdown() {
            Box::new(enum_controller_builder::create(yacl_option).enum_class(r#type).value_formatter(move |v: T| formatter(v)))
        } else {
            Box::new(enum_dropdown_controller_builder::create(yacl_option).value_formatter(move |v: T| formatter(v)))
        }
    }
}
