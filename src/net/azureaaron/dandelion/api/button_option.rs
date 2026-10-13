//! Ported from net.azureaaron.dandelion.api.ButtonOption.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option::Option;
use crate::net::azureaaron::dandelion::r#impl::button_option_impl::ButtonOptionBuilderImpl;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: `Consumer<Screen>` - a button's action (the option's value type). A struct, not an alias: `ButtonOption`'s
/// own supertrait names this type (PATTERNS_Y2 P6). The screen handed in is the one that is up when the button is
/// pressed (the YACL screen, or `Minecraft.getInstance().gui.screen()` on the MoulConfig side), so it is
/// `compat::ScreenRef::Current` - what a new screen built from it keeps as its parent.
#[derive(Clone)]
pub struct ScreenConsumer(pub Arc<dyn Fn(compat::ScreenRef)>);

impl ScreenConsumer {
    /// `action.accept(screen)`
    pub fn accept(&self, screen: compat::ScreenRef) {
        (self.0)(screen)
    }

    /// `Consumers.nop()`
    pub fn nop() -> ScreenConsumer {
        ScreenConsumer(Arc::new(|_screen: compat::ScreenRef| {}))
    }
}

/// Java compares two lambdas by identity.
impl PartialEq for ScreenConsumer {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(Arc::as_ptr(&self.0) as *const (), Arc::as_ptr(&other.0) as *const ())
    }
}

/// java: public interface ButtonOption extends Option<Consumer<Screen>>
pub trait ButtonOption: Option<ScreenConsumer> {
    /// java: Component prompt();
    fn prompt(&self) -> MutableComponent;

    /// java: Consumer<Screen> action();
    fn action(&self) -> ScreenConsumer;
}

/// java: static Builder createBuilder()
pub fn create_builder() -> ButtonOptionBuilderImpl {
    ButtonOptionBuilderImpl::new()
}

/// java: interface Builder
pub trait Builder: Sized {
    /// java: Builder id(Identifier id);
    fn id(self, id: Identifier) -> Self;

    /// java: Builder name(Component name);
    fn name(self, name: MutableComponent) -> Self;

    /// java: Builder description(Component... texts);
    fn description(self, texts: Vec<MutableComponent>) -> Self;

    /// java: Builder tags(Component... tags);
    fn tags(self, tags: Vec<MutableComponent>) -> Self;

    /// java: Builder prompt(Component prompt);
    fn prompt(self, prompt: MutableComponent) -> Self;

    /// java: Builder action(Consumer<Screen> action);
    fn action(self, action: impl Fn(compat::ScreenRef) + 'static) -> Self;

    /// java: ButtonOption build();
    fn build(self) -> Arc<dyn ButtonOption>;
}
