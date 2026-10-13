//! Ported from net.azureaaron.dandelion.impl.OptionGroupImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::option::AnyOption;
use crate::net::azureaaron::dandelion::api::option_adder::OptionAdder;
use crate::net::azureaaron::dandelion::api::option_group::{self, OptionGroup};
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_option_group_adapter::YACLOptionGroupAdapter;
use ::yacl::dev::isxander::yacl3::api::option_group::OptionGroup as YOptionGroup;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class OptionGroupImpl implements OptionGroup
pub struct OptionGroupImpl {
    /// java: private final @Nullable Identifier id;
    pub id: Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
    /// java: private final boolean collapsed;
    pub collapsed: bool,
    /// java: private final List<Option<?>> options;
    pub options: Vec<Arc<dyn AnyOption>>,
}

impl OptionGroupImpl {
    /// java: protected OptionGroupImpl(@Nullable Identifier id, Component name, List<Component> description,
    /// List<Component> tags, boolean collapsed, List<Option<?>> options)
    pub fn new(
        id: Option<Identifier>,
        name: MutableComponent,
        description: Vec<MutableComponent>,
        tags: Vec<MutableComponent>,
        collapsed: bool,
        options: Vec<Arc<dyn AnyOption>>,
    ) -> Self {
        Self { id, name, description, tags, collapsed, options }
    }
}

impl OptionGroup for OptionGroupImpl {
    /// java: @Override public @Nullable Identifier id()
    fn id(&self) -> Option<Identifier> {
        self.id.clone()
    }

    /// java: @Override public Component name()
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: @Override public List<Component> description()
    fn description(&self) -> Vec<MutableComponent> {
        self.description.clone()
    }

    /// java: @Override public List<Component> tags()
    fn tags(&self) -> Vec<MutableComponent> {
        self.tags.clone()
    }

    /// java: @Override public boolean collapsed()
    fn collapsed(&self) -> bool {
        self.collapsed
    }

    /// java: @Override public List<Option<?>> options()
    fn options(&self) -> Vec<Arc<dyn AnyOption>> {
        self.options.clone()
    }

    fn to_yacl_option_group(self: Arc<Self>, manager: &Arc<dyn AnyConfigManager>) -> Arc<dyn YOptionGroup> {
        YACLOptionGroupAdapter::to_yacl_plain_option_group(manager, self)
    }
}

/// java: public static class OptionGroupBuilderImpl implements OptionGroup.Builder
pub struct OptionGroupBuilderImpl {
    /// java: private @Nullable Identifier id = null;
    pub id: Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private boolean collapsed = false;
    pub collapsed: bool,
    /// java: private List<Option<?>> options = new ArrayList<>();
    pub options: Vec<Arc<dyn AnyOption>>,
}

impl OptionGroupBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { id: None, name: component::empty(), description: Vec::new(), tags: Vec::new(), collapsed: false, options: Vec::new() }
    }
}

impl OptionAdder for OptionGroupBuilderImpl {
    /// java: @Override public Builder option(Option<?> option)
    fn option(mut self, option: Arc<dyn AnyOption>) -> Self {
        self.options.push(option);
        self
    }
}

impl option_group::Builder for OptionGroupBuilderImpl {
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

    /// java: @Override public Builder description(Component... texts)
    fn description(mut self, texts: Vec<MutableComponent>) -> Self {
        self.description = texts;
        self
    }

    /// java: @Override public Builder tags(Component... tags)
    fn tags(mut self, tags: Vec<MutableComponent>) -> Self {
        self.tags = tags;
        self
    }

    /// java: @Override public Builder collapsed(boolean collapsed)
    fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// java: @Override public OptionGroup build()
    fn build(self) -> Arc<dyn OptionGroup> {
        Arc::new(OptionGroupImpl::new(self.id, self.name, self.description, self.tags, self.collapsed, self.options))
    }
}
