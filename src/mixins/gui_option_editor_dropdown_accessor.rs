//! Ported from the Fabric mixin net.azureaaron.dandelion.mixins.GuiOptionEditorDropdownAccessor.
//! @Mixin(io.github.notenoughupdates.moulconfig.gui.editors.GuiOptionEditorDropdown)
//!
//! `@Accessor void setConstants(Enum<?>[] constants)` writes the dropdown's private `constants` field; the ported
//! class exposes that field's setter (`GuiOptionEditorDropdown::set_constants`), which is what the accessor is. A Java
//! enum constant handed as `Enum<?>` is `JavaEnumConstant` (a `compat::JavaEnum` value as MoulConfig's
//! `EnumConstant`).

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig::gui::editors::gui_option_editor_dropdown::{EnumConstant, GuiOptionEditorDropdown};
use std::any::Any;

/// `java.lang.Enum<?>` - one constant of a `T extends Enum<T>`
pub struct JavaEnumConstant<T: compat::JavaEnum>(pub T);

impl<T: compat::JavaEnum> EnumConstant for JavaEnumConstant<T> {
    /// `Enum.ordinal()`
    fn ordinal(&self) -> i32 {
        self.0.ordinal()
    }

    /// the constant as the option's `Object` value (an option's value is its `T`)
    fn to_value(&self) -> Arc<dyn Any> {
        Arc::new(self.0.clone())
    }

    /// `((Enum) value).ordinal()` for a value of this class
    fn ordinal_of(&self, value: &dyn Any) -> Option<i32> {
        value.downcast_ref::<T>().map(|v| v.ordinal())
    }
}

/// @Accessor setConstants -> GuiOptionEditorDropdown.constants
pub fn set_constants(editor: &GuiOptionEditorDropdown, constants: Vec<Arc<dyn EnumConstant>>) {
    editor.set_constants(constants);
}

/// Mixins apply before the game runs: the `preLaunch` entrypoint calls this. (An `@Accessor` has nothing to register.)
pub fn register() {}
