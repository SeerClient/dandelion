//! Ported from net.azureaaron.dandelion.api.PlatformLinks.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::platform_links_impl::PlatformLinksBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: Identifier GITHUB_ICON = Dandelion.id("textures/gui/github.png");
///
/// Built-in GitHub icon. The texture was made by Bedrock Armor.
#[allow(non_snake_case)]
pub fn GITHUB_ICON() -> Identifier {
    crate::entry::dandelion::id("textures/gui/github.png")
}

/// java: Identifier MODRINTH_ICON = Dandelion.id("textures/gui/modrinth.png");
///
/// Built-in Modrinth icon. The texture was made by Bedrock Armor.
#[allow(non_snake_case)]
pub fn MODRINTH_ICON() -> Identifier {
    crate::entry::dandelion::id("textures/gui/modrinth.png")
}

/// java: public interface PlatformLinks
pub trait PlatformLinks: compat::DynAny {
    /// java: List<Link> links();
    fn links(&self) -> Vec<Arc<dyn Link>>;
}

/// java: static Builder createBuilder()
///
/// Creates a new builder from which platform links can be appended.
pub fn create_builder() -> PlatformLinksBuilderImpl {
    PlatformLinksBuilderImpl::new()
}

/// java: interface Builder
pub trait Builder: Sized {
    /// java: Builder link(Component name, Identifier icon, String link);
    ///
    /// Adds a new link. The provided `link` is checked for validity.
    fn link(self, name: MutableComponent, icon: Identifier, link: &str) -> Self;

    /// java: PlatformLinks build();
    fn build(self) -> Arc<dyn PlatformLinks>;
}

/// java: interface Link
pub trait Link: compat::DynAny {
    /// java: Component name();
    fn name(&self) -> MutableComponent;

    /// java: Identifier icon();
    fn icon(&self) -> Identifier;

    /// java: String link();
    fn link(&self) -> String;
}
