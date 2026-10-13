//! Ported from net.azureaaron.dandelion.api.ListOption.
//!
//! Both supertraits declare `id()` / `name()` / `description()` / `tags()`: on a list option call them as
//! `AnyOption::name(&*x)` or `OptionGroup::name(&*x)` (Java's single method).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::option::Option;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use crate::net::azureaaron::dandelion::r#impl::list_option_impl::BuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface ListOption<T> extends Option<List<T>>, OptionGroup
pub trait ListOption<T: Clone + PartialEq + 'static>: Option<Vec<T>> + OptionGroup {
    /// java: Supplier<T> initialValue();
    fn initial_value(&self) -> Arc<dyn Fn() -> T>;

    /// java: Controller<T> entryController();
    fn entry_controller(&self) -> Arc<dyn Controller<T>>;

    /// java: Class<T> entryType();
    fn entry_type(&self) -> compat::ClassOf<T>;
}

/// java: static <T> Builder<T> createBuilder()
pub fn create_builder<T: Clone + PartialEq + 'static>() -> BuilderImpl<T> {
    BuilderImpl::new()
}

/// java: interface Builder<T>
pub trait Builder<T: Clone + PartialEq + 'static>: Sized {
    /// java: Builder<T> id(Identifier id);
    fn id(self, id: Identifier) -> Self;

    /// java: Builder<T> name(Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder<T> description(Component... texts);
    fn description(self, texts: Vec<MutableComponent>) -> Self;

    /// java: Builder<T> tags(Component... tags);
    fn tags(self, tags: Vec<MutableComponent>) -> Self;

    /// java: Builder<T> binding(List<T> defaultValue, Supplier<List<T>> getter, Consumer<List<T>> setter);
    fn binding(self, default_value: Vec<T>, getter: impl Fn() -> Vec<T> + 'static, setter: impl Fn(Vec<T>) + 'static) -> Self;

    /// java: Builder<T> controller(Controller<T> controller);
    fn controller(self, controller: Arc<dyn Controller<T>>) -> Self;

    /// java: Builder<T> initial(Supplier<T> initialValue);
    fn initial(self, initial_value: impl Fn() -> T + 'static) -> Self;

    /// java: Builder<T> initial(T initialValue);
    fn initial_t(self, initial_value: T) -> Self;

    /// java: Builder<T> collapsed(boolean collapsed);
    fn collapsed(self, collapsed: bool) -> Self;

    /// java: Builder<T> modifiable(boolean modifiable);
    fn modifiable(self, modifiable: bool) -> Self;

    /// java: Builder<T> flags(OptionFlag... flags);
    fn flags(self, flags: Vec<Arc<dyn OptionFlag>>) -> Self;

    /// java: Builder<T> listener(OptionListener<List<T>> listener);
    fn listener(self, listener: Arc<dyn OptionListener<Vec<T>>>) -> Self;

    /// java: ListOption<T> build();
    fn build(self) -> Arc<dyn ListOption<T>>;
}
