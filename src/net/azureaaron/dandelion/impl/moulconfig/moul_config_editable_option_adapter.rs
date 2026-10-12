//! Ported from net.azureaaron.dandelion.impl.moulconfig.MoulConfigEditableOptionAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::button_option::ButtonOption;
use crate::net::azureaaron::dandelion::api::key_mapping_option::KeyMappingOption;
use crate::net::azureaaron::dandelion::api::label_option::LabelOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_editable_option::DandelionProcessedEditableOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;

/// java: public class MoulConfigEditableOptionAdapter
#[derive(Clone)]
pub struct MoulConfigEditableOptionAdapter {}

impl MoulConfigEditableOptionAdapter {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static <T> BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>> createEditableOptionFactory(Option<T> option)
    pub fn create_editable_option_factory<T: 'static>(option: &compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>) -> Arc<dyn Fn(i32, compat::Shared<MoulConfigDefinition>) -> compat::Shared<dyn DandelionProcessedEditableOption<compat::Object>>> {
        // TODO(fabric2seer): a pattern switch expression
        //   java: return switch (option) {
        //   java: 			case ButtonOption button -> createButtonOption(button);
        //   java: 			case KeyMappingOption keyMapping -> createKeyMappingOption(keyMapping);
        //   java: 			case LabelOption label -> createLabelOption(label);
        //   java: 			case Option<T> _ when option.controller() instanceof IntegerController -> (accordionId, configDefinition) -> new DandelionProcessedEditableOption<>(option, accordionId, configDefinition) {
        //   java: 				@Override
        //   java: 				public Object get() {
        //   java: 					return ((Integer) super.get()).floatValue();
        //   java: 				}
        //   java:
        //   java: 				@Override
        //   java: 				public boolean set(Object value) {
        //   java: 					return super.set(((Float) value).intValue());
        //   java: 				}
        //   java:
        //   java: 				@Override
        //   java: 				protected GuiOptionEditor createEditor() {
        //   java: 					return option.controller().controllerMoulConfig(option, this, configDefinition);
        //   java: 				}
        //   java: 			};
        //   java: 			case Option<T> _ -> (accordionId, configDefinition) -> new DandelionProcessedEditableOption<>(option, accordionId, configDefinition) {
        //   java: 				@Override
        //   java: 				protected GuiOptionEditor createEditor() {
        //   java: 					return option.controller().controllerMoulConfig(option, this, configDefinition);
        //   java: 				}
        //   java: 			};
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>> createButtonOption(ButtonOption button)
    pub fn create_button_option(button: &compat::Shared<dyn ButtonOption>) -> Arc<dyn Fn(i32, compat::Shared<MoulConfigDefinition>) -> compat::Shared<dyn DandelionProcessedEditableOption<compat::Object>>> {
        // TODO(fabric2seer): `new DandelionProcessedEditableOption(..) {..}` (an anonymous class with state or several methods)
        //   java: return (accordionId, configDefinition) -> new DandelionProcessedEditableOption<>(button, accordionId, configDefinition) {
        //   java: 			@Override
        //   java: 			public Type getType() {
        //   java: 				return Runnable.class;
        //   java: 			}
        //   java:
        //   java: 			@Override
        //   java: 			public Object get() {
        //   java: 				return (Runnable) () -> ((ButtonOption) this.option).action().accept(Minecraft.getInstance().gui.screen());
        //   java: 			}
        //   java:
        //   java: 			@Override
        //   java: 			protected GuiOptionEditor createEditor() {
        //   java: 				ButtonOption instance = (ButtonOption) this.option;
        //   java: 				return new GuiOptionEditorButton(this, -1, MoulConfigPlatform.wrap(instance.prompt()), this.getConfig());
        //   java: 			}
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>> createKeyMappingOption(KeyMappingOption keyMapping)
    pub fn create_key_mapping_option(key_mapping: &compat::Shared<dyn KeyMappingOption>) -> Arc<dyn Fn(i32, compat::Shared<MoulConfigDefinition>) -> compat::Shared<dyn DandelionProcessedEditableOption<compat::Object>>> {
        // TODO(fabric2seer): `new DandelionProcessedEditableOption(..) {..}` (an anonymous class with state or several methods)
        //   java: return (accordionId, configDefinition) -> new DandelionProcessedEditableOption<>(keyMapping, accordionId, configDefinition) {
        //   java: 			@Override
        //   java: 			protected GuiOptionEditor createEditor() {
        //   java: 				return new DandelionKeyMappingEditor(this, ((KeyMappingOption) this.option).keyMapping());
        //   java: 			}
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private static BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>> createLabelOption(LabelOption label)
    pub fn create_label_option(label: &compat::Shared<dyn LabelOption>) -> Arc<dyn Fn(i32, compat::Shared<MoulConfigDefinition>) -> compat::Shared<dyn DandelionProcessedEditableOption<compat::Object>>> {
        // TODO(fabric2seer): `new DandelionProcessedEditableOption(..) {..}` (an anonymous class with state or several methods)
        //   java: return (accordionId, configDefinition) -> new DandelionProcessedEditableOption<>(label, accordionId, configDefinition) {
        //   java: 			@Override
        //   java: 			protected GuiOptionEditor createEditor() {
        //   java: 				return new DandelionLabelEditor(this, MoulConfigPlatform.wrap(((LabelOption) this.option).label()));
        //   java: 			}
        //   java: 		};
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
