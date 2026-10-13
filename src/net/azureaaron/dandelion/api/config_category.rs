//! Ported from net.azureaaron.dandelion.api.ConfigCategory.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option_adder::OptionAdder;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::r#impl::config_category_impl::ConfigCategoryBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public interface ConfigCategory
pub trait ConfigCategory: compat::DynAny {
    /// java: Identifier id();
    fn id(&self) -> Identifier;

    /// java: Component name();
    fn name(&self) -> MutableComponent;

    /// java: Component description();
    fn description(&self) -> MutableComponent;

    /// java: @Nullable OptionGroup rootGroup();
    fn root_group(&self) -> Option<Arc<dyn OptionGroup>>;

    /// java: List<OptionGroup> groups();
    fn groups(&self) -> Vec<Arc<dyn OptionGroup>>;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> ConfigCategoryBuilderImpl {
    ConfigCategoryBuilderImpl::new()
}

/// java: non-sealed interface Builder extends OptionAdder
///
/// `option` / `optionIf` / `options` are `OptionAdder`'s (the overrides here only narrow Java's return type).
pub trait Builder: OptionAdder {
    /// java: Builder id(Identifier id);
    fn id(self, id: Identifier) -> Self;

    /// java: Builder name(Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder description(Component description);
    fn description(self, description: MutableComponent) -> Self;

    /// java: Builder group(OptionGroup group);
    fn group(self, group: Arc<dyn OptionGroup>) -> Self;

    /// java: default Builder groupIf(boolean condition, OptionGroup group)
    fn group_if(self, condition: bool, group: Arc<dyn OptionGroup>) -> Self {
        if condition { self.group(group) } else { self }
    }

    /// java: default Builder groups(List<OptionGroup> groups)
    fn groups(mut self, groups: Vec<Arc<dyn OptionGroup>>) -> Self {
        for group in groups {
            self = self.group(group);
        }
        self
    }

    /// java: ConfigCategory build();
    fn build(self) -> Arc<dyn ConfigCategory>;
}
