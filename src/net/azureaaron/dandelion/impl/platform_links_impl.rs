//! Ported from net.azureaaron.dandelion.impl.PlatformLinksImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::platform_links::{self, Link, PlatformLinks};
use mcrust::java::net::uri::Uri;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class PlatformLinksImpl implements PlatformLinks
pub struct PlatformLinksImpl {
    /// java: private final List<PlatformLinks.Link> links;
    pub links: Vec<Arc<dyn Link>>,
}

impl PlatformLinksImpl {
    /// java: protected PlatformLinksImpl(List<PlatformLinks.Link> links)
    pub fn new(links: Vec<Arc<dyn Link>>) -> Self {
        Self { links }
    }
}

impl PlatformLinks for PlatformLinksImpl {
    /// java: @Override public List<PlatformLinks.Link> links()
    fn links(&self) -> Vec<Arc<dyn Link>> {
        self.links.clone()
    }
}

/// java: public static class PlatformLinksBuilderImpl implements PlatformLinks.Builder
pub struct PlatformLinksBuilderImpl {
    /// java: private final List<PlatformLinks.Link> links = new ArrayList<>();
    pub links: Vec<Arc<dyn Link>>,
}

impl PlatformLinksBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { links: Vec::new() }
    }
}

impl platform_links::Builder for PlatformLinksBuilderImpl {
    /// java: @Override public Builder link(Component name, Identifier icon, String link)
    fn link(mut self, name: MutableComponent, icon: Identifier, link: &str) -> Self {
        self.links.push(Arc::new(PlatformLinksLinkImpl::new(name, icon, link.to_string())));
        self
    }

    /// java: @Override public PlatformLinks build()
    fn build(self) -> Arc<dyn PlatformLinks> {
        Arc::new(PlatformLinksImpl::new(self.links))
    }
}

/// java: private record PlatformLinksLinkImpl(Component name, Identifier icon, String link) implements PlatformLinks.Link
#[derive(Clone)]
pub struct PlatformLinksLinkImpl {
    /// java: Component name
    pub name: MutableComponent,
    /// java: Identifier icon
    pub icon: Identifier,
    /// java: String link
    pub link: String,
}

impl PlatformLinksLinkImpl {
    /// java: public PlatformLinksLinkImpl { .. } (the compact canonical constructor)
    pub fn new(name: MutableComponent, icon: Identifier, link: String) -> Self {
        // java: Objects.requireNonNull(..) - none of the three can be null here
        // Ensures that the link is valid and can be parsed by Java
        // java: URI.create(link); - throws IllegalArgumentException for an invalid URI
        if let Err(e) = Uri::create(&link) {
            panic!("IllegalArgumentException: {e:?}");
        }
        Self { name, icon, link }
    }
}

impl Link for PlatformLinksLinkImpl {
    /// java: the record accessor `name()`
    fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: the record accessor `icon()`
    fn icon(&self) -> Identifier {
        self.icon.clone()
    }

    /// java: the record accessor `link()`
    fn link(&self) -> String {
        self.link.clone()
    }
}
