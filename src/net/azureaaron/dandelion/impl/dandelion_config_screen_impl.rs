//! Ported from net.azureaaron.dandelion.impl.DandelionConfigScreenImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::{AnyConfigManager, ConfigManager};
use crate::net::azureaaron::dandelion::api::config_type::ConfigType;
use crate::net::azureaaron::dandelion::api::dandelion_config_screen::{self, DandelionConfigScreen};
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_adapter::MoulConfigAdapter;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_screen_adapter::YACLScreenAdapter;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class DandelionConfigScreenImpl<T> implements DandelionConfigScreen
pub struct DandelionConfigScreenImpl<T: 'static> {
    /// java: private final ConfigManager<T> manager;
    pub manager: Arc<dyn ConfigManager<T>>,
    /// java: private final Component title;
    pub title: MutableComponent,
    /// java: private final List<ConfigCategory> categories;
    pub categories: Vec<Arc<dyn ConfigCategory>>,
    /// java: private final String search;
    pub search: String,
    /// java: private final @Nullable PlatformLinks platformLinks;
    pub platform_links: Option<Arc<dyn PlatformLinks>>,
}

impl<T: 'static> DandelionConfigScreenImpl<T> {
    /// java: public DandelionConfigScreenImpl(ConfigManager<T> manager, TriFunction<T, T, Builder, Builder> screenBuilder)
    pub fn new(
        manager: Arc<dyn ConfigManager<T>>,
        screen_builder: impl FnOnce(compat::Shared<T>, compat::Shared<T>, DandelionConfigScreenBuilderImpl) -> DandelionConfigScreenBuilderImpl,
    ) -> Self {
        let builder = DandelionConfigScreenBuilderImpl::new();
        // java: screenBuilder.apply(manager.defaults(), manager.unpatchedInstance(), builder); - the builder is a
        // value here, so the one the function answers is the one read
        let builder = screen_builder(manager.defaults(), manager.unpatched_instance(), builder);
        Self {
            manager,
            title: builder.title,
            categories: builder.categories,
            search: builder.search,
            platform_links: builder.platform_links,
        }
    }

    /// `this.manager` as the `ConfigManager<?>` the backends take.
    fn any_manager(&self) -> Arc<dyn AnyConfigManager> {
        self.manager.clone()
    }
}

impl<T: 'static> DandelionConfigScreen for DandelionConfigScreenImpl<T> {
    /// java: @Override public Screen generateScreen(@Nullable Screen parent, ConfigType configType)
    fn generate_screen(&self, parent: Option<compat::ScreenRef>, config_type: ConfigType) -> compat::ScreenRef {
        // java: Objects.requireNonNull(configType, ..) - never null here
        match config_type {
            ConfigType::Yacl => YACLScreenAdapter::generate_yacl_screen(&self.any_manager(), self.title.clone(), &self.categories, parent),
            ConfigType::MoulConfig => {
                MoulConfigAdapter::new(self.any_manager(), self.title.clone(), self.platform_links.clone()).generate_moul_config_screen(&self.categories, parent, &self.search)
            }
        }
    }
}

/// java: protected static class DandelionConfigScreenBuilderImpl implements DandelionConfigScreen.Builder
pub struct DandelionConfigScreenBuilderImpl {
    /// java: private Component title = Component.empty();
    pub title: MutableComponent,
    /// java: private List<ConfigCategory> categories = new ArrayList<>();
    pub categories: Vec<Arc<dyn ConfigCategory>>,
    /// java: private String search = "";
    pub search: String,
    /// java: private @Nullable PlatformLinks platformLinks = null;
    pub platform_links: Option<Arc<dyn PlatformLinks>>,
}

impl DandelionConfigScreenBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { title: component::empty(), categories: Vec::new(), search: String::new(), platform_links: None }
    }
}

impl dandelion_config_screen::Builder for DandelionConfigScreenBuilderImpl {
    /// java: @Override public Builder title(Component title)
    fn title(mut self, title: MutableComponent) -> Self {
        self.title = title;
        self
    }

    /// java: @Override public Builder category(ConfigCategory category)
    fn category(mut self, category: Arc<dyn ConfigCategory>) -> Self {
        self.categories.push(category);
        self
    }

    /// java: @Override public Builder search(String search)
    fn search(mut self, search: &str) -> Self {
        self.search = search.to_string();
        self
    }

    /// java: @Override public Builder platformLinks(PlatformLinks links)
    fn platform_links(mut self, links: Arc<dyn PlatformLinks>) -> Self {
        self.platform_links = Some(links);
        self
    }
}
