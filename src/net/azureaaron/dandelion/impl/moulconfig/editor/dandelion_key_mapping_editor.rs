//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionKeyMappingEditor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::com::mojang::blaze3d::platform::input_constants::Key;

/// java: public class DandelionKeyMappingEditor extends ComponentEditor
pub struct DandelionKeyMappingEditor {
    /// java: extends io.github.notenoughupdates.moulconfig.gui.editors.ComponentEditor
    pub base: crate::unresolved::ComponentEditor,
    /// java: private final GuiComponent component;
    pub component: Option<crate::unresolved::GuiComponent>,
}

impl std::ops::Deref for DandelionKeyMappingEditor {
    type Target = crate::unresolved::ComponentEditor;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DandelionKeyMappingEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DandelionKeyMappingEditor {
    /// java: public DandelionKeyMappingEditor(ProcessedOption option, KeyMapping keyMapping)
    pub fn new(mut option: crate::unresolved::ProcessedOption, mut key_mapping: mcrust::net::minecraft::client::key_mapping::KeyMapping) -> Self {
        // TODO(fabric2seer): `new ComponentEditor(..)`: the port has no `ComponentEditor::new`
        //   java: super(option);
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): `ComponentEditor.wrapComponent` has no Rust method in the port (looked for `wrap_component` in ComponentEditor and JavaSrc)
        // rust: this.component = Some(this.wrap_component(&KeyMappingComponent::new(key_mapping.clone())));
        this
    }

    /// java: @Override public GuiComponent getDelegate()
    pub fn get_delegate(&self) -> crate::unresolved::GuiComponent {
        return self.component.clone().unwrap();
    }

}

// NOTE(fabric2seer): getDelegate override(s) of ComponentEditor - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: protected static class KeyMappingComponent extends GuiComponent
pub struct KeyMappingComponent {
    /// java: extends io.github.notenoughupdates.moulconfig.gui.GuiComponent
    pub base: crate::unresolved::GuiComponent,
    /// java: private final KeyMapping keyMapping;
    pub key_mapping: mcrust::net::minecraft::client::key_mapping::KeyMapping,
    /// java: private boolean editingKeycode = false;
    pub editing_keycode: bool,
}

