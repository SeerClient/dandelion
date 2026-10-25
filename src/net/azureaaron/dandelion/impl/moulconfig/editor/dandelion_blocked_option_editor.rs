//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionBlockedOptionEditor.

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use mcrust::net::minecraft::network::chat::component;
use moul::common::i_minecraft;
use moul::common::my_resource_location::MyResourceLocation;
use moul::common::render_context::RenderContext;
use moul::gui::gui_option_editor::{GuiOptionEditor, GuiOptionEditorBase};
use moul::gui::keyboard_event::KeyboardEvent;
use moul::gui::mouse_event::MouseEvent;
use moul::platform::moul_config_platform::MoulConfigPlatform;
use std::any::Any;

/// java: private static final MyResourceLocation BLOCKED_OVERLAY = MoulConfigPlatform.wrap(Dandelion.id("textures/gui/option_blocked.png"));
///
/// Special thanks to nea for this asset.
pub static BLOCKED_OVERLAY: LazyLock<MyResourceLocation> =
    LazyLock::new(|| MoulConfigPlatform::wrap_identifier(&crate::entry::dandelion::id("textures/gui/option_blocked.png")));

/// java: public class DandelionBlockedOptionEditor extends GuiOptionEditor
pub struct DandelionBlockedOptionEditor {
    /// java: extends GuiOptionEditor
    editor: GuiOptionEditorBase,
    /// java: private final GuiOptionEditor base;
    pub base: Arc<dyn GuiOptionEditor>,
}

impl DandelionBlockedOptionEditor {
    /// java: public DandelionBlockedOptionEditor(GuiOptionEditor base)
    pub fn new(base: Arc<dyn GuiOptionEditor>) -> Arc<Self> {
        Arc::new(Self { editor: GuiOptionEditorBase::new(base.get_option()), base })
    }
}

impl GuiOptionEditor for DandelionBlockedOptionEditor {
    fn editor_base(&self) -> &GuiOptionEditorBase {
        &self.editor
    }

    /// java: @Override public void render(RenderContext context, int x, int y, int width)
    fn render(&self, context: &Arc<dyn RenderContext>, x: i32, y: i32, width: i32) {
        self.base.render(context, x, y, width);

        // Darken the original option
        context.draw_colored_rect(x as f32, y as f32, (x + width) as f32, (y + self.get_height()) as f32, 0x80000000u32 as i32);

        // Draw blocked icon
        let icon_width: f32 = self.get_height() as f32 * 96.0 / 64.0;
        context.draw_textured_rect(&BLOCKED_OVERLAY, x as f32, y as f32, icon_width, self.get_height() as f32);

        // Clarifying Text
        let text = MoulConfigPlatform::wrap_component(component::translatable("text.dandelion.option.unavailable"));
        let font = i_minecraft::instance().get_default_font_renderer();
        context.draw_string_scaled_max_width(
            &text,
            &*font,
            (x as f32 + icon_width) as i32,
            (y as f32 + self.get_height() as f32 / 2.0 - font.get_height() as f32 / 2.0) as i32,
            true,
            (width as f32 - icon_width) as i32,
            0xFFFF4444u32 as i32,
        );
    }

    /// java: @Override public boolean mouseInput(int x, int y, int width, int mouseX, int mouseY, MouseEvent mouseEvent)
    fn mouse_input(&self, x: i32, y: i32, width: i32, mouse_x: i32, mouse_y: i32, mouse_event: &MouseEvent) -> bool {
        let _ = (x, y, width, mouse_x, mouse_y, mouse_event);
        false
    }

    /// java: @Override public boolean keyboardInput(KeyboardEvent event)
    fn keyboard_input(&self, event: &KeyboardEvent) -> bool {
        let _ = event;
        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
