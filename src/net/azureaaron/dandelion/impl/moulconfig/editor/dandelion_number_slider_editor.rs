//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionNumberSliderEditor.

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use mcrust::java::text::decimal_format::DecimalFormat;
use mcrust::java::text::decimal_format_symbols::{self, Locale};
use moul::common::i_minecraft;
use moul::common::text::structured_text::{self, StructuredText};
use moul::gui::component::align_component::AlignComponent;
use moul::gui::component::row_component::RowComponent;
use moul::gui::component::slider_component::SliderComponent;
use moul::gui::component::text_component::{TextAlignment, TextComponent};
use moul::gui::editors::component_editor::{self, ComponentEditor, ComponentEditorBase};
use moul::gui::gui_component::GuiComponent;
use moul::gui::gui_option_editor::GuiOptionEditor;
use moul::gui::horizontal_align::HorizontalAlign;
use moul::gui::vertical_align::VerticalAlign;
use moul::observer::get_setter::{self, GetSetter};
use moul::processor::processed_option::{self, ProcessedOption};
use std::any::Any;

/// java: private static final NumberFormat FORMATTER = Util.make(NumberFormat.getInstance(Locale.CANADA), nf -> nf.setMaximumFractionDigits(2));
///
/// `NumberFormat.getInstance(Locale.CANADA)` is the locale's number pattern `#,##0.###` (grouping, up to three
/// fraction digits, HALF_EVEN); `setMaximumFractionDigits(2)` makes it `#,##0.##`. en-CA's symbols are the US/ROOT ones
/// (`.` `,` `-`).
pub static FORMATTER: LazyLock<DecimalFormat> =
    LazyLock::new(|| DecimalFormat::new("#,##0.##", decimal_format_symbols::get_instance(Locale::Us)));

/// java: public class DandelionNumberSliderEditor extends ComponentEditor
pub struct DandelionNumberSliderEditor {
    base: ComponentEditorBase,
    /// java: protected final GuiComponent component;
    pub component: Arc<dyn GuiComponent>,
}

impl DandelionNumberSliderEditor {
    /// java: public DandelionNumberSliderEditor(ProcessedOption option, float minValue, float maxValue, float minStep)
    pub fn new(option: Arc<dyn ProcessedOption>, min_value: f32, max_value: f32, min_step: f32) -> Arc<Self> {
        //if (minStep < 0) minStep = 0.01f;
        let text_option = option.clone();
        let horizontal = get_setter::constant(HorizontalAlign::Center);
        let vertical = get_setter::constant(VerticalAlign::Center);
        let component = component_editor::wrap_component(
            &option,
            RowComponent::new(vec![
                AlignComponent::new(
                    TextComponent::new(
                        i_minecraft::instance().get_default_font_renderer(),
                        // java: () -> StructuredText.of(FORMATTER.format(((Float) option.get()).doubleValue()))
                        Arc::new(move || -> Arc<dyn StructuredText> {
                            let value = *text_option.get().downcast_ref::<f32>().expect("ClassCastException: the option's value is not a Float");
                            structured_text::of(&FORMATTER.format(value as f64))
                        }),
                        20,
                        TextAlignment::Center,
                        false,
                        false,
                    ),
                    Arc::new(move || horizontal.get()),
                    Arc::new(move || vertical.get()),
                ) as Arc<dyn GuiComponent>,
                // java: new SliderComponent((GetSetter<Float>) option.intoProperty(), minValue, maxValue, minStep, 40)
                SliderComponent::new(processed_option::into_property_as::<f32>(&option), min_value, max_value, min_step, 40) as Arc<dyn GuiComponent>,
            ]),
        );
        Arc::new(Self { base: ComponentEditorBase::new(option), component })
    }
}

impl ComponentEditor for DandelionNumberSliderEditor {
    fn component_editor_base(&self) -> &ComponentEditorBase {
        &self.base
    }

    /// java: @Override public GuiComponent getDelegate()
    fn get_delegate(&self) -> Arc<dyn GuiComponent> {
        self.component.clone()
    }
}

impl GuiOptionEditor for DandelionNumberSliderEditor {
    ::moulconfig::component_editor_overrides!();

    fn get_height(&self) -> i32 {
        component_editor::get_height(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
