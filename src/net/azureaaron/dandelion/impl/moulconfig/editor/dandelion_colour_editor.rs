//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionColourEditor (with its inner class
//! `ColourOverlayComponent`).
//!
//! A colour option's value is `java.awt.Color` - the game's `mcrust::java::awt::color::Color`. The component's
//! reference back to its editor is weak (the editor holds the component).

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use mcrust::java::awt::color::Color;
use mcrust::net::minecraft::util::argb;
use moul::chroma_colour::ChromaColour;
use moul::common::my_resource_location::MyResourceLocation;
use moul::gui::component::color_select_component::ColorSelectComponent;
use moul::gui::editors::component_editor::{self, ComponentEditor, ComponentEditorBase};
use moul::gui::gui_component::{self, GuiComponent, GuiComponentBase};
use moul::gui::gui_immediate_context::GuiImmediateContext;
use moul::gui::gui_option_editor::GuiOptionEditor;
use moul::gui::mouse_event::MouseEvent;
use moul::gui_textures::GuiTextures;
use moul::observer::get_setter::GetSetter;
use moul::platform::moul_config_platform::MoulConfigPlatform;
use moul::processor::processed_option::{self, ProcessedOption};
use std::any::Any;
use std::sync::Weak;

/// java: private static final MyResourceLocation ALPHA_BUTTON = MoulConfigPlatform.wrap(Dandelion.id("textures/gui/button_alpha.png"));
pub static ALPHA_BUTTON: LazyLock<MyResourceLocation> =
    LazyLock::new(|| MoulConfigPlatform::wrap_identifier(&crate::entry::dandelion::id("textures/gui/button_alpha.png")));
/// java: private static final MyResourceLocation ALPHA_BUTTON_OVERLAY = MoulConfigPlatform.wrap(Dandelion.id("textures/gui/button_alpha_overlay.png"));
pub static ALPHA_BUTTON_OVERLAY: LazyLock<MyResourceLocation> =
    LazyLock::new(|| MoulConfigPlatform::wrap_identifier(&crate::entry::dandelion::id("textures/gui/button_alpha_overlay.png")));

/// java: `CommonColors.WHITE`
const WHITE: i32 = -1;

/// java: public class DandelionColourEditor extends ComponentEditor
pub struct DandelionColourEditor {
    base: ComponentEditorBase,
    /// java: private final GuiComponent component;
    component: Arc<dyn GuiComponent>,
}

impl DandelionColourEditor {
    /// java: @SuppressWarnings("unchecked") public DandelionColourEditor(ProcessedOption option, boolean hasAlpha)
    pub fn new(option: Arc<dyn ProcessedOption>, has_alpha: bool) -> Arc<Self> {
        Arc::new_cyclic(|this: &Weak<DandelionColourEditor>| {
            // java: this.component = this.wrapComponent(new ColourOverlayComponent(this, (GetSetter<Color>) option.intoProperty(), hasAlpha));
            let component = component_editor::wrap_component(
                &option,
                gui_component::new_component(ColourOverlayComponent {
                    base: GuiComponentBase::new(),
                    editor: this.clone(),
                    get_setter: processed_option::into_property_as::<Color>(&option),
                    has_alpha,
                }),
            );
            Self { base: ComponentEditorBase::new(option.clone()), component }
        })
    }
}

impl ComponentEditor for DandelionColourEditor {
    fn component_editor_base(&self) -> &ComponentEditorBase {
        &self.base
    }

    /// java: @Override public GuiComponent getDelegate()
    fn get_delegate(&self) -> Arc<dyn GuiComponent> {
        self.component.clone()
    }
}

impl GuiOptionEditor for DandelionColourEditor {
    ::moulconfig::component_editor_overrides!();

