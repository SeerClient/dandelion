//! Ported from net.azureaaron.dandelion.impl.ConfigCategoryImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::{self, ConfigCategory};
use crate::net::azureaaron::dandelion::api::option::AnyOption;
use crate::net::azureaaron::dandelion::api::option_adder::OptionAdder;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::r#impl::option_group_impl::OptionGroupImpl;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class ConfigCategoryImpl implements ConfigCategory
pub struct ConfigCategoryImpl {
    /// java: private final Identifier id;
    pub id: Identifier,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final Component description;
    pub description: MutableComponent,
    /// java: private final OptionGroup rootGroup;
    pub root_group: Arc<dyn OptionGroup>,
    /// java: private final List<OptionGroup> groups;
    pub groups: Vec<Arc<dyn OptionGroup>>,
}

impl ConfigCategoryImpl {
    /// java: protected ConfigCategoryImpl(Identifier id, Component name, Component description, OptionGroup rootGroup,
    /// List<OptionGroup> groups)
    pub fn new(id: Identifier, name: MutableComponent, description: MutableComponent, root_group: Arc<dyn OptionGroup>, groups: Vec<Arc<dyn OptionGroup>>) -> Self {
        Self { id, name, description, root_group, groups }
    }
}

impl ConfigCategory for ConfigCategoryImpl {
    /// java: @Override public Identifier id()
    fn id(&self) -> Identifier {
        self.id.clone()
    }

    /// java: @Override public Component name()
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: @Override public Component description()
    fn description(&self) -> MutableComponent {
        self.description.clone()
    }

    /// java: @Override public @Nullable OptionGroup rootGroup()
    fn root_group(&self) -> Option<Arc<dyn OptionGroup>> {
        Some(self.root_group.clone())
    }

    /// java: @Override public List<OptionGroup> groups()
    fn groups(&self) -> Vec<Arc<dyn OptionGroup>> {
        self.groups.clone()
    }
}

/// java: public static class ConfigCategoryBuilderImpl implements ConfigCategory.Builder
pub struct ConfigCategoryBuilderImpl {
    /// java: private @Nullable Identifier id = null;
    pub id: Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private Component description = Component.empty();
    pub description: MutableComponent,
    /// java: private List<Option<?>> rootOptions = new ArrayList<>();
    pub root_options: Vec<Arc<dyn AnyOption>>,
    /// java: private List<OptionGroup> groups = new ArrayList<>();
    pub groups: Vec<Arc<dyn OptionGroup>>,
}

impl ConfigCategoryBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { id: None, name: component::empty(), description: component::empty(), root_options: Vec::new(), groups: Vec::new() }
    }
}

impl OptionAdder for ConfigCategoryBuilderImpl {
    /// java: @Override public Builder option(Option<?> option)
    fn option(mut self, option: Arc<dyn AnyOption>) -> Self {
        self.root_options.push(option);
        self
    }
}

impl config_category::Builder for ConfigCategoryBuilderImpl {
    /// java: @Override public Builder id(Identifier id)
    fn id(mut self, id: Identifier) -> Self {
        self.id = Some(id);
        self
    }

    /// java: @Override public Builder name(Component name)
    fn name(mut self, name: MutableComponent) -> Self {
        self.name = name;
        self
    }

    /// java: @Override public Builder description(Component description)
    fn description(mut self, description: MutableComponent) -> Self {
        self.description = description;
        self
    }

    /// java: @Override public Builder group(OptionGroup group)
    fn group(mut self, group: Arc<dyn OptionGroup>) -> Self {
        self.groups.push(group);
        self
    }

    /// java: @Override public ConfigCategory build()
    fn build(self) -> Arc<dyn ConfigCategory> {
        let id = self.id.expect("NullPointerException: an id is required.");
        let root_group: Arc<dyn OptionGroup> = Arc::new(OptionGroupImpl::new(None, component::empty(), Vec::new(), Vec::new(), false, self.root_options));
        Arc::new(ConfigCategoryImpl::new(id, self.name, self.description, root_group, self.groups))
    }
}
