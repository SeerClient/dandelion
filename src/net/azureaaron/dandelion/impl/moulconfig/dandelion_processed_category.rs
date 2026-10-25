//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedCategory.
//!
//! Represents a category in the config which holds a list of options which themselves can be editable options under
//! the root or group/accordion options that hold editable options.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::wrap_component;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use moul::common::text::structured_text::StructuredText;
use moul::gui::editors::gui_option_editor_accordion::GuiOptionEditorAccordion;
use moul::processor::has_debug_location::HasDebugLocation;
use moul::processor::processed_category::ProcessedCategory;
use moul::processor::processed_option::ProcessedOption;
use std::any::Any;
use std::collections::HashMap;

/// java: public class DandelionProcessedCategory implements ProcessedCategory
pub struct DandelionProcessedCategory {
    /// java: private final ConfigCategory category; - the `ConfigCategory` that this category is bound to
    pub category: Arc<dyn ConfigCategory>,
    /// java: private final List<DandelionProcessedOption> options; - the complete list of options this category holds,
    /// including editable ones under the root, group/accordion ones, and options under groups/accordions
    pub options: Vec<Arc<dyn DandelionProcessedOption>>,
    /// java: private final Map<Integer, DandelionProcessedOption> accordions; - accordion ids to accordion options
    pub accordions: HashMap<i32, Arc<dyn DandelionProcessedOption>>,
}

impl DandelionProcessedCategory {
    /// java: protected DandelionProcessedCategory(ConfigCategory category, List<DandelionProcessedOption> options)
    pub fn new(category: Arc<dyn ConfigCategory>, options: Vec<Arc<dyn DandelionProcessedOption>>) -> Arc<DandelionProcessedCategory> {
        // java: this.accordions = this.options.stream()
        //           .filter(opt -> opt.editor instanceof GuiOptionEditorAccordion)
        //           .collect(toUnmodifiableMap(opt -> ((GuiOptionEditorAccordion) opt.editor).getAccordionId(), identity()));
        // (`editor` is the FIELD, created lazily by `getEditor()`; no option has one yet when a category is built, so
        // the map is empty here as it is in Java)
        let mut accordions: HashMap<i32, Arc<dyn DandelionProcessedOption>> = HashMap::new();
        for option in &options {
            let editor = option.dandelion_processed_option_base().editor.borrow().clone();
            if let Some(editor) = editor {
                if let Some(accordion) = editor.as_any().downcast_ref::<GuiOptionEditorAccordion>() {
                    // java: toUnmodifiableMap throws IllegalStateException on a duplicate key
                    if accordions.insert(accordion.get_accordion_id(), option.clone()).is_some() {
                        panic!("IllegalStateException: Duplicate key {}", accordion.get_accordion_id());
                    }
                }
            }
        }
        let this = Arc::new(DandelionProcessedCategory { category, options, accordions });
        // Bind our processed options to this category
        let as_category: Arc<dyn ProcessedCategory> = this.clone();
        for option in &this.options {
            *option.dandelion_processed_option_base().category.borrow_mut() = Some(Arc::downgrade(&as_category));
        }
        this
    }
}

impl HasDebugLocation for DandelionProcessedCategory {
    /// java: @Override public @Nullable String getDebugDeclarationLocation()
    fn get_debug_declaration_location(&self) -> Option<String> {
        Some(self.category.id().to_string())
    }
}

impl ProcessedCategory for DandelionProcessedCategory {
    /// java: @Override public StructuredText getDisplayName()
    fn get_display_name(&self) -> Arc<dyn StructuredText> {
        wrap_component(&self.category.name())
    }

    /// java: @Override public StructuredText getDescription()
    fn get_description(&self) -> Arc<dyn StructuredText> {
        wrap_component(&self.category.description())
    }

    /// java: @Override public String getIdentifier()
    fn get_identifier(&self) -> String {
        self.category.id().to_string()
    }

    /// java: @Override public @Nullable String getParentCategoryId()
    fn get_parent_category_id(&self) -> Option<String> {
        None
    }

    /// java: @Override @Unmodifiable public List<ProcessedOption> getOptions()
    fn get_options(&self) -> Vec<Arc<dyn ProcessedOption>> {
        self.options.iter().map(|o| o.as_processed_option()).collect()
    }

    /// java: @Override @Unmodifiable public Map<Integer, ProcessedOption> getAccordionAnchors()
    fn get_accordion_anchors(&self) -> HashMap<i32, Arc<dyn ProcessedOption>> {
        self.accordions.iter().map(|(k, v)| (*k, v.as_processed_option())).collect()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
