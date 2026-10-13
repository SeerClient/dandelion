//! Ported from net.azureaaron.dandelion.api.DandelionConfigScreen.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::config_type::ConfigType;
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use crate::net::azureaaron::dandelion::r#impl::dandelion_config_screen_impl::{DandelionConfigScreenBuilderImpl, DandelionConfigScreenImpl};
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface DandelionConfigScreen
pub trait DandelionConfigScreen: compat::DynAny {
    /// java: Screen generateScreen(@Nullable Screen parent, ConfigType configType);
    ///
    /// `parent` is `compat::ScreenRef` (`None` is Java's `null`), as YACL's `generateScreen`.
    fn generate_screen(&self, parent: Option<compat::ScreenRef>, config_type: ConfigType) -> compat::ScreenRef;
}

/// java: static <T> DandelionConfigScreen create(ConfigManager<T> manager, TriFunction<T, T, Builder, Builder> screenBuilder)
///
/// The `TriFunction` is handed `(defaults, config, builder)` and answers the builder (by value, PATTERNS_Y2 P2).
pub fn create<T: 'static>(
    manager: Arc<dyn ConfigManager<T>>,
    screen_builder: impl FnOnce(compat::Shared<T>, compat::Shared<T>, DandelionConfigScreenBuilderImpl) -> DandelionConfigScreenBuilderImpl,
) -> Arc<dyn DandelionConfigScreen> {
    Arc::new(DandelionConfigScreenImpl::new(manager, screen_builder))
}

/// java: interface Builder
pub trait Builder: Sized {
    /// java: Builder title(Component title);
    fn title(self, title: MutableComponent) -> Self;

    /// java: Builder category(ConfigCategory category);
    fn category(self, category: Arc<dyn ConfigCategory>) -> Self;

    /// java: Builder search(String search);
    fn search(self, search: &str) -> Self;

    /// java: Builder platformLinks(PlatformLinks links);
    fn platform_links(self, links: Arc<dyn PlatformLinks>) -> Self;

    /// java: default Builder categoryIf(boolean condition, ConfigCategory category)
    fn category_if(self, condition: bool, category: Arc<dyn ConfigCategory>) -> Self {
        if condition { self.category(category) } else { self }
    }

    /// java: default Builder categories(List<ConfigCategory> categories)
    fn categories(mut self, categories: Vec<Arc<dyn ConfigCategory>>) -> Self {
        for category in categories {
            self = self.category(category);
        }
        self
    }
}
