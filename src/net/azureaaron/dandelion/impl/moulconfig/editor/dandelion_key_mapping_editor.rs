//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionKeyMappingEditor (with its inner class
//! `KeyMappingComponent`).

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use mcrust::com::mojang::blaze3d::platform::input_constants::{Key, Type};
use mcrust::net::minecraft::network::chat::component;
use moul::common::i_minecraft;
use moul::common::keyboard_constants;
use moul::common::text::structured_text::{self, StructuredText};
use moul::gui::editors::component_editor::{self, ComponentEditor, ComponentEditorBase};
use moul::gui::gui_component::{self, GuiComponent, GuiComponentBase};
use moul::gui::gui_immediate_context::GuiImmediateContext;
use moul::gui::gui_option_editor::GuiOptionEditor;
use moul::gui::keyboard_event::KeyboardEvent;
use moul::gui::mouse_event::MouseEvent;
use moul::gui_textures::GuiTextures;
use moul::platform::moul_config_platform::MoulConfigPlatform;
use moul::processor::processed_option::ProcessedOption;
use std::any::Any;
use std::cell::{Cell, RefCell};

/// java (26.2): `GLFW.GLFW_MOUSE_BUTTON_LEFT`; 26.3 is on SDL3, whose left button is `InputConstants.MOUSE_BUTTON_LEFT` = 1.
const GLFW_MOUSE_BUTTON_LEFT: i32 = mcrust::com::mojang::blaze3d::platform::input_constants::MOUSE_BUTTON_LEFT;

/// java: public class DandelionKeyMappingEditor extends ComponentEditor
pub struct DandelionKeyMappingEditor {
    base: ComponentEditorBase,
    /// java: private final GuiComponent component;
    component: Arc<dyn GuiComponent>,
}

impl DandelionKeyMappingEditor {
    /// java: public DandelionKeyMappingEditor(ProcessedOption option, KeyMapping keyMapping)
    pub fn new(option: Arc<dyn ProcessedOption>, key_mapping: KeyMapping) -> Arc<Self> {
        let component = component_editor::wrap_component(&option, KeyMappingComponent::new(key_mapping));
        Arc::new(Self { base: ComponentEditorBase::new(option), component })
    }
}

impl ComponentEditor for DandelionKeyMappingEditor {
    fn component_editor_base(&self) -> &ComponentEditorBase {
        &self.base
    }

    /// java: @Override public GuiComponent getDelegate()
    fn get_delegate(&self) -> Arc<dyn GuiComponent> {
        self.component.clone()
    }
}

impl GuiOptionEditor for DandelionKeyMappingEditor {
    ::moulconfig::component_editor_overrides!();

