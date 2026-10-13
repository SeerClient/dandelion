//! Ported from net.azureaaron.dandelion.impl.moulconfig.MoulConfigDefinition.
//!
//! rust: port gap (GAPS_D.md D1) - MoulConfig's `Config` (the superclass), `StructuredText`, `Social`,
//! `MoulConfigPlatform.wrap` and the behaviour enums are not ported; their sites keep the converter's TODOs.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::option::AnyOption;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_social_link::DandelionSocialLink;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use std::cell::RefCell;

/// java: `MoulConfigPlatform.wrap(Component)`
pub fn wrap_component(component: &MutableComponent) -> crate::unresolved::StructuredText {
    let _ = component;
    todo!("fabric2seer: MoulConfigPlatform.wrap(Component) - MoulConfig is not ported (GAPS_D.md D1)")
}

/// java: `MoulConfigPlatform.wrap(Identifier)`
pub fn wrap_identifier(identifier: &Identifier) -> crate::unresolved::MyResourceLocation {
    let _ = identifier;
    todo!("fabric2seer: MoulConfigPlatform.wrap(Identifier) - MoulConfig is not ported (GAPS_D.md D1)")
}

/// java: public class MoulConfigDefinition extends Config
pub struct MoulConfigDefinition {
    /// java: extends io.github.notenoughupdates.moulconfig.Config
    pub config: crate::unresolved::Config,
    /// java: Config.saveRunnables (the superclass's field this constructor adds to)
    pub save_runnables: RefCell<Vec<Arc<dyn Fn()>>>,
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
            config: Default::default(),
            save_runnables: RefCell::new(Vec::new()),
            manager: manager.clone(),
            title,
            platform_links,
            queued_option_flags: RefCell::new(Vec::new()),
        };
        // java: this.saveRunnables.add(((ConfigManagerImpl<?>) manager)::saveAll);
        this.save_runnables.borrow_mut().push(Arc::new(move || {
            manager.as_config_manager_impl().expect("ClassCastException: the manager is not a ConfigManagerImpl").save_all();
        }));
        this
    }

    /// java: @Override public StructuredText getTitle()
    pub fn get_title(&self) -> crate::unresolved::StructuredText {
        wrap_component(&self.title)
    }

    /// java: @Override public List<Social> getSocials()
    pub fn get_socials(&self) -> Vec<crate::unresolved::Social> {
        if let Some(platform_links) = &self.platform_links {
            return platform_links.links().into_iter().map(|link| crate::unresolved::Social::of(Arc::new(DandelionSocialLink::new(link)))).collect();
        }
        Vec::new()
    }

    /// java: @Override public void saveNow()
    pub fn save_now(&self) {
        // java: super.saveNow(); - `Config.saveNow` runs the save runnables
        let runnables = self.save_runnables.borrow().clone();
        for runnable in runnables {
            runnable();
        }
        let flags = self.queued_option_flags.borrow().clone();
        for flag in flags {
            flag.accept(compat::minecraft());
        }
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

    /// java: @Override public boolean shouldAutoFocusSearchbar()
    pub fn should_auto_focus_searchbar(&self) -> bool {
        true
    }

    /// java: @Override public TitleRenderingBehaviour getTitleRenderingBehaviour(ProcessedOption option)
    pub fn get_title_rendering_behaviour(&self, option: &crate::unresolved::ProcessedOption) -> crate::unresolved::TitleRenderingBehaviour {
        let _ = option;
        // TODO(fabric2seer): static field TitleRenderingBehaviour.WIDE_CENTERED_UNDERLINED
        todo!("fabric2seer: TitleRenderingBehaviour.WIDE_CENTERED_UNDERLINED (GAPS_D.md D1)")
    }

    /// java: @Override public DescriptionRendereringBehaviour getDescriptionBehaviour(ProcessedOption option)
    pub fn get_description_behaviour(&self, option: &crate::unresolved::ProcessedOption) -> crate::unresolved::DescriptionRendereringBehaviour {
        let _ = option;
        // TODO(fabric2seer): static field DescriptionRendereringBehaviour.EXPAND_PANEL
        todo!("fabric2seer: DescriptionRendereringBehaviour.EXPAND_PANEL (GAPS_D.md D1)")
    }
}
