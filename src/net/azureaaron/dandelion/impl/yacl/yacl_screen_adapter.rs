//! Ported from net.azureaaron.dandelion.impl.yacl.YACLScreenAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use ::yacl::dev::isxander::yacl3::api::yet_another_config_lib;
use ::yacl::dev::isxander::yacl3::api::yet_another_config_lib::Builder;
use ::yacl::dev::isxander::yacl3::api::yet_another_config_lib::YetAnotherConfigLib;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::r#impl::config_manager_impl::ConfigManagerImpl;
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_config_category_adapter::YACLConfigCategoryAdapter;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: public class YACLScreenAdapter
#[derive(Clone)]
pub struct YACLScreenAdapter {}

impl YACLScreenAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static Screen generateYaclScreen(ConfigManager<?> manager, Component title, List<ConfigCategory> categories, @Nullable Screen parent)
    pub fn generate_yacl_screen(manager: &compat::Shared<dyn ConfigManager<compat::Object>>, title: &MutableComponent, categories: &Vec<compat::Shared<dyn ConfigCategory>>, mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        let mut yacl_screen_builder = ({ let __r = &({ let __r = &yet_another_config_lib::create_builder(); let __f = compat::dispatch(__r, |__t| __t.title()); __f(__r, title.clone()) }); let __f = compat::dispatch(__r, |__t| __t.save()); __f(__r, Arc::new(move || { ConfigManagerImpl::save_all(manager.lock().borrow().as_any().downcast_ref::<compat::Shared<ConfigManagerImpl<compat::Object>>>().unwrap()); })) });
        if !categories.is_empty() {
            yacl_screen_builder = ({ let __f = compat::dispatch(&yacl_screen_builder, |__t| __t.categories()); __f(&yacl_screen_builder, &YACLConfigCategoryAdapter::to_yacl_config_categories(manager, categories)) });
        }
        return ({ let __r = &({ let __f = compat::dispatch(&yacl_screen_builder, |__t| __t.build()); __f(&yacl_screen_builder) }); let __f = compat::dispatch(__r, |__t| __t.generate_screen()); __f(__r, parent.clone()) });
    }

}
