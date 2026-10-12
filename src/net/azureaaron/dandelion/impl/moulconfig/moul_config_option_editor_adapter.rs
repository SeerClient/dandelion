//! Ported from net.azureaaron.dandelion.impl.moulconfig.MoulConfigOptionEditorAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::editor::dandelion_blocked_option_editor::DandelionBlockedOptionEditor;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;
use crate::net::azureaaron::dandelion::api::option::Option as _;

/// java: public class MoulConfigOptionEditorAdapter
#[derive(Clone)]
pub struct MoulConfigOptionEditorAdapter {}

impl MoulConfigOptionEditorAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static <T> GuiOptionEditor createMoulConfigEditor(Option<T> option, ProcessedOption moulConfigOption, MoulConfigDefinition configDefinition)
    pub fn create_moul_config_editor<T: 'static>(option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>, moul_config_option: &crate::unresolved::ProcessedOption, config_definition: &compat::Shared<MoulConfigDefinition>) -> crate::unresolved::GuiOptionEditor {
        // TODO(fabric2seer): a pattern switch expression
        //   java: GuiOptionEditor optionEditor = switch (option.controller()) {
        //   java: 			case BooleanController _ -> new GuiOptionEditorBoolean(moulConfigOption, -1, configDefinition);
        //   java: 			case ColourController colourController -> new DandelionColourEditor(moulConfigOption, colourController.hasAlpha());
        //   java: 			case @SuppressWarnings("rawtypes") EnumController enumController -> {
        //   java: 				T[] constants = option.type().getEnumConstants();
        //   java: 				@SuppressWarnings("unchecked")
        //   java: 				String[] displayValues = (String[]) Arrays.stream(constants)
        //   java: 					.map(enumController.formatter())
        //   java: 					.map(t -> ((Component) t).getString())
        //   java: 					.toArray(String[]::new);
        //   java: 				GuiOptionEditorDropdown editor = new GuiOptionEditorDropdown(moulConfigOption, displayValues, true);
        //   java: 				((GuiOptionEditorDropdownAccessor) editor).setConstants((Enum<?>[]) constants);
        //   java: 				yield editor;
        //   java: 			}
        //   java: 			case FloatController floatController when !floatController.slider() -> new DandelionNumberFieldEditor(moulConfigOption, floatController.min(), floatController.max(), (Float) option.binding().defaultValue());
        //   java: 			case FloatController floatController when floatController.slider() -> new DandelionNumberSliderEditor(moulConfigOption, floatController.min(), floatController.max(), floatController.step());
        //   java: 			case IntegerController integerController when !integerController.slider() -> new DandelionNumberFieldEditor(moulConfigOption, integerController.min(), integerController.max(), (Integer) option.binding().defaultValue());
        //   java: 			case IntegerController integerController when integerController.slider() -> new DandelionNumberSliderEditor(moulConfigOption, integerController.min(), integerController.max(), integerController.step());
        //   java: 			case ItemController _ -> new DandelionItemEditor(moulConfigOption);
        //   java: 			case StringController _ -> new GuiOptionEditorText(moulConfigOption, "§");
        //   java: 			default -> throw new UnsupportedOperationException(String.format("The controller %s is not supported by the MoulConfig backend.", option.controller().getClass().getName()));
        //   java: 		};
        let mut option_editor: crate::unresolved::GuiOptionEditor = todo!("fabric2seer: the initialiser of `optionEditor` (TODO above)");
        if option.lock().borrow().modifiable() && !(MoulConfigDefinition::is_option_patched(config_definition, option)) {
            return option_editor;
        } else {
            return DandelionBlockedOptionEditor::new(option_editor.clone());
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