    fn get_height(&self) -> i32 {
        component_editor::get_height(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// java: protected static class KeyMappingComponent extends GuiComponent
pub struct KeyMappingComponent {
    base: GuiComponentBase,
    /// java: private final KeyMapping keyMapping; - the registered mapping is the one in `KeyMapping.ALL` (by name);
    /// this object is the option's copy
    key_mapping: RefCell<KeyMapping>,
    /// java: private boolean editingKeycode = false;
    editing_keycode: Cell<bool>,
}

impl KeyMappingComponent {
    /// java: protected KeyMappingComponent(KeyMapping keyMapping)
    pub fn new(key_mapping: KeyMapping) -> Arc<KeyMappingComponent> {
        gui_component::new_component(KeyMappingComponent {
            base: GuiComponentBase::new(),
            key_mapping: RefCell::new(key_mapping),
            editing_keycode: Cell::new(false),
        })
    }

    /// java: private void setKey(InputConstants.Key key)
    ///
    /// Updates the `keyMapping` to be the `key` and refreshes all key mappings.
    fn set_key(&self, key: Key) {
        // java: this.keyMapping.setKey(key); - the LIVE mapping is the one in `KeyMapping.ALL` (this field is a copy)
        self.key_mapping.borrow_mut().set_key(key.clone());
        key_mapping::set_key(self.key_mapping.borrow().get_name(), key);
        // java: KeyMapping.resetMapping(); - the port derives MAP from ALL, so the new key is indexed already
    }
}

impl GuiComponent for KeyMappingComponent {
    fn component_base(&self) -> &GuiComponentBase {
        &self.base
    }

    /// java: @Override public int getWidth()
    fn get_width(&self) -> i32 {
        0
    }

    /// java: @Override public int getHeight()
    fn get_height(&self) -> i32 {
        30
    }

    /// java: @Override public void render(GuiImmediateContext context)
    fn render(&self, context: &GuiImmediateContext) {
        let render_context = context.get_render_context();
        let width = self.get_width();
        let height = self.get_height();

        render_context.draw_textured_rect(&GuiTextures::button(), (width / 6 - 24) as f32, (height - 7 - 14) as f32, 48.0, 16.0);

        // java: StructuredText keyName = MoulConfigPlatform.wrap(this.keyMapping.getTranslatedKeyMessage());
        // 26.3: the KEYBOARD display supplier is `SDL_GetKeyName(SDL_GetKeyFromScancode(value, 0, false))`
        // (`input_constants::sdl_key_name`); `get_translated_key_message` hands in `(value, -1)`.
        let key_name_of = |key: i32, _scancode: i32| -> Option<String> {
            mcrust::com::mojang::blaze3d::platform::input_constants::sdl_key_name(key)
        };
        let translated = key_mapping::get_translated_key_message(self.key_mapping.borrow().get_name(), &key_name_of);
        let key_name: Arc<dyn StructuredText> = MoulConfigPlatform::wrap_component(component::from_formatted_text(&translated));
        let text: Arc<dyn StructuredText> =
            if self.editing_keycode.get() { structured_text::of("> ").append(&key_name).append_str(" <") } else { key_name };
        render_context.draw_string_centered_scaled_max_width(
            &text,
            &*i_minecraft::instance().get_default_font_renderer(),
            (width / 6) as f32,
            (height - 7 - 6) as f32,
            false,
            38,
            0xFF303030u32 as i32,
        );

        let reset_x = width / 6 - 24 + 48 + 3;
        let reset_y = height - 7 - 14 + 3;
        render_context.draw_textured_rect(&GuiTextures::reset(), reset_x as f32, reset_y as f32, 10.0, 11.0);

        let mouse_x = context.get_mouse_x();
        let mouse_y = context.get_mouse_y();

        if mouse_x >= reset_x && mouse_x < reset_x + 10 && mouse_y >= reset_y && mouse_y < reset_y + 11 {
            let tooltip =
                vec![MoulConfigPlatform::wrap_component(component::translatable("text.dandelion.option.keyMapping.reset")).red()];
            render_context.schedule_draw_tooltip(mouse_x, mouse_y, tooltip);
        }
    }

    /// java: @Override public boolean mouseEvent(MouseEvent mouseEvent, GuiImmediateContext context)
    fn mouse_event(&self, mouse_event: &MouseEvent, context: &GuiImmediateContext) -> bool {
        if let MouseEvent::Click { mouse_button, mouse_state } = *mouse_event {
            if mouse_state && mouse_button != -1 && self.editing_keycode.get() {
                self.editing_keycode.set(false);
                self.set_key(Type::Mouse.get_or_create(mouse_button));
                return true;
            }

            if mouse_state && mouse_button == GLFW_MOUSE_BUTTON_LEFT {
                let height = self.get_height();
                // (Java reads the height here too)
                let width = self.get_height();
                let mouse_x = context.get_mouse_x();
                let mouse_y = context.get_mouse_y();

                if mouse_x > width / 6 - 24 && mouse_x < width / 6 + 16 && mouse_y > height - 7 - 14 && mouse_y < height - 7 + 2 {
                    self.editing_keycode.set(true);
                    return true;
                }

                if mouse_x > width / 6 - 24 + 48 - 3
                    && mouse_x < width / 6 - 24 + 48 + 13 - 5
                    && mouse_y > height - 7 - 14 + 3
                    && mouse_y < height - 7 - 14 + 3 + 11
                {
                    let default_key = self.key_mapping.borrow().get_default_key().clone();
                    self.set_key(default_key);
                    return true;
                }
            }
        }

        false
    }

    /// java: @Override public boolean keyboardEvent(KeyboardEvent keyboardEvent, GuiImmediateContext context)
    fn keyboard_event(&self, keyboard_event: &KeyboardEvent, context: &GuiImmediateContext) -> bool {
        let _ = context;
        if let KeyboardEvent::KeyPressed { keycode, pressed, .. } = *keyboard_event {
            if self.editing_keycode.get() {
                if pressed {
                    return true;
                }

                self.editing_keycode.set(false);
                let mut keycode = keycode;

                if keycode == keyboard_constants::instance().get_escape() || keycode == 0 {
                    keycode = keyboard_constants::instance().get_none();
                }

                self.set_key(Type::Keyboard.get_or_create(keycode));
                return true;
            } else {
                return false;
            }
        }

        self.editing_keycode.get()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
