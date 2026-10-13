//! Ported from net.azureaaron.dandelion.api.Option.
//!
//! `Option<T>` is the typed interface; `AnyOption` is Java's `Option<?>` - every method that does not mention `T`,
//! plus the erased forms of the generic adapter calls Java makes through a wildcard (`YACLOptionAdapter.toYaclOption`,
//! `MoulConfigEditableOptionAdapter.createEditableOptionFactory`: both capture `T` from an `Option<?>`) and the
//! `instanceof` tests of the three built-in option kinds. `Option<T>: AnyOption`, so an `Arc<dyn Option<T>>` coerces
//! to `Arc<dyn AnyOption>` wherever Java widens to `Option<?>` (the shape of YACL's `Option` / `AnyOption`,
//! PATTERNS_Y2 P1).
//!
//! Inside this module `Option` is the Dandelion interface; `std::option::Option` is spelled out.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::button_option::ButtonOption;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::key_mapping_option::KeyMappingOption;
use crate::net::azureaaron::dandelion::api::label_option::LabelOption;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_editable_option_adapter::EditableOptionFactory;
use crate::net::azureaaron::dandelion::r#impl::option_impl::OptionBuilderImpl;
use ::yacl::dev::isxander::yacl3::api::option::AnyOption as YAnyOption;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: `Option<?>` - the members of `public interface Option<T>` that do not mention `T`.
pub trait AnyOption: compat::DynAny {
    /// java: @Nullable Identifier id();
    fn id(&self) -> std::option::Option<Identifier>;

    /// java: Component name();
    fn name(&self) -> MutableComponent;

    /// java: List<Component> description();
    fn description(&self) -> Vec<MutableComponent>;

    /// java: List<Component> tags();
    fn tags(&self) -> Vec<MutableComponent>;

    /// java: boolean modifiable();
    fn modifiable(&self) -> bool;

    /// java: List<OptionFlag> flags();
    fn flags(&self) -> Vec<Arc<dyn OptionFlag>>;

    /// java: `option instanceof ButtonOption button`
    fn as_button_option(self: Arc<Self>) -> std::option::Option<Arc<dyn ButtonOption>> {
        None
    }

    /// java: `option instanceof KeyMappingOption keyMapping`
    fn as_key_mapping_option(self: Arc<Self>) -> std::option::Option<Arc<dyn KeyMappingOption>> {
        None
    }

    /// java: `option instanceof LabelOption label`
    fn as_label_option(self: Arc<Self>) -> std::option::Option<Arc<dyn LabelOption>> {
        None
    }

    /// java: `YACLOptionAdapter.toYaclOption(manager, option)` called with an `Option<?>`: the generic method with this
    /// option's `T` (every implementation forwards to `yacl_option_adapter::to_yacl_option::<T>`).
    fn to_yacl_option(self: Arc<Self>, manager: &Arc<dyn AnyConfigManager>) -> Arc<dyn YAnyOption>;

    /// java: `MoulConfigEditableOptionAdapter.createEditableOptionFactory(option)` called with an `Option<?>`: the generic
    /// method with this option's `T`.
    fn create_editable_option_factory(self: Arc<Self>) -> EditableOptionFactory;
}

/// java: public interface Option<T>
pub trait Option<T: Clone + PartialEq + 'static>: AnyOption {
    /// java: OptionBinding<T> binding();
    fn binding(&self) -> Arc<dyn OptionBinding<T>>;

    /// java: Controller<T> controller();
    fn controller(&self) -> Arc<dyn Controller<T>>;

    /// java: List<OptionListener<T>> listeners();
    fn listeners(&self) -> Vec<Arc<dyn OptionListener<T>>>;

    /// java: Class<T> type();
    fn r#type(&self) -> compat::ClassOf<T>;
}

/// java: static <T> Builder<T> createBuilder()
pub fn create_builder<T: Clone + PartialEq + 'static>() -> OptionBuilderImpl<T> {
    OptionBuilderImpl::new()
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

    /// java: Builder<T> binding(T defaultValue, Supplier<T> getter, Consumer<T> setter);
    fn binding(self, default_value: T, getter: impl Fn() -> T + 'static, setter: impl Fn(T) + 'static) -> Self;

    /// java: Builder<T> controller(Controller<T> controller);
    fn controller(self, controller: Arc<dyn Controller<T>>) -> Self;

    /// java: Builder<T> modifiable(boolean modifiable);
    fn modifiable(self, modifiable: bool) -> Self;

    /// java: Builder<T> flags(OptionFlag... flags);
    fn flags(self, flags: Vec<Arc<dyn OptionFlag>>) -> Self;

    /// java: Builder<T> listener(OptionListener<T> listener);
    fn listener(self, listener: Arc<dyn OptionListener<T>>) -> Self;

    /// java: Option<T> build();
    fn build(self) -> Arc<dyn Option<T>>;
}
