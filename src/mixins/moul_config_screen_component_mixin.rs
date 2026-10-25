//! Ported from the Fabric mixin net.azureaaron.dandelion.mixins.MoulConfigScreenComponentMixin.
//! @Mixin(io.github.notenoughupdates.moulconfig.platform.MoulConfigScreenComponent)
//!
//! ```java
//! @Overwrite @Override public void onClose() {
//!     if (this.guiContext.onBeforeClose() == CloseEventListener.CloseAction.NO_OBJECTIONS_TO_CLOSE)
//!         this.guiContext.requestClose();
//! }
//! ```
//!
//! The target is a class of the MoulConfig library crate (Dandelion shades it in Java, so the mixin only ever sees
//! Dandelion's copy); the crate takes an `@Overwrite` of `onClose` through `ON_CLOSE_OVERWRITE`.

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use moul::gui::close_event_listener::CloseAction;
use moul::gui::gui_context::GuiContext;
use moul::platform::moul_config_screen_component::{self, ON_CLOSE_OVERWRITE};

/// @Overwrite onClose
fn on_close(gui_context: &GuiContext) {
    if gui_context.on_before_close() == CloseAction::NoObjectionsToClose {
        gui_context.request_close();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    *ON_CLOSE_OVERWRITE.write().unwrap_or_else(|p| p.into_inner()) = Some(on_close);
    // `MoulConfigScreenComponent.keyReleased` - the release the game hands only to vanilla screens (see
    // `register_key_released_bridge`); the library's own wiring, applied with the library's first user's mixins.
    moul_config_screen_component::register_key_released_bridge();
}
