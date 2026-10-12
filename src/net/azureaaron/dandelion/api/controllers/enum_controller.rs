//! Ported from net.azureaaron.dandelion.api.controllers.EnumController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::r#impl::controllers::enum_controller_impl::EnumControllerBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public non-sealed interface EnumController<T extends Enum<T>> extends Controller<T>
pub trait EnumController<T: Clone + 'static>: Controller {
    /// java: boolean dropdown();
    fn dropdown(&self) -> bool;

    /// java: Function<T, Component> formatter();
    fn formatter(&self) -> Arc<dyn Fn(T) -> MutableComponent>;

}

/// java: static <T extends Enum<T>> Builder<T> createBuilder()
pub fn create_builder<T: 'static>() -> compat::Shared<dyn Builder<T>> {
    return (compat::shared(EnumControllerBuilderImpl::new()) as compat::Shared<dyn Builder<compat::Object>>);
}

/// java: interface Builder<T extends Enum<T>> extends Controller.Builder<T, EnumController<T>>
pub trait Builder<T: Clone + 'static>: crate::net::azureaaron::dandelion::api::controllers::controller::Builder {
    /// java: Builder<T> dropdown(boolean dropdown);
    fn dropdown(&mut self, dropdown: bool) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> formatter(Function<T, Component> formatter);
    fn formatter(&mut self, formatter: Arc<dyn Fn(T) -> MutableComponent>) -> compat::Shared<dyn Builder<T>>;

}
