//! Ported from net.azureaaron.dandelion.impl.yacl.YACLControllerAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use ::yacl::dev::isxander::yacl3::api::controller::boolean_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::boolean_controller_builder::BooleanControllerBuilder;
use ::yacl::dev::isxander::yacl3::api::controller::color_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::color_controller_builder::ColorControllerBuilder;
use ::yacl::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use ::yacl::dev::isxander::yacl3::api::controller::enum_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::enum_dropdown_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::float_field_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::float_slider_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::integer_field_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::integer_slider_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::item_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::string_controller_builder;
use ::yacl::dev::isxander::yacl3::api::controller::value_formatter::ValueFormatter;
use crate::net::azureaaron::dandelion::api::controllers::boolean_controller::BooleanController;
use crate::net::azureaaron::dandelion::api::controllers::boolean_controller::BooleanStyle;
use crate::net::azureaaron::dandelion::api::controllers::colour_controller::ColourController;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::controllers::enum_controller::EnumController;
use crate::net::azureaaron::dandelion::api::controllers::float_controller::FloatController;
use crate::net::azureaaron::dandelion::api::controllers::integer_controller::IntegerController;
use crate::net::azureaaron::dandelion::api::controllers::item_controller::ItemController;
use crate::net::azureaaron::dandelion::api::controllers::number_controller::NumberController;
use crate::net::azureaaron::dandelion::api::controllers::string_controller::StringController;
use ::yacl::dev::isxander::yacl3::api::controller::enum_controller_builder::EnumControllerBuilder as _;
use ::yacl::dev::isxander::yacl3::api::controller::number_field_controller_builder::NumberFieldControllerBuilder as _;
use ::yacl::dev::isxander::yacl3::api::controller::slider_controller_builder::SliderControllerBuilder as _;
use ::yacl::dev::isxander::yacl3::api::controller::value_formattable_controller::ValueFormattableController as _;

/// java: public class YACLControllerAdapter
#[derive(Clone)]
pub struct YACLControllerAdapter {}

impl YACLControllerAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: @SuppressWarnings(
    pub fn to_yacl_controller_builder<T: Clone + 'static>(yacl_option: &compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option::Option<T>>, r#type: &compat::ClassOf<T>, controller: &compat::Shared<dyn Controller<T>>) -> compat::Shared<dyn ControllerBuilder<compat::Object>> {
        if let Some(boolean_controller) = controller.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn BooleanController>>() {
            let mut yacl_controller: compat::Shared<dyn BooleanControllerBuilder> = ({ let __r = &boolean_controller_builder::create(yacl_option); let __f = compat::dispatch(__r, |__t| __t.coloured()); __f(__r, ({ let __f = compat::dispatch(boolean_controller, |__t| __t.coloured()); __f(boolean_controller) })) });
            yacl_controller = match ({ let __f = compat::dispatch(boolean_controller, |__t| __t.style()); __f(boolean_controller) }) {
                BooleanStyle::OnOff => ({ let __f = compat::dispatch(&yacl_controller, |__t| __t.on_off_formatter()); __f(&yacl_controller) }),
                BooleanStyle::YesNo => ({ let __f = compat::dispatch(&yacl_controller, |__t| __t.yes_no_formatter()); __f(&yacl_controller) }),
                BooleanStyle::TrueFalse => ({ let __f = compat::dispatch(&yacl_controller, |__t| __t.true_false_formatter()); __f(&yacl_controller) }),
                _ => unreachable!("switch expression: no case matched"),
            };
            return (yacl_controller as compat::Shared<dyn ControllerBuilder<compat::Object>>);
        } else if let Some(colour_controller) = controller.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ColourController>>() {
            return (({ let __r = &color_controller_builder::create(yacl_option); let __f = compat::dispatch(__r, |__t| __t.allow_alpha()); __f(__r, ({ let __f = compat::dispatch(colour_controller, |__t| __t.has_alpha()); __f(colour_controller) })) }) as compat::Shared<dyn ControllerBuilder<compat::Object>>);
        } else if let Some(enum_controller) = controller.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn EnumController<compat::Object>>>() {
            let mut yacl_formatter: compat::Shared<dyn ValueFormatter<T>> = Arc::new(move |__a0| (enum_controller.lock().borrow().formatter())(__a0.clone()));
            if !(enum_controller.lock().borrow().dropdown()) {
                return { let (__a0,) = (yacl_formatter.clone(),); ({ let (__a0,) = (r#type.clone(),); enum_controller_builder::create(yacl_option).lock().borrow_mut().enum_class(__a0) }.lock().borrow_mut()).format_value(__a0) };
            } else {
                return { let (__a0,) = (yacl_formatter.clone(),); enum_dropdown_controller_builder::create(yacl_option).lock().borrow_mut().format_value(__a0) };
            }
        } else if let Some(number_controller) = controller.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn NumberController<compat::Object>>>() {
            if let Some(i) = number_controller.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn IntegerController>>() {
                if i.lock().borrow().slider() {
                    // TODO(fabric2seer): `.step(..)` on a B
                    //   java: return IntegerSliderControllerBuilder.create((Option<Integer>) yaclOption).range(i.min(), i.max()).step(i.step());
                } else {
                    return { let (__a0, __a1,) = (&i.lock().borrow().min(), &i.lock().borrow().max(),); integer_field_controller_builder::create(yacl_option).lock().borrow_mut().range(__a0, __a1) };
                }
            } else if let Some(f) = number_controller.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn FloatController>>() {
                if f.lock().borrow().slider() {
                    // TODO(fabric2seer): `.step(..)` on a B
                    //   java: return FloatSliderControllerBuilder.create((Option<Float>) yaclOption).range(f.min(), f.max()).step(f.step());
                } else {
                    return { let (__a0, __a1,) = (&f.lock().borrow().min(), &f.lock().borrow().max(),); float_field_controller_builder::create(yacl_option).lock().borrow_mut().range(__a0, __a1) };
                }
            }
        } else if let Some(_) = controller.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn ItemController>>() {
            return (item_controller_builder::create(yacl_option) as compat::Shared<dyn ControllerBuilder<compat::Object>>);
        } else if let Some(_) = controller.lock().borrow().as_any().downcast_ref::<compat::Shared<dyn StringController>>() {
            return (string_controller_builder::create(yacl_option) as compat::Shared<dyn ControllerBuilder<compat::Object>>);
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