    fn get_height(&self) -> i32 {
        component_editor::get_height(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// java: protected static class ColourOverlayComponent extends GuiComponent
pub struct ColourOverlayComponent {
    base: GuiComponentBase,
    /// java: private final DandelionColourEditor editor;
    editor: Weak<DandelionColourEditor>,
    /// java: private final GetSetter<Color> getSetter;
    get_setter: Arc<dyn GetSetter<Color>>,
    /// java: private final boolean hasAlpha;
    has_alpha: bool,
}

impl ColourOverlayComponent {
    /// java: private ChromaColour toChromaColour()
    fn to_chroma_colour(&self) -> ChromaColour {
        let colour = self.get_setter.get();
        ChromaColour::from_static_rgb(colour.get_red(), colour.get_green(), colour.get_blue(), colour.get_alpha())
    }

    /// java: private void fromChromaColourString(String colourString)
    fn from_chroma_colour_string(get_setter: &Arc<dyn GetSetter<Color>>, colour_string: &str) {
        // java: @SuppressWarnings("deprecation") Color colour = new Color(ChromaColour.specialToSimpleRGB(colourString), true);
        let colour = Color::from_rgba(ChromaColour::special_to_simple_rgb(colour_string), true);
        get_setter.set(colour);
    }
}

impl GuiComponent for ColourOverlayComponent {
    fn component_base(&self) -> &GuiComponentBase {
        &self.base
    }

    /// java: @Override public int getWidth()
    fn get_width(&self) -> i32 {
        48
    }

    /// java: @Override public int getHeight()
    fn get_height(&self) -> i32 {
        16
    }

    /// java: @Override public void render(GuiImmediateContext context)
    fn render(&self, context: &GuiImmediateContext) {
        let colour = self.get_setter.get().get_rgb();
        let render_context = context.get_render_context();

        if self.has_alpha {
            // Draw alpha button
            render_context.draw_complex_texture(&ALPHA_BUTTON, 0.0, 0.0, context.get_width() as f32, context.get_height() as f32, &mut |block| {
                block.color(WHITE);
            });

            // Draw tinted overlay
            render_context.draw_complex_texture(
                &ALPHA_BUTTON_OVERLAY,
                0.0,
                0.0,
                context.get_width() as f32,
                context.get_height() as f32,
                &mut |block| {
                    block.color(colour);
                },
            );
        } else {
            render_context.draw_complex_texture(
                &GuiTextures::button_white(),
                0.0,
                0.0,
                context.get_width() as f32,
                context.get_height() as f32,
                &mut |block| {
                    block.color(argb::opaque(colour));
                },
            );
        }
    }

    /// java: @SuppressWarnings("deprecation") @Override public boolean mouseEvent(MouseEvent mouseEvent, GuiImmediateContext context)
    fn mouse_event(&self, mouse_event: &MouseEvent, context: &GuiImmediateContext) -> bool {
        if let MouseEvent::Click { mouse_button: 0, mouse_state: true } = *mouse_event {
            if context.is_hovered() {
                // java: new ColorSelectComponent(0, 0, this.toChromaColour().toLegacyString(), this::fromChromaColourString,
                //       this.editor::closeOverlay, this.hasAlpha, true)
                let get_setter = self.get_setter.clone();
                let close_editor = self.editor.clone();
                let component = ColorSelectComponent::new(
                    0,
                    0,
                    &self.to_chroma_colour().to_legacy_string(),
                    Arc::new(move |colour_string: String| ColourOverlayComponent::from_chroma_colour_string(&get_setter, &colour_string)),
                    Arc::new(move || {
                        if let Some(editor) = close_editor.upgrade() {
                            editor.base.close_overlay();
                        }
                    }),
                    self.has_alpha,
                    true,
                );
                //Clamp the Y so that the colour picker can't go off screen
                let scaled_height = context.get_render_context().get_minecraft().get_scaled_height();
                let clamped_y = if context.get_absolute_mouse_y() + component.get_height() > scaled_height {
                    scaled_height - component.get_height()
                } else {
                    context.get_absolute_mouse_y()
                };

                if let Some(editor) = self.editor.upgrade() {
                    editor.base.open_overlay(component, context.get_absolute_mouse_x(), clamped_y);
                }
                return true;
            }
        }

        false
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
