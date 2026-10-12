//! Ported from net.azureaaron.dandelion.api.LabelOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::label_option_impl::LabelOptionBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface LabelOption extends Option<Component>
pub trait LabelOption: crate::net::azureaaron::dandelion::api::option::Option + compat::DynAny {
    /// java: Component label(); (dispatch: the body for this class)
    fn label(&self) -> fn(&compat::Shared<dyn LabelOption>) -> MutableComponent;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (LabelOptionBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder
pub trait Builder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder label(Component... texts); (dispatch: the body for this class)
    fn label(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: LabelOption build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn LabelOption>;

}
