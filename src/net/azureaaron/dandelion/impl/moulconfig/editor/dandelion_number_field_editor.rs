//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionNumberFieldEditor.

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use moul::common::i_minecraft;
use moul::gui::component::text_field_component::TextFieldComponent;
use moul::gui::editors::component_editor::{self, ComponentEditor, ComponentEditorBase};
use moul::gui::gui_component::GuiComponent;
use moul::gui::gui_option_editor::GuiOptionEditor;
use moul::observer::get_setter;
use moul::processor::processed_option::ProcessedOption;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashSet;

/// The two methods a subclass overrides (`toString(Float)`, `parseNumber(..)` - Skyblocker's
/// `DurationController.DurationControllerMoul`); the class's own bodies are `NumberFieldClass::base()`.
pub struct NumberFieldClass {
    /// java: protected String toString(Float floatValue)
    pub to_string: Box<dyn Fn(f32) -> String>,
    /// java: protected float parseNumber(String input, float minValue, float maxValue, float defaultValue)
    pub parse_number: Box<dyn Fn(&str, f32, f32, f32) -> f32>,
}

impl NumberFieldClass {
    /// `DandelionNumberFieldEditor`'s own `toString` / `parseNumber`
    pub fn base() -> NumberFieldClass {
        NumberFieldClass {
            to_string: Box::new(DandelionNumberFieldEditor::to_string),
            parse_number: Box::new(DandelionNumberFieldEditor::parse_number),
        }
    }
}

/// java: public class DandelionNumberFieldEditor extends ComponentEditor
pub struct DandelionNumberFieldEditor {
    base: ComponentEditorBase,
    /// java: private final GuiComponent component;
    component: Arc<dyn GuiComponent>,
    /// java: private String lastText = ""; - needed for reasons similar to the `Item` editor
    last_text: Arc<RefCell<String>>,
}

impl DandelionNumberFieldEditor {
    /// java: public DandelionNumberFieldEditor(ProcessedOption option, float minValue, float maxValue, float defaultValue)
    pub fn new(option: Arc<dyn ProcessedOption>, min_value: f32, max_value: f32, default_value: f32) -> Arc<Self> {
        DandelionNumberFieldEditor::new_with_class(option, min_value, max_value, default_value, NumberFieldClass::base())
    }

    /// The constructor of a subclass that overrides `toString` / `parseNumber` (both are called virtually: the
    /// constructor's `toString((Float) option.get())` and the text field's setter's `parseNumber(..)`).
    pub fn new_with_class(
        option: Arc<dyn ProcessedOption>,
        min_value: f32,
        max_value: f32,
        default_value: f32,
        class: NumberFieldClass,
    ) -> Arc<Self> {
        let class = Arc::new(class);
        let last_text = Arc::new(RefCell::new(String::new()));
        // java: new GetSetter<>() { get() { return lastText; } set(newValue) { lastText = newValue;
        //       option.set(parseNumber(newValue, minValue, maxValue, defaultValue)); } }
        let (get_text, set_text) = (last_text.clone(), last_text.clone());
        let set_option = option.clone();
        let set_class = class.clone();
        let get_setter = get_setter::of(
            move || get_text.borrow().clone(),
            move |new_value: String| {
                *set_text.borrow_mut() = new_value.clone();
                set_option.set(Arc::new((set_class.parse_number)(&new_value, min_value, max_value, default_value)));
            },
        );
        // java: this.lastText = toString((Float) option.get());
        let current = *option.get().downcast_ref::<f32>().expect("ClassCastException: the option's value is not a Float");
        *last_text.borrow_mut() = (class.to_string)(current);
        let component = component_editor::wrap_component(
            &option,
            TextFieldComponent::new(
                get_setter,
                80,
                Arc::new(|| true),
                "",
                i_minecraft::instance().get_default_font_renderer(),
                HashSet::from(['\u{00A7}']),
            ),
        );
        Arc::new(Self { base: ComponentEditorBase::new(option), component, last_text })
    }

    /// java: @ApiStatus.Experimental protected String toString(Float floatValue)
    ///
    /// Converts a `Float` to a string while dropping the decimal if possible. This is done since this is also used for
    /// integer fields.
    pub fn to_string(float_value: f32) -> String {
        // java: floatValue.intValue() - a saturating `(int)` cast
        let int_value = float_value as i32;
        let converted_back_to_float = int_value as f32;
        if converted_back_to_float == float_value {
            int_value.to_string()
        } else {
            // java: floatValue.toString() - `Float.toString`
            mcrust::javacompat::java_float_to_string(float_value)
        }
    }

    /// java: @ApiStatus.Experimental protected float parseNumber(String input, float minValue, float maxValue, float defaultValue)
    ///
    /// Attempts to parse then `input` as a float, if it fails then the `defaultValue` is returned.
    pub fn parse_number(input: &str, min_value: f32, max_value: f32, default_value: f32) -> f32 {
        // java: Float.parseFloat(input) - surrounding whitespace is allowed, a trailing `f`/`d` too
        let trimmed = input.trim_matches(|c: char| c <= ' ');
        let number = trimmed.strip_suffix(['f', 'F', 'd', 'D']).unwrap_or(trimmed);
        match number.parse::<f32>() {
            // java: Math.clamp(Float.parseFloat(input), minValue, maxValue) - throws IllegalArgumentException when
            // min > max
            Ok(value) => {
                assert!(min_value <= max_value, "IllegalArgumentException: {min_value} > {max_value}");
                if value.is_nan() { value } else { value.max(min_value).min(max_value) }
            }
            Err(_) => default_value,
        }
    }
}

impl ComponentEditor for DandelionNumberFieldEditor {
    fn component_editor_base(&self) -> &ComponentEditorBase {
        &self.base
    }

    /// java: @Override public GuiComponent getDelegate()
    fn get_delegate(&self) -> Arc<dyn GuiComponent> {
        self.component.clone()
    }
}

impl GuiOptionEditor for DandelionNumberFieldEditor {
    ::moulconfig::component_editor_overrides!();

    fn get_height(&self) -> i32 {
        component_editor::get_height(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
