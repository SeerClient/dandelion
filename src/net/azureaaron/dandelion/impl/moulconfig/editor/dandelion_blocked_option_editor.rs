//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionBlockedOptionEditor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
//   java: private static final MyResourceLocation BLOCKED_OVERLAY = MoulConfigPlatform.wrap(Dandelion.id("textures/gui/option_blocked.png"));
/// java: private static final MyResourceLocation BLOCKED_OVERLAY = MoulConfigPlatform.wrap(Dandelion.id("textures/gui/option_blocked.png"));
pub static BLOCKED_OVERLAY: compat::MainThread<crate::unresolved::MyResourceLocation> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `BLOCKED_OVERLAY` - see the TODO above"));

/// java: public class DandelionBlockedOptionEditor extends GuiOptionEditor
pub struct DandelionBlockedOptionEditor {
    /// java: extends io.github.notenoughupdates.moulconfig.gui.GuiOptionEditor
    pub base: crate::unresolved::GuiOptionEditor,
    /// java: private final GuiOptionEditor base;
    pub base: crate::unresolved::GuiOptionEditor,
}

impl std::ops::Deref for DandelionBlockedOptionEditor {
    type Target = crate::unresolved::GuiOptionEditor;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DandelionBlockedOptionEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DandelionBlockedOptionEditor {
    /// java: public DandelionBlockedOptionEditor(GuiOptionEditor base)
    pub fn new(mut base: crate::unresolved::GuiOptionEditor) -> Self {
        let __init_base = base.clone();
        // TODO(fabric2seer): `.getOption(..)` on a GuiOptionEditor
        //   java: super(base.getOption());
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public void render(RenderContext context, int x, int y, int width)
    pub fn render(&mut self, context: &crate::unresolved::RenderContext, mut x: i32, mut y: i32, mut width: i32) {
        // TODO(fabric2seer): `.render(..)` on a GuiOptionEditor
        //   java: this.base.render(context, x, y, width);
        // TODO(fabric2seer): `.drawColoredRect(..)` on a RenderContext
        //   java: context.drawColoredRect(x, y, x + width, y + this.getHeight(), 0x80000000);
        // TODO(fabric2seer): `GuiOptionEditor.getHeight` has no Rust method in the port (looked for `get_height` in GuiOptionEditor and JavaSrc)
        // rust: let mut icon_width: f32 = ((self.get_height() * 96.0f32) / (64 as f32));
        let mut icon_width: f32 = todo!("fabric2seer: the initialiser of `iconWidth` (TODO above)");
        // TODO(fabric2seer): `.drawTexturedRect(..)` on a RenderContext
        //   java: context.drawTexturedRect(BLOCKED_OVERLAY, x, y, iconWidth, this.getHeight());
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: StructuredText text = MoulConfigPlatform.wrap(Component.translatable("text.dandelion.option.unavailable"));
        let mut text: crate::unresolved::StructuredText = todo!("fabric2seer: the initialiser of `text` (TODO above)");
        // TODO(fabric2seer): static field IMinecraft.INSTANCE
        //   java: IFontRenderer font = IMinecraft.INSTANCE.getDefaultFontRenderer();
        let mut font: crate::unresolved::IFontRenderer = todo!("fabric2seer: the initialiser of `font` (TODO above)");
        // TODO(fabric2seer): `.drawStringScaledMaxWidth(..)` on a RenderContext
        //   java: context.drawStringScaledMaxWidth(text, font, (int) (x + iconWidth), (int) (y + this.getHeight() / 2f  - font.getHeight() / 2f), true, (int) (width - iconWidth), 0xFFFF4444);
    }

    /// java: @Override public boolean mouseInput(int x, int y, int width, int mouseX, int mouseY, MouseEvent mouseEvent)
    pub fn mouse_input(&self, mut x: i32, mut y: i32, mut width: i32, mut mouse_x: i32, mut mouse_y: i32, mouse_event: &crate::unresolved::MouseEvent) -> bool {
        return false;
    }

    /// java: @Override public boolean keyboardInput(KeyboardEvent event)
    pub fn keyboard_input(&self, event: &crate::unresolved::KeyboardEvent) -> bool {
        return false;
    }

}

// NOTE(fabric2seer): keyboardInput, mouseInput, render override(s) of GuiOptionEditor - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
