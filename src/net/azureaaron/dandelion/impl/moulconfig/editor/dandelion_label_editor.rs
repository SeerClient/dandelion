//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionLabelEditor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class DandelionLabelEditor extends ComponentEditor
pub struct DandelionLabelEditor {
    /// java: extends io.github.notenoughupdates.moulconfig.gui.editors.ComponentEditor
    pub base: crate::unresolved::ComponentEditor,
    /// java: private final GuiComponent component;
    pub component: crate::unresolved::GuiComponent,
}

impl std::ops::Deref for DandelionLabelEditor {
    type Target = crate::unresolved::ComponentEditor;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DandelionLabelEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DandelionLabelEditor {
    /// java: public DandelionLabelEditor(ProcessedOption option, StructuredText label)
    pub fn new(mut option: crate::unresolved::ProcessedOption, mut label: crate::unresolved::StructuredText) -> Self {
        // TODO(fabric2seer): `new AlignComponent(..)`
        //   java: this.component = new AlignComponent(
        //   java: 			new TextComponent(
        //   java: 				IMinecraft.INSTANCE.getDefaultFontRenderer(),
        //   java: 				() -> label,
        //   java: 				250,
        //   java: 				TextComponent.TextAlignment.CENTER,
        //   java: 				true,
        //   java: 				true
        //   java: 			),
        //   java: 			GetSetter.constant(HorizontalAlign.CENTER),
        //   java: 			GetSetter.constant(VerticalAlign.CENTER)
        //   java: 		);
        // TODO(fabric2seer): `new ComponentEditor(..)`: the port has no `ComponentEditor::new`
        //   java: super(option);
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        // TODO(fabric2seer): `.getHeight(..)` on a GuiComponent
        //   java: return Math.max(20, component.getHeight());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public GuiComponent getDelegate()
    pub fn get_delegate(&self) -> crate::unresolved::GuiComponent {
        return self.component.clone();
    }

}

// NOTE(fabric2seer): getDelegate, getHeight override(s) of ComponentEditor - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
