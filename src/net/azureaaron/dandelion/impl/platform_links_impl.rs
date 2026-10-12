//! Ported from net.azureaaron.dandelion.impl.PlatformLinksImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::platform_links::Builder;
use crate::net::azureaaron::dandelion::api::platform_links::Link;
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use mcrust::java::net::uri::Uri;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class PlatformLinksImpl implements PlatformLinks
#[derive(Clone)]
pub struct PlatformLinksImpl {
    /// java: private final List<PlatformLinks.Link> links;
    pub links: Vec<compat::Shared<dyn Link>>,
}

impl PlatformLinksImpl {
    /// java: protected PlatformLinksImpl(List<PlatformLinks.Link> links)
    pub fn new(mut links: Vec<compat::Shared<dyn Link>>) -> compat::Shared<Self> {
        let __init_links = links.clone();
        let __this = compat::shared(Self {
            links: __init_links,
        });
        __this
    }

}

impl PlatformLinks for PlatformLinksImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public List<PlatformLinks.Link> links() (dispatch: the body for this class)
    fn links(&self) -> fn(&compat::Shared<dyn PlatformLinks>) -> Vec<compat::Shared<dyn Link>> {
        PlatformLinksImpl::links_dyn
    }

}

impl PlatformLinksImpl {
    /// java: @Override public List<PlatformLinks.Link> links()
    pub fn links_impl(__this: &compat::Shared<Self>) -> Vec<compat::Shared<dyn Link>> {
        return compat::read(__this, |__t| __t.links.clone());
    }

    /// PlatformLinksImpl.links as the dispatch of PlatformLinks.links
    pub fn links_dyn(__this: &compat::Shared<dyn PlatformLinks>) -> Vec<compat::Shared<dyn Link>> {
        Self::links_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class PlatformLinksBuilderImpl implements PlatformLinks.Builder
#[derive(Clone)]
pub struct PlatformLinksBuilderImpl {
    /// java: private final List<PlatformLinks.Link> links = new ArrayList<>();
    pub links: Vec<compat::Shared<dyn Link>>,
}

impl PlatformLinksBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            links: Vec::new(),
        });
        __this
    }

}

impl Builder for PlatformLinksBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Builder link(Component name, Identifier icon, String link) (dispatch: the body for this class)
    fn link(&self) -> fn(&compat::Shared<dyn Builder>, &MutableComponent, &Identifier, &str) -> compat::Shared<dyn Builder> {
        PlatformLinksBuilderImpl::link_dyn
    }

    /// java: @Override public PlatformLinks build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn PlatformLinks> {
        PlatformLinksBuilderImpl::build_dyn
    }

}

impl PlatformLinksBuilderImpl {
    /// java: @Override public Builder link(Component name, Identifier icon, String link)
    pub fn link_impl(__this: &compat::Shared<Self>, name: &MutableComponent, icon: &Identifier, link: &str) -> compat::Shared<dyn Builder> {
        let __h0 = (compat::shared(PlatformLinksLinkImpl::new(name.clone(), icon.clone(), link.to_string())) as compat::Shared<dyn Link>);
        __this.lock().borrow_mut().links.push(__h0);
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// PlatformLinksBuilderImpl.link as the dispatch of Builder.link
    pub fn link_dyn(__this: &compat::Shared<dyn Builder>, __p0: &MutableComponent, __p1: &Identifier, __p2: &str) -> compat::Shared<dyn Builder> {
        Self::link_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0, __p1, __p2)
    }

    /// java: @Override public PlatformLinks build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn PlatformLinks> {
        return (PlatformLinksImpl::new(compat::read(__this, |__t| __t.links.clone())) as compat::Shared<dyn PlatformLinks>);
    }

    /// PlatformLinksBuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn PlatformLinks> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: private record PlatformLinksLinkImpl(Component name, Identifier icon, String link) implements PlatformLinks.Link
#[derive(Clone, Debug)]
pub struct PlatformLinksLinkImpl {
    /// java: Component name
    pub name: MutableComponent,
    /// java: Identifier icon
    pub icon: Identifier,
    /// java: String link
    pub link: String,
}

impl PlatformLinksLinkImpl {
    /// java: the canonical record constructor
    pub fn new(mut name: MutableComponent, mut icon: Identifier, mut link: String) -> Self {
        name;
        icon;
        link;
        Uri::create(&link).unwrap();
        Self { name, icon, link }
    }

    /// java: the record accessor `name()`
    pub fn name(&self) -> MutableComponent {
        self.name.clone()
    }

    /// java: the record accessor `icon()`
    pub fn icon(&self) -> Identifier {
        self.icon.clone()
    }

    /// java: the record accessor `link()`
    pub fn link(&self) -> String {
        self.link.clone()
    }

}

impl Link for PlatformLinksLinkImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}
