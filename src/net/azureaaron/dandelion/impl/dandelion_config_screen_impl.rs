//! Ported from net.azureaaron.dandelion.impl.DandelionConfigScreenImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::config_type::ConfigType;
use crate::net::azureaaron::dandelion::api::dandelion_config_screen::Builder;
use crate::net::azureaaron::dandelion::api::dandelion_config_screen::DandelionConfigScreen;
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_adapter::MoulConfigAdapter;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_screen_adapter::YACLScreenAdapter;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class DandelionConfigScreenImpl<T> implements DandelionConfigScreen
#[derive(Clone)]
pub struct DandelionConfigScreenImpl<T> {
    /// java: private final ConfigManager<T> manager;
    pub manager: compat::Shared<dyn ConfigManager<T>>,
    /// java: private final Component title;
    pub title: Option<MutableComponent>,
    /// java: private final List<ConfigCategory> categories;
    pub categories: Vec<compat::Shared<dyn ConfigCategory>>,
    /// java: private final String search;
    pub search: String,
    /// java: private final @Nullable PlatformLinks platformLinks;
    pub platform_links: Option<compat::Shared<dyn PlatformLinks>>,
}

impl<T: Clone + 'static> DandelionConfigScreenImpl<T> {
    /// java: public DandelionConfigScreenImpl(ConfigManager<T> manager, TriFunction<T, T, Builder, Builder> screenBuilder)
    pub fn new(mut manager: compat::Shared<dyn ConfigManager<T>>, mut screen_builder: crate::unresolved::TriFunction) -> compat::Shared<Self> {
        let __init_manager = manager.clone();
        let __this = compat::shared(Self {
            manager: __init_manager,
            title: None,
            categories: Default::default(),
            search: Default::default(),
            platform_links: None,
        });
        screen_builder;
        let mut builder: compat::Shared<DandelionConfigScreenBuilderImpl> = DandelionConfigScreenBuilderImpl::new();
        // TODO(fabric2seer): `.apply(..)` on a TriFunction
        //   java: screenBuilder.apply(manager.defaults(), manager.unpatchedInstance(), builder);
        { let __v = Some(compat::read(&builder, |__t| __t.title.clone())); compat::write(&__this, |__t| __t.title = __v); }
        { let __v = compat::read(&builder, |__t| __t.categories.clone()); compat::write(&__this, |__t| __t.categories = __v); }
        { let __v = compat::read(&builder, |__t| __t.search.clone()); compat::write(&__this, |__t| __t.search = __v); }
        { let __v = compat::read(&builder, |__t| __t.platform_links.clone()); compat::write(&__this, |__t| __t.platform_links = __v); }
        __this
    }

}

impl<T: Clone + 'static> DandelionConfigScreen for DandelionConfigScreenImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Screen generateScreen(@Nullable Screen parent, ConfigType configType) (dispatch: the body for this class)
    fn generate_screen(&self) -> fn(&compat::Shared<dyn DandelionConfigScreen>, Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, ConfigType) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        DandelionConfigScreenImpl::generate_screen_dyn
    }

}

impl<T: Clone + 'static> DandelionConfigScreenImpl<T> {
    /// java: @Override public Screen generateScreen(@Nullable Screen parent, ConfigType configType)
    pub fn generate_screen_impl(__this: &compat::Shared<Self>, mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, mut config_type: ConfigType) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        config_type;
        return match config_type {
            ConfigType::Yacl => YACLScreenAdapter::generate_yacl_screen(&compat::read(__this, |__t| __t.manager.clone()), &compat::read(__this, |__t| __t.title.clone()).unwrap(), &compat::read(__this, |__t| __t.categories.clone()), parent.clone()),
            ConfigType::MoulConfig => (MoulConfigAdapter::new(compat::read(__this, |__t| __t.manager.clone()), compat::read(__this, |__t| __t.title.clone()).unwrap(), compat::read(__this, |__t| __t.platform_links.clone()))).generate_moul_config_screen(&compat::read(__this, |__t| __t.categories.clone()), parent.clone(), &compat::read(__this, |__t| __t.search.clone())),
            _ => {
                panic!("UnsupportedOperationException: {}", "The requested backend is unavailable.");
            }
        };
    }

    /// DandelionConfigScreenImpl.generateScreen as the dispatch of DandelionConfigScreen.generateScreen
    pub fn generate_screen_dyn(__this: &compat::Shared<dyn DandelionConfigScreen>, __p0: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, __p1: ConfigType) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        Self::generate_screen_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1)
    }

}

