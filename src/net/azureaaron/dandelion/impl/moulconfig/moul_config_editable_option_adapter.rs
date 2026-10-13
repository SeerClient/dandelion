//! Ported from net.azureaaron.dandelion.impl.moulconfig.MoulConfigEditableOptionAdapter.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::button_option::{ButtonOption, ScreenConsumer};
use crate::net::azureaaron::dandelion::api::controllers::controller::{ControllerKind, NumberControllerKind};
use crate::net::azureaaron::dandelion::api::key_mapping_option::KeyMappingOption;
use crate::net::azureaaron::dandelion::api::label_option::LabelOption;
use crate::net::azureaaron::dandelion::api::option::{AnyOption, Option};
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_editable_option::{DandelionProcessedEditableOption, EditableOptionClass};
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::editor::dandelion_key_mapping_editor::DandelionKeyMappingEditor;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::editor::dandelion_label_editor::DandelionLabelEditor;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::{wrap_component, MoulConfigDefinition};
use mcrust::net::minecraft::client::key_mapping::KeyMapping;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use std::any::Any;

/// java: `BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>>`
pub type EditableOptionFactory = Arc<dyn Fn(i32, Arc<MoulConfigDefinition>) -> Arc<dyn DandelionProcessedOption>>;

/// java: public class MoulConfigEditableOptionAdapter
pub struct MoulConfigEditableOptionAdapter {}

