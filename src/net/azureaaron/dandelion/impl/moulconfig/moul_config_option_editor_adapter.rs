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
use crate::unresolved::GuiOptionEditor;

/// java: public class MoulConfigOptionEditorAdapter
pub struct MoulConfigOptionEditorAdapter {}

impl MoulConfigOptionEditorAdapter {
    /// java: public static <T> GuiOptionEditor createMoulConfigEditor(Option<T> option, ProcessedOption moulConfigOption,
    /// MoulConfigDefinition configDefinition)
    pub fn create_moul_config_editor<T: Clone + PartialEq + 'static>(
        option: Arc<dyn Option<T>>,
        moul_config_option: crate::unresolved::ProcessedOption,
        config_definition: &Arc<MoulConfigDefinition>,
    ) -> GuiOptionEditor {
        let controller = option.controller();
        let option_editor: GuiOptionEditor = match controller.kind() {
            // java: new GuiOptionEditorBoolean(moulConfigOption, -1, configDefinition)
            ControllerKind::Boolean(_) => todo!("fabric2seer: `new GuiOptionEditorBoolean(..)` - MoulConfig is not ported (GAPS_D.md D1)"),
            ControllerKind::Colour(colour_controller) => GuiOptionEditor::of(DandelionColourEditor::new(moul_config_option, colour_controller.has_alpha())),
            ControllerKind::Enum(enum_controller) => {
                // java: String[] displayValues = ..; GuiOptionEditorDropdown editor = new GuiOptionEditorDropdown(moulConfigOption,
                // displayValues, true); ((GuiOptionEditorDropdownAccessor) editor).setConstants((Enum<?>[]) constants);
                let _display_values = enum_controller.display_values();
                todo!("fabric2seer: `new GuiOptionEditorDropdown(..)` - MoulConfig is not ported (GAPS_D.md D1)")
            }
            ControllerKind::Number(NumberControllerKind::Float(float_controller)) if !float_controller.slider() => {
                let default_value: f32 = compat::java_cast(option.binding().default_value());
                GuiOptionEditor::of(Arc::new(DandelionNumberFieldEditor::new(moul_config_option, float_controller.min(), float_controller.max(), default_value)))
            }
            ControllerKind::Number(NumberControllerKind::Float(float_controller)) => {
                GuiOptionEditor::of(Arc::new(DandelionNumberSliderEditor::new(moul_config_option, float_controller.min(), float_controller.max(), float_controller.step())))
            }
            ControllerKind::Number(NumberControllerKind::Integer(integer_controller)) if !integer_controller.slider() => {
                let default_value: i32 = compat::java_cast(option.binding().default_value());
                GuiOptionEditor::of(Arc::new(DandelionNumberFieldEditor::new(
                    moul_config_option,
                    integer_controller.min() as f32,
                    integer_controller.max() as f32,
                    default_value as f32,
                )))
            }
            ControllerKind::Number(NumberControllerKind::Integer(integer_controller)) => GuiOptionEditor::of(Arc::new(DandelionNumberSliderEditor::new(
                moul_config_option,
                integer_controller.min() as f32,
                integer_controller.max() as f32,
                integer_controller.step() as f32,
            ))),
            ControllerKind::Item(_) => GuiOptionEditor::of(Arc::new(DandelionItemEditor::new(moul_config_option))),
            // java: new GuiOptionEditorText(moulConfigOption, "§")
            ControllerKind::String(_) => todo!("fabric2seer: `new GuiOptionEditorText(..)` - MoulConfig is not ported (GAPS_D.md D1)"),
        };
        if option.modifiable() && !config_definition.is_option_patched(&*option) {
            option_editor
        } else {
            GuiOptionEditor::of(Arc::new(DandelionBlockedOptionEditor::new(option_editor)))
        }
    }
}