/// java: protected static class DandelionConfigScreenBuilderImpl implements DandelionConfigScreen.Builder
#[derive(Clone)]
pub struct DandelionConfigScreenBuilderImpl {
    /// java: private Component title = Component.empty();
    pub title: MutableComponent,
    /// java: private List<ConfigCategory> categories = new ArrayList<>();
    pub categories: Vec<compat::Shared<dyn ConfigCategory>>,
    /// java: private String search = "";
    pub search: String,
    /// java: private @Nullable PlatformLinks platformLinks = null;
    pub platform_links: Option<compat::Shared<dyn PlatformLinks>>,
}

impl DandelionConfigScreenBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            title: component::empty(),
            categories: Vec::new(),
            search: "".to_string(),
            platform_links: None,
        });
        __this
    }

}

impl Builder for DandelionConfigScreenBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder title(Component title) (dispatch: the body for this class)
    fn title(&self) -> fn(&compat::Shared<dyn Builder>, &MutableComponent) -> compat::Shared<dyn Builder> {
        DandelionConfigScreenBuilderImpl::title_dyn
    }

    /// java: @Override public Builder category(ConfigCategory category) (dispatch: the body for this class)
    fn category(&self) -> fn(&compat::Shared<dyn Builder>, &compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        DandelionConfigScreenBuilderImpl::category_dyn
    }

    /// java: @Override public Builder search(String search) (dispatch: the body for this class)
    fn search(&self) -> fn(&compat::Shared<dyn Builder>, &str) -> compat::Shared<dyn Builder> {
        DandelionConfigScreenBuilderImpl::search_dyn
    }

    /// java: @Override public Builder platformLinks(PlatformLinks links) (dispatch: the body for this class)
    fn platform_links(&self) -> fn(&compat::Shared<dyn Builder>, &compat::Shared<dyn PlatformLinks>) -> compat::Shared<dyn Builder> {
        DandelionConfigScreenBuilderImpl::platform_links_dyn
    }

}

impl DandelionConfigScreenBuilderImpl {
    /// java: @Override public Builder title(Component title)
    pub fn title_impl(__this: &compat::Shared<Self>, title: &MutableComponent) -> compat::Shared<dyn Builder> {
        { let __v = title.clone(); compat::write(__this, |__t| __t.title = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// DandelionConfigScreenBuilderImpl.title as the dispatch of Builder.title
    pub fn title_dyn(__this: &compat::Shared<dyn Builder>, __p0: &MutableComponent) -> compat::Shared<dyn Builder> {
        Self::title_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder category(ConfigCategory category)
    pub fn category_impl(__this: &compat::Shared<Self>, category: &compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        let __h0 = category.clone();
        __this.lock().borrow_mut().categories.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// DandelionConfigScreenBuilderImpl.category as the dispatch of Builder.category
    pub fn category_dyn(__this: &compat::Shared<dyn Builder>, __p0: &compat::Shared<dyn ConfigCategory>) -> compat::Shared<dyn Builder> {
        Self::category_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder search(String search)
    pub fn search_impl(__this: &compat::Shared<Self>, search: &str) -> compat::Shared<dyn Builder> {
        { let __v = search.to_string(); compat::write(__this, |__t| __t.search = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// DandelionConfigScreenBuilderImpl.search as the dispatch of Builder.search
    pub fn search_dyn(__this: &compat::Shared<dyn Builder>, __p0: &str) -> compat::Shared<dyn Builder> {
        Self::search_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public Builder platformLinks(PlatformLinks links)
    pub fn platform_links_impl(__this: &compat::Shared<Self>, links: &compat::Shared<dyn PlatformLinks>) -> compat::Shared<dyn Builder> {
        { let __v = Some(links.clone()); compat::write(__this, |__t| __t.platform_links = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// DandelionConfigScreenBuilderImpl.platformLinks as the dispatch of Builder.platformLinks
    pub fn platform_links_dyn(__this: &compat::Shared<dyn Builder>, __p0: &compat::Shared<dyn PlatformLinks>) -> compat::Shared<dyn Builder> {
        Self::platform_links_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

}
