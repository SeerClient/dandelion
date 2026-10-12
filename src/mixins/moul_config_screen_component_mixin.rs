//! Ported from the Fabric mixin net.azureaaron.dandelion.mixins.MoulConfigScreenComponentMixin by fabric2seer.
//! @Mixin(io.github.notenoughupdates.moulconfig.platform.MoulConfigScreenComponent)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Overwrite onClose
///   java: @Overwrite
///   java: @Override
/// target: io.github.notenoughupdates.moulconfig.platform.MoulConfigScreenComponent#onClose  [class not in JavaSrc]
/// note: class not in JavaSrc: io.github.notenoughupdates.moulconfig.platform.MoulConfigScreenComponent has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
fn on_close(ci: &mut CallbackInfo<'_>) {
    // TODO(fabric2seer): the condition: `this` (the hooked object) is not available in this callback
    //   java: this.guiContext.onBeforeClose() == CloseEventListener.CloseAction.NO_OBJECTIONS_TO_CLOSE
    if todo!("fabric2seer: the condition (TODO above)") {
        // TODO(fabric2seer): `this` (the hooked object) is not available in this callback
        //   java: this.guiContext.requestClose();
    }
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // TODO(fabric2seer): not registered - class not in JavaSrc: io.github.notenoughupdates.moulconfig.platform.MoulConfigScreenComponent has no file in JavaSrc/src - a remapping problem, or a class this Minecraft version does not have
    // report(hook::register_injector(`io.github.notenoughupdates.moulconfig.platform.MoulConfigScreenComponent#onClose`, InjectOptions::at(At::Head).kind(InjectorKind::Overwrite).priority(1000), Arc::new(on_close)));
}
