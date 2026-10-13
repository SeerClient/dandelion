//! Ported from net.azureaaron.dandelion.api.LabelOption.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option::Option;
use crate::net::azureaaron::dandelion::r#impl::label_option_impl::LabelOptionBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface LabelOption extends Option<Component>
pub trait LabelOption: Option<MutableComponent> {
    /// java: Component label();
    fn label(&self) -> MutableComponent;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> LabelOptionBuilderImpl {
    LabelOptionBuilderImpl::new()
}

/// java: interface Builder
pub trait Builder: Sized {
    /// java: Builder label(Component... texts);
    fn label(self, texts: Vec<MutableComponent>) -> Self;

    /// java: LabelOption build();
    fn build(self) -> Arc<dyn LabelOption>;
}
