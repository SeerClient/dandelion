//! Ported from net.azureaaron.dandelion.impl.moulconfig.MoulConfigDefinition.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::option::AnyOption;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_social_link::DandelionSocialLink;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use moul::common::my_resource_location::MyResourceLocation;
use moul::common::text::structured_text::StructuredText;
use moul::config::{self, Config, ConfigBase};
use moul::description_renderering_behaviour::DescriptionRendereringBehaviour;
use moul::platform::moul_config_platform::MoulConfigPlatform;
use moul::processor::processed_option::ProcessedOption;
use moul::social::Social;
use moul::title_rendering_behaviour::TitleRenderingBehaviour;
use std::any::Any;
use std::cell::RefCell;

/// java: `MoulConfigPlatform.wrap(Component)`
pub fn wrap_component(component: &MutableComponent) -> Arc<dyn StructuredText> {
    MoulConfigPlatform::wrap_component(component.clone())
}

/// java: `MoulConfigPlatform.wrap(Identifier)`
pub fn wrap_identifier(identifier: &Identifier) -> MyResourceLocation {
    MoulConfigPlatform::wrap_identifier(identifier)
}

/// java: public class MoulConfigDefinition extends Config
pub struct MoulConfigDefinition {
    /// java: extends io.github.notenoughupdates.moulconfig.Config - its `saveRunnables`
    pub config: ConfigBase,
    /// java: private final ConfigManager<?> manager;
    pub manager: Arc<dyn AnyConfigManager>,
    /// java: private final Component title;
    pub title: MutableComponent,
    /// java: private final @Nullable PlatformLinks platformLinks;
    pub platform_links: Option<Arc<dyn PlatformLinks>>,
    /// java: private final Set<OptionFlag> queuedOptionFlags = new HashSet<>(); - a set by identity
    pub queued_option_flags: RefCell<Vec<Arc<dyn OptionFlag>>>,
}

impl MoulConfigDefinition {
    /// java: protected MoulConfigDefinition(ConfigManager<?> manager, Component title, @Nullable PlatformLinks platformLinks)
    pub fn new(manager: Arc<dyn AnyConfigManager>, title: MutableComponent, platform_links: Option<Arc<dyn PlatformLinks>>) -> Self {
        let this = Self {
            config: ConfigBase::default(),
            manager: manager.clone(),
            title,
            platform_links,
            queued_option_flags: RefCell::new(Vec::new()),
        };
        // java: this.saveRunnables.add(((ConfigManagerImpl<?>) manager)::saveAll);
        this.config.save_runnables.borrow_mut().push(Arc::new(move || {
            manager.as_config_manager_impl().expect("ClassCastException: the manager is not a ConfigManagerImpl").save_all();
        }));
        this
    }

    /// java: public void queueOptionFlags(List<OptionFlag> flags)
    ///
    /// Queues the `flags` to run after the config screen is saved. Each unique option flag will only execute once.
    pub fn queue_option_flags(&self, flags: Vec<Arc<dyn OptionFlag>>) {
        let mut queued = self.queued_option_flags.borrow_mut();
        for flag in flags {
            if !queued.iter().any(|q| std::ptr::eq(Arc::as_ptr(q) as *const (), Arc::as_ptr(&flag) as *const ())) {
                queued.push(flag);
            }
        }
    }

    /// java: public boolean isOptionPatched(Option<?> option)
    pub fn is_option_patched(&self, option: &dyn AnyOption) -> bool {
        self.manager
            .as_config_manager_impl()
            .expect("ClassCastException: the manager is not a ConfigManagerImpl")
            .is_option_patched(option.id().as_ref())
    }
}

impl Config for MoulConfigDefinition {
    fn config_base(&self) -> &ConfigBase {
        &self.config
    }

    /// java: @Override public StructuredText getTitle()
    fn get_title(&self) -> Arc<dyn StructuredText> {
        wrap_component(&self.title)
    }

    /// java: @Override public List<Social> getSocials()
    fn get_socials(&self) -> Vec<Arc<dyn Social>> {
        if let Some(platform_links) = &self.platform_links {
            return platform_links.links().into_iter().map(|link| -> Arc<dyn Social> { Arc::new(DandelionSocialLink::new(link)) }).collect();
        }
        Vec::new()
    }

    /// java: @Override public void saveNow()
    fn save_now(&self) {
        // java: super.saveNow();
        config::save_now_base(&self.config);
        let flags = self.queued_option_flags.borrow().clone();
        for flag in flags {
            flag.accept(compat::minecraft());
        }
    }

    /// java: @Override public boolean shouldAutoFocusSearchbar()
    fn should_auto_focus_searchbar(&self) -> bool {
        true
    }

    /// java: @Override public TitleRenderingBehaviour getTitleRenderingBehaviour(ProcessedOption option)
    fn get_title_rendering_behaviour(&self, option: &dyn ProcessedOption) -> TitleRenderingBehaviour {
        let _ = option;
        TitleRenderingBehaviour::WideCenteredUnderlined
    }

    /// java: @Override public DescriptionRendereringBehaviour getDescriptionBehaviour(ProcessedOption option)
    fn get_description_behaviour(&self, option: &dyn ProcessedOption) -> DescriptionRendereringBehaviour {
        let _ = option;
        DescriptionRendereringBehaviour::ExpandPanel
    }
}

impl MoulConfigDefinition {
    /// `instanceof` / casts
    pub fn as_any(&self) -> &dyn Any {
        self
    }
}
