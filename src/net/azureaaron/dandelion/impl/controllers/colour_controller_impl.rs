//! Ported from net.azureaaron.dandelion.impl.controllers.ColourControllerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::colour_controller::Builder;
use crate::net::azureaaron::dandelion::api::controllers::colour_controller::ColourController;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;

/// java: public class ColourControllerImpl implements ColourController
#[derive(Clone)]
pub struct ColourControllerImpl {
    /// java: private final boolean hasAlpha;
    pub has_alpha: bool,
}

impl ColourControllerImpl {
    /// java: protected ColourControllerImpl(boolean hasAlpha)
    pub fn new(mut has_alpha: bool) -> compat::Shared<Self> {
        let __init_has_alpha = has_alpha;
        let __this = compat::shared(Self {
            has_alpha: __init_has_alpha,
        });
        __this
    }

}

impl ColourController for ColourControllerImpl {
    /// java: @Override public boolean hasAlpha() (dispatch: the body for this class)
    fn has_alpha(&self) -> fn(&compat::Shared<dyn ColourController>) -> bool {
        ColourControllerImpl::has_alpha_dyn
    }

}

impl Controller<compat::Object> for ColourControllerImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

impl ColourControllerImpl {
    /// java: @Override public boolean hasAlpha()
    pub fn has_alpha_impl(__this: &compat::Shared<Self>) -> bool {
        return compat::read(__this, |__t| __t.has_alpha);
    }

    /// ColourControllerImpl.hasAlpha as the dispatch of ColourController.hasAlpha
    pub fn has_alpha_dyn(__this: &compat::Shared<dyn ColourController>) -> bool {
        Self::has_alpha_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class ColourControllerBuilderImpl implements ColourController.Builder
#[derive(Clone)]
pub struct ColourControllerBuilderImpl {
    /// java: private boolean hasAlpha = false;
    pub has_alpha: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl ColourControllerBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            has_alpha: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl Builder for ColourControllerBuilderImpl {
    /// java: @Override public ColourController.Builder hasAlpha(boolean hasAlpha) (dispatch: the body for this class)
    fn has_alpha(&self) -> fn(&compat::Shared<dyn Builder>, bool) -> compat::Shared<dyn Builder> {
        ColourControllerBuilderImpl::has_alpha_dyn
    }

}

impl crate::net::azureaaron::dandelion::api::controllers::controller::Builder<compat::Object, compat::Object> for ColourControllerBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public ColourController build()
    fn build(&self) -> compat::Shared<dyn ColourController> {
        return (ColourControllerImpl::new(self.has_alpha) as compat::Shared<dyn ColourController>);
    }

}

impl ColourControllerBuilderImpl {
    /// java: @Override public ColourController.Builder hasAlpha(boolean hasAlpha)
    pub fn has_alpha_impl(__this: &compat::Shared<Self>, mut has_alpha: bool) -> compat::Shared<dyn Builder> {
        { let __v = has_alpha; compat::write(__this, |__t| __t.has_alpha = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ColourControllerBuilderImpl.hasAlpha as the dispatch of Builder.hasAlpha
    pub fn has_alpha_dyn(__this: &compat::Shared<dyn Builder>, __p0: bool) -> compat::Shared<dyn Builder> {
        Self::has_alpha_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
