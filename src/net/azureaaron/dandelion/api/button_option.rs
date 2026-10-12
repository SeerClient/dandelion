//! Ported from net.azureaaron.dandelion.api.ButtonOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::button_option_impl::ButtonOptionBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface ButtonOption extends Option<Consumer<Screen>>
pub trait ButtonOption: crate::net::azureaaron::dandelion::api::option::Option + compat::DynAny {
    /// java: Component prompt(); (dispatch: the body for this class)
    fn prompt(&self) -> fn(&compat::Shared<dyn ButtonOption>) -> MutableComponent;

    /// java: Consumer<Screen> action(); (dispatch: the body for this class)
    fn action(&self) -> fn(&compat::Shared<dyn ButtonOption>) -> Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (ButtonOptionBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder
pub trait Builder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder id(Identifier id); (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn Builder>, Identifier) -> compat::Shared<dyn Builder>;

    /// java: Builder name(Component name); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder description(Component... texts); (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: Builder tags(Component... tags); (dispatch: the body for this class)
    fn tags(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder>;

    /// java: Builder prompt(Component prompt); (dispatch: the body for this class)
    fn prompt(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder action(Consumer<Screen> action); (dispatch: the body for this class)
    fn action(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>) -> compat::Shared<dyn Builder>;

    /// java: ButtonOption build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn ButtonOption>;

}
