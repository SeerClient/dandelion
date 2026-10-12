//! Ported from the Fabric mixin net.azureaaron.dandelion.mixins.GuiOptionEditorDropdownAccessor by fabric2seer.
//! @Mixin(io.github.notenoughupdates.moulconfig.gui.editors.GuiOptionEditorDropdown)

#[allow(unused_imports)]
use crate::prelude::*;

/// @Accessor setConstants -> io.github.notenoughupdates.moulconfig.gui.editors.GuiOptionEditorDropdown.constants
// TODO(fabric2seer): `Enum<?>[]` - assign `obj.constants` with its Rust type

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this.
pub fn register() {
    // (nothing to register)
}
