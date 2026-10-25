//! Ported from net.azureaaron.dandelion.impl.moulconfig.MoulConfigOptionEditorAdapter.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::{ControllerKind, NumberControllerKind};
use crate::net::azureaaron::dandelion::api::option::Option;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::editor::dandelion_blocked_option_editor::DandelionBlockedOptionEditor;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::editor::dandelion_colour_editor::DandelionColourEditor;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::editor::dandelion_item_editor::DandelionItemEditor;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::editor::dandelion_number_field_editor::DandelionNumberFieldEditor;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::editor::dandelion_number_slider_editor::DandelionNumberSliderEditor;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use moul::config::Config;
use moul::gui::editors::gui_option_editor_boolean::GuiOptionEditorBoolean;
use moul::gui::editors::gui_option_editor_dropdown::GuiOptionEditorDropdown;
use moul::gui::editors::gui_option_editor_text::GuiOptionEditorText;
use moul::gui::gui_option_editor::GuiOptionEditor;
use moul::processor::processed_option::ProcessedOption;

/// java: public class MoulConfigOptionEditorAdapter
pub struct MoulConfigOptionEditorAdapter {}

impl MoulConfigOptionEditorAdapter {
    /// java: public static <T> GuiOptionEditor createMoulConfigEditor(Option<T> option, ProcessedOption moulConfigOption,
    /// MoulConfigDefinition configDefinition)
    pub fn create_moul_config_editor<T: Clone + PartialEq + 'static>(
        option: Arc<dyn Option<T>>,
        moul_config_option: Arc<dyn ProcessedOption>,
        config_definition: &Arc<MoulConfigDefinition>,
    ) -> Arc<dyn GuiOptionEditor> {
        let controller = option.controller();
        let option_editor: Arc<dyn GuiOptionEditor> = match controller.kind() {
            // java: new GuiOptionEditorBoolean(moulConfigOption, -1, configDefinition)
            ControllerKind::Boolean(_) => {
                let config: Arc<dyn Config> = config_definition.clone();
                GuiOptionEditorBoolean::new(moul_config_option, -1, config)
            }
            ControllerKind::Colour(colour_controller) => DandelionColourEditor::new(moul_config_option, colour_controller.has_alpha()),
            ControllerKind::Enum(enum_controller) => {
                // java: T[] constants = option.type().getEnumConstants();
                //       String[] displayValues = Arrays.stream(constants).map(enumController.formatter()).map(t -> ((Component) t).getString()).toArray(String[]::new);
                let display_values = enum_controller.display_values();
                // java: GuiOptionEditorDropdown editor = new GuiOptionEditorDropdown(moulConfigOption, displayValues, true);
                let editor = GuiOptionEditorDropdown::new(moul_config_option, &display_values, true);
                // java: ((GuiOptionEditorDropdownAccessor) editor).setConstants((Enum<?>[]) constants);
                crate::mixins::gui_option_editor_dropdown_accessor::set_constants(&editor, enum_controller.enum_constants());
                editor
            }
            ControllerKind::Number(NumberControllerKind::Float(float_controller)) if !float_controller.slider() => {
                let default_value: f32 = compat::java_cast(option.binding().default_value());
                DandelionNumberFieldEditor::new(moul_config_option, float_controller.min(), float_controller.max(), default_value)
            }
            ControllerKind::Number(NumberControllerKind::Float(float_controller)) => {
                DandelionNumberSliderEditor::new(moul_config_option, float_controller.min(), float_controller.max(), float_controller.step())
            }
            ControllerKind::Number(NumberControllerKind::Integer(integer_controller)) if !integer_controller.slider() => {
                let default_value: i32 = compat::java_cast(option.binding().default_value());
                DandelionNumberFieldEditor::new(
                    moul_config_option,
                    integer_controller.min() as f32,
                    integer_controller.max() as f32,
                    default_value as f32,
                )
            }
            ControllerKind::Number(NumberControllerKind::Integer(integer_controller)) => DandelionNumberSliderEditor::new(
                moul_config_option,
                integer_controller.min() as f32,
                integer_controller.max() as f32,
                integer_controller.step() as f32,
            ),
            ControllerKind::Item(_) => DandelionItemEditor::new(moul_config_option),
            // java: new GuiOptionEditorText(moulConfigOption, "§")
            ControllerKind::String(_) => GuiOptionEditorText::new(moul_config_option, "\u{00A7}"),
        };
        if option.modifiable() && !config_definition.is_option_patched(&*option) {
            option_editor
        } else {
            DandelionBlockedOptionEditor::new(option_editor)
        }
    }
}