impl std::ops::Deref for KeyMappingComponent {
    type Target = crate::unresolved::GuiComponent;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for KeyMappingComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl KeyMappingComponent {
    /// java: protected KeyMappingComponent(KeyMapping keyMapping)
    pub fn new(mut key_mapping: mcrust::net::minecraft::client::key_mapping::KeyMapping) -> Self {
        let __init_key_mapping = key_mapping.clone();
        // TODO(fabric2seer): `new GuiComponent(..)`: the port has no `GuiComponent::new`
        //   java:
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public int getWidth()
    pub fn get_width(&self) -> i32 {
        return 0;
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        return 30;
    }

    /// java: @Override public void render(GuiImmediateContext context)
    pub fn render(&self, context: &crate::unresolved::GuiImmediateContext) {
        // TODO(fabric2seer): `.getRenderContext(..)` on a GuiImmediateContext
        //   java: RenderContext renderContext = context.getRenderContext();
        let mut render_context: crate::unresolved::RenderContext = todo!("fabric2seer: the initialiser of `renderContext` (TODO above)");
        let mut width: i32 = self.get_width();
        let mut height: i32 = self.get_height();
        // TODO(fabric2seer): `.drawTexturedRect(..)` on a RenderContext
        //   java: renderContext.drawTexturedRect(GuiTextures.BUTTON, width / 6 - 24, height - 7 - 14, 48, 16);
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: StructuredText keyName = MoulConfigPlatform.wrap(this.keyMapping.getTranslatedKeyMessage());
        let mut key_name: crate::unresolved::StructuredText = todo!("fabric2seer: the initialiser of `keyName` (TODO above)");
        // TODO(fabric2seer): static call StructuredText.of(..)
        //   java: StructuredText text = this.editingKeycode ? StructuredText.of("> ").append(keyName).append(" <") : keyName;
        let mut text: crate::unresolved::StructuredText = todo!("fabric2seer: the initialiser of `text` (TODO above)");
        // TODO(fabric2seer): `.drawStringCenteredScaledMaxWidth(..)` on a RenderContext
        //   java: renderContext.drawStringCenteredScaledMaxWidth(text, IMinecraft.INSTANCE.getDefaultFontRenderer(), width / 6, height - 7 - 6, false, 38, 0xFF303030);
        let mut reset_x: i32 = (((width.wrapping_div(6)).wrapping_sub(24)).wrapping_add(48)).wrapping_add(3);
        let mut reset_y: i32 = ((height.wrapping_sub(7)).wrapping_sub(14)).wrapping_add(3);
        // TODO(fabric2seer): `.drawTexturedRect(..)` on a RenderContext
        //   java: renderContext.drawTexturedRect(GuiTextures.RESET, resetX, resetY, 10, 11);
        // TODO(fabric2seer): `.getMouseX(..)` on a GuiImmediateContext
        //   java: int mouseX = context.getMouseX();
        let mut mouse_x: i32 = todo!("fabric2seer: the initialiser of `mouseX` (TODO above)");
        // TODO(fabric2seer): `.getMouseY(..)` on a GuiImmediateContext
        //   java: int mouseY = context.getMouseY();
        let mut mouse_y: i32 = todo!("fabric2seer: the initialiser of `mouseY` (TODO above)");
        if mouse_x >= reset_x && mouse_x < reset_x.wrapping_add(10) && mouse_y >= reset_y && mouse_y < reset_y.wrapping_add(11) {
            // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
            //   java: List<StructuredText> tooltip = List.of(MoulConfigPlatform.wrap(Component.translatable("text.dandelion.option.keyMapping.reset")).red());
            let mut tooltip: Vec<crate::unresolved::StructuredText> = todo!("fabric2seer: the initialiser of `tooltip` (TODO above)");
            // TODO(fabric2seer): `.scheduleDrawTooltip(..)` on a RenderContext
            //   java: renderContext.scheduleDrawTooltip(mouseX, mouseY, tooltip);
        }
    }

    /// java: @Override public boolean mouseEvent(MouseEvent mouseEvent, GuiImmediateContext context)
    pub fn mouse_event(&mut self, mouse_event: &crate::unresolved::MouseEvent, context: &crate::unresolved::GuiImmediateContext) -> bool {
        if let Some(click) = mouse_event.as_any().downcast_ref::<crate::unresolved::Click>() {
            // TODO(fabric2seer): the condition: `.getMouseState(..)` on a Click
            //   java: click.getMouseState() && click.getMouseButton() != -1 && this.editingKeycode
            if todo!("fabric2seer: the condition (TODO above)") {
                self.editing_keycode = false;
                // TODO(fabric2seer): `.getMouseButton(..)` on a Click
                //   java: int mouseButton = click.getMouseButton();
                let mut mouse_button: i32 = todo!("fabric2seer: the initialiser of `mouseButton` (TODO above)");
                // TODO(fabric2seer): field `MOUSE` of a value of unknown type
                //   java: this.setKey(InputConstants.Type.MOUSE.getOrCreate(mouseButton));
                return true;
            }
            // TODO(fabric2seer): the condition: `.getMouseState(..)` on a Click
            //   java: click.getMouseState() && click.getMouseButton() == GLFW.GLFW_MOUSE_BUTTON_LEFT
            if todo!("fabric2seer: the condition (TODO above)") {
                let mut height: i32 = self.get_height();
                let mut width: i32 = self.get_height();
                // TODO(fabric2seer): `.getMouseX(..)` on a GuiImmediateContext
                //   java: int mouseX = context.getMouseX();
                let mut mouse_x: i32 = todo!("fabric2seer: the initialiser of `mouseX` (TODO above)");
                // TODO(fabric2seer): `.getMouseY(..)` on a GuiImmediateContext
                //   java: int mouseY = context.getMouseY();
                let mut mouse_y: i32 = todo!("fabric2seer: the initialiser of `mouseY` (TODO above)");
                if mouse_x > (width.wrapping_div(6)).wrapping_sub(24) && mouse_x < (width.wrapping_div(6)).wrapping_add(16) && mouse_y > (height.wrapping_sub(7)).wrapping_sub(14) && mouse_y < (height.wrapping_sub(7)).wrapping_add(2) {
                    self.editing_keycode = true;
                    return true;
                }
                if mouse_x > (((width.wrapping_div(6)).wrapping_sub(24)).wrapping_add(48)).wrapping_sub(3) && mouse_x < ((((width.wrapping_div(6)).wrapping_sub(24)).wrapping_add(48)).wrapping_add(13)).wrapping_sub(5) && mouse_y > ((height.wrapping_sub(7)).wrapping_sub(14)).wrapping_add(3) && mouse_y < (((height.wrapping_sub(7)).wrapping_sub(14)).wrapping_add(3)).wrapping_add(11) {
                    { let __s0 = self.key_mapping.get_default_key(); self.set_key(__s0) };
                    return true;
                }
            }
        }
        return false;
    }

    /// java: @Override public boolean keyboardEvent(KeyboardEvent keyboardEvent, GuiImmediateContext context)
    pub fn keyboard_event(&mut self, keyboard_event: &crate::unresolved::KeyboardEvent, context: &crate::unresolved::GuiImmediateContext) -> bool {
        if let Some(key_pressed) = keyboard_event.as_any().downcast_ref::<crate::unresolved::KeyPressed>() {
            if self.editing_keycode {
                // TODO(fabric2seer): the condition: `.getPressed(..)` on a KeyPressed
                //   java: keyPressed.getPressed(..)
                if todo!("fabric2seer: the condition (TODO above)") {
                    return true;
                }
                self.editing_keycode = false;
                // TODO(fabric2seer): `.getKeycode(..)` on a KeyPressed
                //   java: int keycode = keyPressed.getKeycode();
                let mut keycode: i32 = todo!("fabric2seer: the initialiser of `keycode` (TODO above)");
                // TODO(fabric2seer): the condition: static field KeyboardConstants.INSTANCE
                //   java: keycode == KeyboardConstants.INSTANCE.getEscape() || keycode == 0
                if todo!("fabric2seer: the condition (TODO above)") {
                    // TODO(fabric2seer): static field KeyboardConstants.INSTANCE
                    //   java: keycode = KeyboardConstants.INSTANCE.getNone();
                }
                // TODO(fabric2seer): field `KEYSYM` of a value of unknown type
                //   java: this.setKey(InputConstants.Type.KEYSYM.getOrCreate(keycode));
                return true;
            } else {
                return false;
            }
        }
        return self.editing_keycode;
    }

    /// java: private void setKey(InputConstants.Key key)
    pub fn set_key(&mut self, key: &Key) {
        self.key_mapping.set_key(key.clone());
        // TODO(fabric2seer): static `KeyMapping.resetMapping` has no Rust counterpart in the port
        // rust: KeyMapping::reset_mapping();
    }

}

// NOTE(fabric2seer): getHeight, getWidth, keyboardEvent, mouseEvent, render override(s) of GuiComponent - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
