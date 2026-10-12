//! Ported from net.azureaaron.dandelion.api.Option by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use crate::net::azureaaron::dandelion::r#impl::option_impl::OptionBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface Option<T>
pub trait Option<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: @Nullable Identifier id();
    fn id(&self) -> Option<Identifier>;

    /// java: Component name();
    fn name(&self) -> MutableComponent;

    /// java: List<Component> description();
    fn description(&self) -> Vec<MutableComponent>;

    /// java: List<Component> tags();
    fn tags(&self) -> Vec<MutableComponent>;

    /// java: OptionBinding<T> binding();
    fn binding(&self) -> compat::Shared<dyn OptionBinding<T>>;

    /// java: Controller<T> controller();
    fn controller(&self) -> compat::Shared<dyn Controller<T>>;

    /// java: boolean modifiable();
    fn modifiable(&self) -> bool;

    /// java: List<OptionFlag> flags();
    fn flags(&self) -> Vec<compat::Shared<dyn OptionFlag>>;

    /// java: List<OptionListener<T>> listeners();
    fn listeners(&self) -> Vec<compat::Shared<dyn OptionListener<T>>>;

    /// java: Class<T> type();
    fn r#type(&self) -> compat::ClassOf<T>;

}

/// java: static <T> Builder<T> createBuilder()
pub fn create_builder<T: 'static>() -> compat::Shared<dyn Builder<T>> {
    return (compat::shared(OptionBuilderImpl::new()) as compat::Shared<dyn Builder<compat::Object>>);
}

/// java: interface Builder<T>
pub trait Builder<T: Clone + 'static> {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder<T> id(Identifier id);
    fn id(&mut self, id: Identifier) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> name(Component name);
    fn name(&mut self, name: MutableComponent) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> description(Component... texts);
    fn description(&mut self, texts: &Vec<MutableComponent>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> tags(Component... tags);
    fn tags(&mut self, tags: &Vec<MutableComponent>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> binding(T defaultValue, Supplier<T> getter, Consumer<T> setter);
    fn binding(&mut self, default_value: &T, getter: Arc<dyn Fn() -> T>, setter: Arc<dyn Fn(T)>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> controller(Controller<T> controller);
    fn controller(&mut self, controller: compat::Shared<dyn Controller<T>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> modifiable(boolean modifiable);
    fn modifiable(&mut self, modifiable: bool) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> flags(OptionFlag... flags);
    fn flags(&mut self, flags: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> listener(OptionListener<T> listener);
    fn listener(&mut self, listener: compat::Shared<dyn OptionListener<T>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Option<T> build();
    fn build(&self) -> compat::Shared<dyn Option<T>>;

}
