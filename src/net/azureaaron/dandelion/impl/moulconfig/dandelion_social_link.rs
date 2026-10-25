//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionSocialLink.
//!
//! Custom class for implementing social links since the built-in one has a bug where clicking on the button does not
//! open the link in the browser.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::platform_links::Link;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::{wrap_component, wrap_identifier};
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use mcrust::net::minecraft::util::util;
use moul::common::my_resource_location::MyResourceLocation;
use moul::common::text::structured_text::StructuredText;
use moul::social::Social;

/// java: public class DandelionSocialLink extends Social
pub struct DandelionSocialLink {
    /// java: private final String link;
    pub link: String,
    /// java: private final List<StructuredText> tooltip;
    pub tooltip: Vec<Arc<dyn StructuredText>>,
    /// java: private final MyResourceLocation icon;
    pub icon: MyResourceLocation,
}

impl DandelionSocialLink {
    /// java: protected DandelionSocialLink(PlatformLinks.Link link)
    pub fn new(link: Arc<dyn Link>) -> Self {
        Self { link: link.link(), tooltip: vec![wrap_component(&link.name())], icon: wrap_identifier(&link.icon()) }
    }
}

impl Social for DandelionSocialLink {
    /// java: @Override public void onClick() - `Util.getPlatform().openUri(this.link)`
    fn on_click(&self) {
        util::get_platform().open_uri(&self.link);
    }

    /// java: @Override public List<StructuredText> getTooltip()
    fn get_tooltip(&self) -> Vec<Arc<dyn StructuredText>> {
        self.tooltip.clone()
    }

    /// java: @Override public MyResourceLocation getIcon()
    fn get_icon(&self) -> MyResourceLocation {
        self.icon.clone()
    }
}
