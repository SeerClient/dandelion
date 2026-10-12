//! Ported from net.azureaaron.dandelion.api.ListOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use crate::net::azureaaron::dandelion::r#impl::list_option_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface ListOption<T> extends Option<List<T>>, OptionGroup
pub trait ListOption<T: Clone + 'static>: crate::net::azureaaron::dandelion::api::option::Option + OptionGroup {
    /// java: Supplier<T> initialValue();
    fn initial_value(&self) -> Arc<dyn Fn() -> T>;

    /// java: Controller<T> entryController();
    fn entry_controller(&self) -> compat::Shared<dyn Controller<T>>;

    /// java: Class<T> entryType();
    fn entry_type(&self) -> compat::ClassOf<T>;

}

/// java: static <T> Builder<T> createBuilder()
pub fn create_builder<T: 'static>() -> compat::Shared<dyn Builder<T>> {
    return (compat::shared(BuilderImpl::new()) as compat::Shared<dyn Builder<compat::Object>>);
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

    /// java: Builder<T> binding(List<T> defaultValue, Supplier<List<T>> getter, Consumer<List<T>> setter);
    fn binding(&mut self, default_value: &Vec<T>, getter: Arc<dyn Fn() -> Vec<T>>, setter: Arc<dyn Fn(Vec<T>)>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> controller(Controller<T> controller);
    fn controller(&mut self, controller: compat::Shared<dyn Controller<T>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> initial(Supplier<T> initialValue);
    fn initial(&mut self, initial_value: Arc<dyn Fn() -> T>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> initial(T initialValue);
    fn initial_t(&mut self, initial_value: &T) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> collapsed(boolean collapsed);
    fn collapsed(&mut self, collapsed: bool) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> modifiable(boolean modifiable);
    fn modifiable(&mut self, modifiable: bool) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> flags(OptionFlag... flags);
    fn flags(&mut self, flags: &Vec<compat::Shared<dyn OptionFlag>>) -> compat::Shared<dyn Builder<T>>;

    /// java: Builder<T> listener(OptionListener<List<T>> listener);
    fn listener(&mut self, listener: compat::Shared<dyn OptionListener<Vec<T>>>) -> compat::Shared<dyn Builder<T>>;

    /// java: ListOption<T> build();
    fn build(&self) -> compat::Shared<dyn ListOption<T>>;

}
