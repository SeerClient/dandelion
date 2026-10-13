//! Ported from net.azureaaron.dandelion.api.OptionGroup.
//!
//! `ListOption<T>` is both an `Option<List<T>>` and an `OptionGroup`; both traits declare `id()` / `name()` /
//! `description()` / `tags()`, so on a list option call them as `OptionGroup::name(&*x)` (PATTERNS_Y2 P1).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::option::AnyOption;
use crate::net::azureaaron::dandelion::api::option_adder::OptionAdder;
use crate::net::azureaaron::dandelion::r#impl::option_group_impl::OptionGroupBuilderImpl;
use ::yacl::dev::isxander::yacl3::api::option_group::OptionGroup as YOptionGroup;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface OptionGroup
pub trait OptionGroup: compat::DynAny {
    /// java: @Nullable Identifier id();
    fn id(&self) -> Option<Identifier>;

    /// java: Component name();
    fn name(&self) -> MutableComponent;

    /// java: List<Component> description();
    fn description(&self) -> Vec<MutableComponent>;

    /// java: List<Component> tags();
    fn tags(&self) -> Vec<MutableComponent>;

    /// java: boolean collapsed();
    fn collapsed(&self) -> bool;

    /// java: List<? extends Option<?>> options();
    fn options(&self) -> Vec<Arc<dyn AnyOption>>;

    /// java: `YACLOptionGroupAdapter.toYaclOptionGroup(manager, group)` - its `switch (group) { case ListOption
    /// listOption -> ..; case OptionGroup optionGroup -> ..; }`: a list option answers the first arm with its own `T`,
    /// every other group the second.
    fn to_yacl_option_group(self: Arc<Self>, manager: &Arc<dyn AnyConfigManager>) -> Arc<dyn YOptionGroup>;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> OptionGroupBuilderImpl {
    OptionGroupBuilderImpl::new()
}

/// java: non-sealed interface Builder extends OptionAdder
///
/// `option` / `optionIf` / `options` are `OptionAdder`'s (the overrides here only narrow Java's return type).
pub trait Builder: OptionAdder {
    /// java: Builder id(Identifier id);
    fn id(self, id: Identifier) -> Self;

    /// java: Builder name(Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder description(Component... texts);
    fn description(self, texts: Vec<MutableComponent>) -> Self;

    /// java: Builder tags(Component... tags);
    fn tags(self, tags: Vec<MutableComponent>) -> Self;

    /// java: Builder collapsed(boolean collapsed);
    fn collapsed(self, collapsed: bool) -> Self;

    /// java: OptionGroup build();
    fn build(self) -> Arc<dyn OptionGroup>;
}
