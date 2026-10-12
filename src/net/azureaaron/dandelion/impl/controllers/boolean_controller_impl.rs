//! Ported from net.azureaaron.dandelion.impl.controllers.BooleanControllerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::boolean_controller::BooleanController;
use crate::net::azureaaron::dandelion::api::controllers::boolean_controller::BooleanStyle;
use crate::net::azureaaron::dandelion::api::controllers::boolean_controller::Builder;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;

/// java: public class BooleanControllerImpl implements BooleanController
#[derive(Clone)]
pub struct BooleanControllerImpl {
    /// java: private final BooleanStyle style;
    pub style: BooleanStyle,
    /// java: private final boolean coloured;
    pub coloured: bool,
}

impl BooleanControllerImpl {
    /// java: protected BooleanControllerImpl(BooleanStyle style, boolean coloured)
    pub fn new(mut style: BooleanStyle, mut coloured: bool) -> compat::Shared<Self> {
        let __init_style = style;
        let __init_coloured = coloured;
        let __this = compat::shared(Self {
            style: __init_style,
            coloured: __init_coloured,
        });
        __this
    }

}

impl BooleanController for BooleanControllerImpl {
    /// java: @Override public BooleanStyle style() (dispatch: the body for this class)
    fn style(&self) -> fn(&compat::Shared<dyn BooleanController>) -> BooleanStyle {
        BooleanControllerImpl::style_dyn
    }

    /// java: @Override public boolean coloured() (dispatch: the body for this class)
    fn coloured(&self) -> fn(&compat::Shared<dyn BooleanController>) -> bool {
        BooleanControllerImpl::coloured_dyn
    }

}

impl Controller<compat::Object> for BooleanControllerImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl BooleanControllerImpl {
    /// java: @Override public BooleanStyle style()
    pub fn style_impl(__this: &compat::Shared<Self>) -> BooleanStyle {
        return compat::read(__this, |__t| __t.style);
    }

    /// BooleanControllerImpl.style as the dispatch of BooleanController.style
    pub fn style_dyn(__this: &compat::Shared<dyn BooleanController>) -> BooleanStyle {
        Self::style_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public boolean coloured()
    pub fn coloured_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.coloured);
    }

    /// BooleanControllerImpl.coloured as the dispatch of BooleanController.coloured
    pub fn coloured_dyn(__this: &compat::Shared<dyn BooleanController>) -> bool {
        Self::coloured_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class BooleanControllerBuilderImpl implements BooleanController.Builder
#[derive(Clone)]
pub struct BooleanControllerBuilderImpl {
    /// java: private BooleanStyle style = BooleanStyle.ON_OFF;
    pub style: BooleanStyle,
    /// java: private boolean coloured = false;
    pub coloured: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl BooleanControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            style: BooleanStyle::OnOff,
            coloured: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl Builder for BooleanControllerBuilderImpl {
    /// java: @Override public BooleanController.Builder booleanStyle(BooleanStyle style) (dispatch: the body for this class)
    fn boolean_style(&self) -> fn(&compat::Shared<dyn Builder>, BooleanStyle) -> compat::Shared<dyn Builder> {
        BooleanControllerBuilderImpl::boolean_style_dyn
    }

    /// java: @Override public BooleanController.Builder coloured(boolean coloured) (dispatch: the body for this class)
    fn coloured(&self) -> fn(&compat::Shared<dyn Builder>, bool) -> compat::Shared<dyn Builder> {
        BooleanControllerBuilderImpl::coloured_dyn
    }

}

impl crate::net::azureaaron::dandelion::api::controllers::controller::Builder<compat::Object, compat::Object> for BooleanControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public BooleanController build()
    fn build(&self) -> compat::Shared<dyn BooleanController> {
        return (BooleanControllerImpl::new(self.style, self.coloured) as compat::Shared<dyn BooleanController>);
    }

}

impl BooleanControllerBuilderImpl {
    /// java: @Override public BooleanController.Builder booleanStyle(BooleanStyle style)
    pub fn boolean_style_impl(__this: &compat::Shared<Self>, mut style: BooleanStyle) -> compat::Shared<dyn Builder> {
        { let __v = style; compat::write(__this, |__t| __t.style = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BooleanControllerBuilderImpl.booleanStyle as the dispatch of Builder.booleanStyle
    pub fn boolean_style_dyn(__this: &compat::Shared<dyn Builder>, __p0: BooleanStyle) -> compat::Shared<dyn Builder> {
        Self::boolean_style_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public BooleanController.Builder coloured(boolean coloured)
    pub fn coloured_impl(__this: &compat::Shared<Self>, mut coloured: bool) -> compat::Shared<dyn Builder> {
        { let __v = coloured; compat::write(__this, |__t| __t.coloured = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// BooleanControllerBuilderImpl.coloured as the dispatch of Builder.coloured
    pub fn coloured_dyn(__this: &compat::Shared<dyn Builder>, __p0: bool) -> compat::Shared<dyn Builder> {
        Self::coloured_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
