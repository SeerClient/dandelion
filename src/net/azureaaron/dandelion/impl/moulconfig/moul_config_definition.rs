//! Ported from net.azureaaron.dandelion.impl.moulconfig.MoulConfigDefinition by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use crate::net::azureaaron::dandelion::r#impl::config_manager_impl::ConfigManagerImpl;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_social_link::DandelionSocialLink;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::net::azureaaron::dandelion::api::option::Option as _;

/// java: public class MoulConfigDefinition extends Config
pub struct MoulConfigDefinition {
    /// java: extends io.github.notenoughupdates.moulconfig.Config
    pub base: crate::unresolved::Config,
    /// java: private final ConfigManager<?> manager;
    pub manager: compat::Shared<dyn ConfigManager<compat::Object>>,
    /// java: private final Component title;
    pub title: MutableComponent,
    /// java: private final @Nullable PlatformLinks platformLinks;
    pub platform_links: Option<compat::Shared<dyn PlatformLinks>>,
    /// java: private final Set<OptionFlag> queuedOptionFlags = new HashSet<>();
    pub queued_option_flags: std::collections::HashSet<compat::Shared<dyn OptionFlag>>,
}

impl std::ops::Deref for MoulConfigDefinition {
    type Target = crate::unresolved::Config;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for MoulConfigDefinition {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl MoulConfigDefinition {
    /// java: protected MoulConfigDefinition(ConfigManager<?> manager, Component title, @Nullable PlatformLinks platformLinks)
    pub fn new(mut manager: compat::Shared<dyn ConfigManager<compat::Object>>, mut title: MutableComponent, mut platform_links: Option<compat::Shared<dyn PlatformLinks>>) -> compat::Shared<Self> {
        let __init_manager = manager.clone();
        let __init_title = title.clone();
        let __init_platform_links = platform_links.clone();
        // TODO(fabric2seer): `new Config(..)`: the port has no `Config::new`
        //   java:
        let __this: compat::Shared<Self> = todo!("constructor");
        // TODO(fabric2seer): `.add(..)` on a value of unknown type
        //   java: this.saveRunnables.add(((ConfigManagerImpl<?>) manager)::saveAll);
        __this
    }

    /// java: @Override public StructuredText getTitle()
    pub fn get_title(__this: &compat::Shared<Self>) -> crate::unresolved::StructuredText {
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: return MoulConfigPlatform.wrap(this.title);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public List<Social> getSocials()
    pub fn get_socials(__this: &compat::Shared<Self>) -> Vec<crate::unresolved::Social> {
        if compat::read(__this, |__t| __t.platform_links.clone()).is_some() {
            // TODO(fabric2seer): class literal
            //   java: return this.platformLinks.links().stream()
            //   java: 					.map(DandelionSocialLink::new)
            //   java: 					.map(Social.class::cast)
            //   java: 					.toList();
        }
        return vec![];
    }

    /// java: @Override public void saveNow()
    pub fn save_now(__this: &compat::Shared<Self>) {
        // TODO(fabric2seer): `Config.saveNow` has no Rust method in the port (looked for `save_now` in Config and JavaSrc)
        // rust: __this.lock().borrow_mut().base.save_now();
        for flag in compat::read(__this, |__t| __t.queued_option_flags.clone()) {
            // TODO(fabric2seer): `accept` is not a method of OptionFlag
            //   java: flag.accept(Minecraft.getInstance());
        }
    }

    /// java: public void queueOptionFlags(List<OptionFlag> flags)
    pub fn queue_option_flags(__this: &compat::Shared<Self>, flags: &Vec<compat::Shared<dyn OptionFlag>>) {
        let __h0 = flags.iter().cloned();
        __this.lock().borrow_mut().queued_option_flags.extend(__h0);
    }

    /// java: public boolean isOptionPatched(Option<?> option)
    pub fn is_option_patched(__this: &compat::Shared<Self>, option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>) -> bool {
        return ConfigManagerImpl::is_option_patched(__this.lock().borrow().manager.as_any().downcast_ref::<compat::Shared<ConfigManagerImpl<compat::Object>>>().unwrap(), option.lock().borrow().id());
    }

    /// java: @Override public boolean shouldAutoFocusSearchbar()
    pub fn should_auto_focus_searchbar(__this: &compat::Shared<Self>) -> bool {
        return true;
    }

    /// java: @Override public TitleRenderingBehaviour getTitleRenderingBehaviour(ProcessedOption option)
    pub fn get_title_rendering_behaviour(__this: &compat::Shared<Self>, option: &crate::unresolved::ProcessedOption) -> crate::unresolved::TitleRenderingBehaviour {
        // TODO(fabric2seer): static field TitleRenderingBehaviour.WIDE_CENTERED_UNDERLINED
        //   java: return TitleRenderingBehaviour.WIDE_CENTERED_UNDERLINED;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public DescriptionRendereringBehaviour getDescriptionBehaviour(ProcessedOption option)
    pub fn get_description_behaviour(__this: &compat::Shared<Self>, option: &crate::unresolved::ProcessedOption) -> crate::unresolved::DescriptionRendereringBehaviour {
        // TODO(fabric2seer): static field DescriptionRendereringBehaviour.EXPAND_PANEL
        //   java: return DescriptionRendereringBehaviour.EXPAND_PANEL;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

// NOTE(fabric2seer): getDescriptionBehaviour, getSocials, getTitle, getTitleRenderingBehaviour, saveNow, shouldAutoFocusSearchbar override(s) of Config - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
