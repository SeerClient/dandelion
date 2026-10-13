//! Ported from net.azureaaron.dandelion.api.controllers.BooleanController.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::r#impl::controllers::boolean_controller_impl::BooleanControllerBuilderImpl;

/// java: public non-sealed interface BooleanController extends Controller<Boolean>
pub trait BooleanController: Controller<bool> {
    /// java: BooleanStyle style();
    fn style(&self) -> BooleanStyle;

    /// java: boolean coloured();
    fn coloured(&self) -> bool;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> BooleanControllerBuilderImpl {
    BooleanControllerBuilderImpl::new()
}

/// java: interface Builder extends Controller.Builder<Boolean, BooleanController>
pub trait Builder: controller::Builder<bool, dyn BooleanController> {
    /// java: Builder booleanStyle(BooleanStyle style);
    fn boolean_style(self, style: BooleanStyle) -> Self;

    /// java: Builder coloured(boolean coloured);
    fn coloured(self, coloured: bool) -> Self;
}

/// java: enum BooleanStyle
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BooleanStyle {
    YesNo,
    OnOff,
    TrueFalse,
}

impl BooleanStyle {
    pub const VALUES: [BooleanStyle; 3] = [BooleanStyle::YesNo, BooleanStyle::OnOff, BooleanStyle::TrueFalse];

    /// java: values()
    pub fn values() -> Vec<BooleanStyle> {
        Self::VALUES.to_vec()
    }

    /// java: ordinal()
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// java: name()
    pub fn name(self) -> &'static str {
        match self {
            BooleanStyle::YesNo => "YES_NO",
            BooleanStyle::OnOff => "ON_OFF",
            BooleanStyle::TrueFalse => "TRUE_FALSE",
        }
    }

    /// java: valueOf(String)
    pub fn value_of(name: &str) -> BooleanStyle {
        match name {
            "YES_NO" => BooleanStyle::YesNo,
            "ON_OFF" => BooleanStyle::OnOff,
            "TRUE_FALSE" => BooleanStyle::TrueFalse,
            _ => panic!("IllegalArgumentException: No enum constant BooleanStyle.{name}"),
        }
    }
}

impl std::fmt::Display for BooleanStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
