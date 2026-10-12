//! Ported from net.azureaaron.dandelion.api.PlatformLinks by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::platform_links_impl::PlatformLinksBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: Identifier GITHUB_ICON = Dandelion.id("textures/gui/github.png");
pub static GITHUB_ICON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::dandelion::id("textures/gui/github.png"));
/// java: Identifier MODRINTH_ICON = Dandelion.id("textures/gui/modrinth.png");
pub static MODRINTH_ICON: compat::MainThread<Identifier> = compat::MainThread::new(|| crate::entry::dandelion::id("textures/gui/modrinth.png"));

/// java: public interface PlatformLinks
pub trait PlatformLinks: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: List<Link> links(); (dispatch: the body for this class)
    fn links(&self) -> fn(&compat::Shared<dyn PlatformLinks>) -> Vec<compat::Shared<dyn Link>>;

}

/// java: static Builder createBuilder()
pub fn create_builder() -> compat::Shared<dyn Builder> {
    return (PlatformLinksBuilderImpl::new() as compat::Shared<dyn Builder>);
}

/// java: interface Builder
pub trait Builder: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Builder link(Component name, Identifier icon, String link); (dispatch: the body for this class)
    fn link(&self) -> fn(&compat::Shared<dyn Builder>, &MutableComponent, &Identifier, &str) -> compat::Shared<dyn Builder>;

    /// java: PlatformLinks build(); (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn PlatformLinks>;

}

/// java: interface Link
pub trait Link: compat::DynAny {
    fn as_any(&self) -> &dyn std::any::Any;
    /// java: Component name(); (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Link>) -> MutableComponent;

    /// java: Identifier icon(); (dispatch: the body for this class)
    fn icon(&self) -> fn(&compat::Shared<dyn Link>) -> Identifier;

    /// java: String link(); (dispatch: the body for this class)
    fn link(&self) -> fn(&compat::Shared<dyn Link>) -> String;

}
