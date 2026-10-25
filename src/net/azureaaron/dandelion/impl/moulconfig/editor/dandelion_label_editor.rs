//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionLabelEditor.

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use moul::common::i_minecraft;
use moul::common::text::structured_text::StructuredText;
use moul::gui::component::align_component::AlignComponent;
use moul::gui::component::text_component::{TextAlignment, TextComponent};
use moul::gui::editors::component_editor::{ComponentEditor, ComponentEditorBase};
use moul::gui::gui_component::GuiComponent;
use moul::gui::gui_option_editor::GuiOptionEditor;
use moul::gui::horizontal_align::HorizontalAlign;
use moul::gui::vertical_align::VerticalAlign;
use moul::observer::get_setter::{self, GetSetter};
use moul::processor::processed_option::ProcessedOption;
use std::any::Any;

/// java: public class DandelionLabelEditor extends ComponentEditor
pub struct DandelionLabelEditor {
    base: ComponentEditorBase,
    /// java: private final GuiComponent component;
    component: Arc<dyn GuiComponent>,
}

impl DandelionLabelEditor {
    /// java: public DandelionLabelEditor(ProcessedOption option, StructuredText label)
    pub fn new(option: Arc<dyn ProcessedOption>, label: Arc<dyn StructuredText>) -> Arc<Self> {
        let horizontal = get_setter::constant(HorizontalAlign::Center);
        let vertical = get_setter::constant(VerticalAlign::Center);
        let component = AlignComponent::new(
            TextComponent::new(
                i_minecraft::instance().get_default_font_renderer(),
                Arc::new(move || label.clone()),
                250,
                TextAlignment::Center,
                true,
                true,
            ),
            Arc::new(move || horizontal.get()),
            Arc::new(move || vertical.get()),
        );
        Arc::new(Self { base: ComponentEditorBase::new(option), component })
    }
}

impl ComponentEditor for DandelionLabelEditor {
    fn component_editor_base(&self) -> &ComponentEditorBase {
        &self.base
    }

    /// java: @Override public GuiComponent getDelegate()
    fn get_delegate(&self) -> Arc<dyn GuiComponent> {
        self.component.clone()
    }
}

impl GuiOptionEditor for DandelionLabelEditor {
    ::moulconfig::component_editor_overrides!();

    /// java: @Override public int getHeight()
    fn get_height(&self) -> i32 {
        20.max(self.component.get_height())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
