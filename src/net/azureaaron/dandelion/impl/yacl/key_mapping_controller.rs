//! Ported from net.azureaaron.dandelion.impl.yacl.KeyMappingController.
//!
//! Custom `KeyMapping` controller for YACL (by isXander, Adaptive Tooltips). The widget is a YACL `ControllerWidget`
//! leaf (PATTERNS_Y2 P3): it implements only its class trait, with `awaitingKeyPress` in a `Cell`.

#[allow(unused_imports)]
use crate::prelude::*;
use ::yacl::dev::isxander::yacl3::api::controller::Controller;
use ::yacl::dev::isxander::yacl3::api::option::Option as YOption;
use ::yacl::dev::isxander::yacl3::api::utils::dimension::Dimension;
use ::yacl::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use ::yacl::dev::isxander::yacl3::gui::controllers::controller_widget::{ControllerWidget, ControllerWidgetBase};
use ::yacl::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::com::mojang::blaze3d::platform::input_constants::{DisplayName, Key, Type};
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::input::key_event::KeyEvent;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::locale::language;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::formatted_text::FormattedText;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::cell::Cell;

/// java: `InputConstants.Key.getDisplayName()` - the key's name (GLFW's for a printable key), its translation, or
/// `key.mouse` with the button number.
fn key_display_name(key: &Key) -> MutableComponent {
    let minecraft = compat::minecraft();
    match key.get_display_name(&|code| minecraft.key_names.get(&code).cloned(), &|name| language::has(name)) {
        DisplayName::Literal(text) => component::literal(&text),
        DisplayName::Translatable(key) => component::translatable(&key),
        DisplayName::MouseIndex(index) => component::translatable_args("key.mouse", vec![FormattedText::of(index.to_string())]),
    }
}

/// java: public class KeyMappingController implements Controller<InputConstants.Key>
pub struct KeyMappingController {
    /// java: private final Option<InputConstants.Key> option;
    pub option: Arc<dyn YOption<Key>>,
}

impl KeyMappingController {
    /// java: protected KeyMappingController(Option<InputConstants.Key> option)
    pub fn new(option: Arc<dyn YOption<Key>>) -> Self {
        Self { option }
    }
}

impl Controller<Key> for KeyMappingController {
    /// java: @Override public Option<InputConstants.Key> option()
    fn option(&self) -> Arc<dyn YOption<Key>> {
        self.option.clone()
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        key_display_name(&self.option().state_manager().get())
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(self: Arc<Self>, screen: &Arc<YACLScreen>, widget_dimension: Arc<dyn Dimension<i32>>) -> Arc<dyn AbstractWidget> {
        Arc::new(KeyCodeControllerElement::new(self, screen, widget_dimension))
    }
}

/// java: protected static class KeyCodeControllerElement extends ControllerWidget<KeyMappingController>
pub struct KeyCodeControllerElement {
    /// java: extends ControllerWidget
    pub cw: ControllerWidgetBase,
    /// java: ControllerWidget.control
    pub control: Arc<KeyMappingController>,
    /// java: private boolean awaitingKeyPress = false;
    pub awaiting_key_press: Cell<bool>,
}

impl KeyCodeControllerElement {
    /// java: protected KeyCodeControllerElement(KeyMappingController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(control: Arc<KeyMappingController>, screen: &Arc<YACLScreen>, dim: Arc<dyn Dimension<i32>>) -> Self {
        Self { cw: ControllerWidgetBase::new::<Key, KeyMappingController>(&control, screen, dim), control, awaiting_key_press: Cell::new(false) }
    }
}

impl ControllerWidget for KeyCodeControllerElement {
    fn controller_widget(&self) -> &ControllerWidgetBase {
        &self.cw
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubleClick)
    fn mouse_clicked(&self, event: &MouseButtonEvent, _double_click: bool) -> bool {
        if !ControllerWidget::is_mouse_over(self, event.x(), event.y()) || !ControllerWidget::is_available(self) {
            return false;
        }
        if self.awaiting_key_press.get() {
            self.control.option().request_set(Type::Mouse.get_or_create(event.button()));
        }
        self.awaiting_key_press.set(!self.awaiting_key_press.get());
        true
    }

    /// java: @Override public boolean keyPressed(KeyEvent event)
    fn key_pressed(&self, event: &KeyEvent) -> bool {
        if self.awaiting_key_press.get() {
            self.control.option().request_set(Type::Keyboard.get_or_create(event.key()));
            self.awaiting_key_press.set(false);
            return true;
        }
        false
    }

    /// java: @Override protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        if self.awaiting_key_press.get() {
            return component::translatable("text.dandelion.option.keyMapping.awaitingPress").with_style_format(ChatFormatting::Italic);
        }
        // java: super.getValueText()
        self.cw.control_format_value()
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        ControllerWidget::get_unhovered_control_width(self)
    }
}