impl MoulConfigEditableOptionAdapter {
    /// java: public static <T> BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>>
    /// createEditableOptionFactory(Option<T> option)
    pub fn create_editable_option_factory<T: Clone + PartialEq + 'static>(option: Arc<dyn Option<T>>) -> EditableOptionFactory {
        let as_any: Arc<dyn AnyOption> = option.clone();
        // case ButtonOption button
        if let Some(button) = as_any.clone().as_button_option() {
            return MoulConfigEditableOptionAdapter::create_button_option(button);
        }
        // case KeyMappingOption keyMapping
        if let Some(key_mapping) = as_any.clone().as_key_mapping_option() {
            return MoulConfigEditableOptionAdapter::create_key_mapping_option(key_mapping);
        }
        // case LabelOption label
        if let Some(label) = as_any.clone().as_label_option() {
            return MoulConfigEditableOptionAdapter::create_label_option(label);
        }
        // case Option<T> _ when option.controller() instanceof IntegerController
        let is_integer = matches!(option.controller().kind(), ControllerKind::Number(NumberControllerKind::Integer(_)));
        if is_integer {
            return Arc::new(move |accordion_id: i32, config_definition: Arc<MoulConfigDefinition>| {
                let editor_option = option.clone();
                let editor_definition = config_definition.clone();
                let processed: Arc<dyn DandelionProcessedOption> = DandelionProcessedEditableOption::new(
                    option.clone(),
                    accordion_id,
                    config_definition,
                    EditableOptionClass {
                        create_editor: Box::new(move |this: &DandelionProcessedEditableOption<T>| {
                            editor_option.controller().controller_moul_config(editor_option.clone(), this.as_processed_option(), &editor_definition)
                        }),
                        get_type: None,
                        // java: return ((Integer) super.get()).floatValue();
                        get: Some(Box::new(|this: &DandelionProcessedEditableOption<T>| -> Arc<dyn Any> {
                            let value = *this.super_get().downcast_ref::<i32>().expect("ClassCastException: not an Integer");
                            Arc::new(value as f32)
                        })),
                        // java: return super.set(((Float) value).intValue());
                        set: Some(Box::new(|this: &DandelionProcessedEditableOption<T>, value: Arc<dyn Any>| {
                            let value = *value.downcast_ref::<f32>().expect("ClassCastException: not a Float");
                            this.super_set(Arc::new(value as i32))
                        })),
                    },
                );
                processed
            });
        }
        // case Option<T> _
        Arc::new(move |accordion_id: i32, config_definition: Arc<MoulConfigDefinition>| {
            let editor_option = option.clone();
            let editor_definition = config_definition.clone();
            let processed: Arc<dyn DandelionProcessedOption> = DandelionProcessedEditableOption::new(
                option.clone(),
                accordion_id,
                config_definition,
                EditableOptionClass {
                    create_editor: Box::new(move |this: &DandelionProcessedEditableOption<T>| {
                        editor_option.controller().controller_moul_config(editor_option.clone(), this.as_processed_option(), &editor_definition)
                    }),
                    get_type: None,
                    get: None,
                    set: None,
                },
            );
            processed
        })
    }

    /// java: private static BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>> createButtonOption(ButtonOption button)
    fn create_button_option(button: Arc<dyn ButtonOption>) -> EditableOptionFactory {
        Arc::new(move |accordion_id: i32, config_definition: Arc<MoulConfigDefinition>| {
            let get_button = button.clone();
            let editor_button = button.clone();
            let processed: Arc<dyn DandelionProcessedOption> = DandelionProcessedEditableOption::<ScreenConsumer>::new(
                button.clone(),
                accordion_id,
                config_definition,
                EditableOptionClass {
                    // java: ButtonOption instance = (ButtonOption) this.option;
                    //       return new GuiOptionEditorButton(this, -1, MoulConfigPlatform.wrap(instance.prompt()), this.getConfig());
                    create_editor: Box::new(move |this: &DandelionProcessedEditableOption<ScreenConsumer>| {
                        let _ = (this, wrap_component(&editor_button.prompt()));
                        todo!("fabric2seer: `new GuiOptionEditorButton(..)` - MoulConfig is not ported (GAPS_D.md D1)")
                    }),
                    // java: return Runnable.class;
                    get_type: Some(Box::new(|_this: &DandelionProcessedEditableOption<ScreenConsumer>| {
                        crate::unresolved::Type::of(Arc::new(std::any::TypeId::of::<Arc<dyn Fn()>>()))
                    })),
                    // java: return (Runnable) () -> ((ButtonOption) this.option).action().accept(Minecraft.getInstance().gui.screen());
                    get: Some(Box::new(move |_this: &DandelionProcessedEditableOption<ScreenConsumer>| -> Arc<dyn Any> {
                        let button = get_button.clone();
                        let runnable: Arc<dyn Fn()> = Arc::new(move || button.action().accept(compat::ScreenRef::Current));
                        Arc::new(runnable)
                    })),
                    set: None,
                },
            );
            processed
        })
    }

    /// java: private static BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>> createKeyMappingOption(KeyMappingOption keyMapping)
    fn create_key_mapping_option(key_mapping: Arc<dyn KeyMappingOption>) -> EditableOptionFactory {
        Arc::new(move |accordion_id: i32, config_definition: Arc<MoulConfigDefinition>| {
            let editor_key_mapping = key_mapping.clone();
            let processed: Arc<dyn DandelionProcessedOption> = DandelionProcessedEditableOption::<KeyMapping>::new(
                key_mapping.clone(),
                accordion_id,
                config_definition,
                EditableOptionClass {
                    // java: return new DandelionKeyMappingEditor(this, ((KeyMappingOption) this.option).keyMapping());
                    create_editor: Box::new(move |this: &DandelionProcessedEditableOption<KeyMapping>| {
                        crate::unresolved::GuiOptionEditor::of(Arc::new(DandelionKeyMappingEditor::new(this.as_processed_option(), editor_key_mapping.key_mapping())))
                    }),
                    get_type: None,
                    get: None,
                    set: None,
                },
            );
            processed
        })
    }

    /// java: private static BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>> createLabelOption(LabelOption label)
    fn create_label_option(label: Arc<dyn LabelOption>) -> EditableOptionFactory {
        Arc::new(move |accordion_id: i32, config_definition: Arc<MoulConfigDefinition>| {
            let editor_label = label.clone();
            let processed: Arc<dyn DandelionProcessedOption> = DandelionProcessedEditableOption::<MutableComponent>::new(
                label.clone(),
                accordion_id,
                config_definition,
                EditableOptionClass {
                    // java: return new DandelionLabelEditor(this, MoulConfigPlatform.wrap(((LabelOption) this.option).label()));
                    create_editor: Box::new(move |this: &DandelionProcessedEditableOption<MutableComponent>| {
                        crate::unresolved::GuiOptionEditor::of(Arc::new(DandelionLabelEditor::new(this.as_processed_option(), wrap_component(&editor_label.label()))))
                    }),
                    get_type: None,
                    get: None,
                    set: None,
                },
            );
            processed
        })
    }
}
