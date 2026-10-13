//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionSocialLink.
//!
//! Custom class for implementing social links since the built-in one has a bug where clicking on the button does not
//! open the link in the browser.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::platform_links::Link;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::{wrap_component, wrap_identifier};
use mcrust::net::minecraft::util::util;

/// java: public class DandelionSocialLink extends Social
pub struct DandelionSocialLink {
    /// java: extends io.github.notenoughupdates.moulconfig.Social
    pub social: crate::unresolved::Social,
    /// java: private final String link;
    pub link: String,
    /// java: private final List<StructuredText> tooltip;
    pub tooltip: Vec<crate::unresolved::StructuredText>,
    /// java: private final MyResourceLocation icon;
    pub icon: crate::unresolved::MyResourceLocation,
}

impl DandelionSocialLink {
    /// java: protected DandelionSocialLink(PlatformLinks.Link link)
    pub fn new(link: Arc<dyn Link>) -> Self {
        Self {
            social: Default::default(),
            link: link.link(),
            tooltip: vec![wrap_component(&link.name())],
            icon: wrap_identifier(&link.icon()),
        }
    }

    /// java: @Override public void onClick()
    pub fn on_click(&self) {
        util::get_platform().open_uri(&self.link);
    }

    /// java: @Override public List<StructuredText> getTooltip()
    pub fn get_tooltip(&self) -> Vec<crate::unresolved::StructuredText> {
        self.tooltip.clone()
    }

    /// java: @Override public MyResourceLocation getIcon()
    pub fn get_icon(&self) -> crate::unresolved::MyResourceLocation {
        self.icon.clone()
    }
}
