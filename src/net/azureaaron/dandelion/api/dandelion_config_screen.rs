//! Ported from net.azureaaron.dandelion.api.DandelionConfigScreen by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::config_type::ConfigType;
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use crate::net::azureaaron::dandelion::r#impl::dandelion_config_screen_impl::DandelionConfigScreenImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public interface DandelionConfigScreen
pub trait DandelionConfigScreen: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Screen generateScreen(@Nullable Screen parent, ConfigType configType); (dispatch: the body for this class)
    fn generate_screen(&self) -> fn(&compat::Shared<dyn DandelionConfigScreen>, Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, ConfigType) -> mcrust::net::minecraft::client::gui::screens::screen::Screen;

}

/// java: static <T> DandelionConfigScreen create(ConfigManager<T> manager, TriFunction<T, T, Builder, Builder> screenBuilder)
pub fn create<T: 'static>(manager: &compat::Shared<dyn ConfigManager<T>>, screen_builder: &crate::unresolved::TriFunction) -> compat::Shared<dyn DandelionConfigScreen> {
    return (DandelionConfigScreenImpl::new(manager.clone(), screen_builder.clone()) as compat::Shared<dyn DandelionConfigScreen>);
}

/// java: interface Builder
pub trait Builder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder title(Component title); (dispatch: the body for this class)
    fn title(&self) -> fn(&compat::Shared<dyn Builder>, &MutableComponent) -> compat::Shared<dyn Builder>;

    /// java: Builder category(ConfigCategory category); (dispatch: the body for this class)
    fn category(&self) -> fn(&compat::Shared<dyn Builder>, &compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder>;

    /// java: Builder search(String search); (dispatch: the body for this class)
    fn search(&self) -> fn(&compat::Shared<dyn Builder>, &str) -> compat::Shared<dyn Builder>;

    /// java: Builder platformLinks(PlatformLinks links); (dispatch: the body for this class)
    fn platform_links(&self) -> fn(&compat::Shared<dyn Builder>, &compat::Shared<dyn PlatformLinks>) -> compat::Shared<dyn Builder>;

    /// java: default Builder categoryIf(boolean condition, ConfigCategory category) (dispatch: the body for this class)
    fn category_if(&self) -> fn(&compat::Shared<dyn Builder>, bool, &compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::category_if_body
    }

    /// java: default Builder categories(List<ConfigCategory> categories) (dispatch: the body for this class)
    fn categories(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        <dyn Builder>::categories_body
    }

}

impl dyn Builder {
    /// java: default Builder categoryIf(boolean condition, ConfigCategory category)
    pub fn category_if_body(__this: &compat::Shared<dyn Builder>, mut condition: bool, category: &compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        return (if condition { ({ let __f = compat::dispatch(__this, |__t| __t.category()); __f(__this, category) }) } else { __this.clone() });
    }

    /// java: default Builder categories(List<ConfigCategory> categories)
    pub fn categories_body(__this: &compat::Shared<dyn Builder>, categories: &Vec<compat::Shared<dyn ConfigCategory>>) -> compat::Shared<dyn Builder> {
        for category in categories.iter() {
            ({ let __f = compat::dispatch(__this, |__t| __t.category()); __f(__this, category) });
        }
        return __this.clone();
    }

}
