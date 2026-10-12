//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionSocialLink by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::platform_links::Link;
use mcrust::net::minecraft::util::util;

/// java: public class DandelionSocialLink extends Social
pub struct DandelionSocialLink {
    /// java: extends io.github.notenoughupdates.moulconfig.Social
    pub base: crate::unresolved::Social,
    /// java: private final String link;
    pub link: String,
    /// java: private final List<StructuredText> tooltip;
    pub tooltip: Vec<crate::unresolved::StructuredText>,
    /// java: private final MyResourceLocation icon;
    pub icon: crate::unresolved::MyResourceLocation,
}

impl std::ops::Deref for DandelionSocialLink {
    type Target = crate::unresolved::Social;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DandelionSocialLink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DandelionSocialLink {
    /// java: protected DandelionSocialLink(PlatformLinks.Link link)
    pub fn new(mut link: compat::Shared<dyn Link>) -> Self {
        let __init_link = ({ let __f = compat::dispatch(&link, |__t| __t.link()); __f(&link) });
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: this.tooltip = List.of(MoulConfigPlatform.wrap(link.name()));
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: this.icon = MoulConfigPlatform.wrap(link.icon());
        // TODO(fabric2seer): `new Social(..)`: the port has no `Social::new`
        //   java:
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public void onClick()
    pub fn on_click(&self) {
        util::open_uri(&self.link);
    }

    /// java: @Override public List<StructuredText> getTooltip()
    pub fn get_tooltip(&self) -> Vec<crate::unresolved::StructuredText> {
        return self.tooltip.clone();
    }

    /// java: @Override public MyResourceLocation getIcon()
    pub fn get_icon(&self) -> crate::unresolved::MyResourceLocation {
        return self.icon.clone();
    }

}

// NOTE(fabric2seer): getIcon, getTooltip, onClick override(s) of Social - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
