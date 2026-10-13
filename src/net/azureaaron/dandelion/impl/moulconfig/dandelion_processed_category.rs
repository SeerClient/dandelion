//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedCategory.
//!
//! Represents a category in the config which holds a list of options which themselves can be editable options under
//! the root or group/accordion options that hold editable options.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::wrap_component;
use std::collections::HashMap;

/// java: public class DandelionProcessedCategory implements ProcessedCategory
pub struct DandelionProcessedCategory {
    /// java: private final ConfigCategory category; - the `ConfigCategory` that this category is bound to
    pub category: Arc<dyn ConfigCategory>,
    /// java: private final List<DandelionProcessedOption> options;
    pub options: Vec<Arc<dyn DandelionProcessedOption>>,
    /// java: private final Map<Integer, DandelionProcessedOption> accordions; - accordion ids to accordion options
    pub accordions: HashMap<i32, Arc<dyn DandelionProcessedOption>>,
}

impl DandelionProcessedCategory {
    /// java: protected DandelionProcessedCategory(ConfigCategory category, List<DandelionProcessedOption> options)
    pub fn new(category: Arc<dyn ConfigCategory>, options: Vec<Arc<dyn DandelionProcessedOption>>) -> Arc<DandelionProcessedCategory> {
        // java: .filter(opt -> opt.editor instanceof GuiOptionEditorAccordion)
        //       .collect(toUnmodifiableMap(opt -> ((GuiOptionEditorAccordion) opt.editor).getAccordionId(), identity()))
        // `editor` is created lazily (`getEditor()`), so no option has one yet and the map is empty - as in Java
        let mut accordions: HashMap<i32, Arc<dyn DandelionProcessedOption>> = HashMap::new();
        for option in &options {
            if let Some(editor) = option.dandelion_processed_option_base().editor.borrow().as_ref() {
                if editor.as_any().is::<crate::unresolved::GuiOptionEditorAccordion>() {
                    // java: ((GuiOptionEditorAccordion) opt.editor).getAccordionId() - rust: port gap (GAPS_D.md D1)
                    todo!("fabric2seer: GuiOptionEditorAccordion.getAccordionId() - MoulConfig is not ported (GAPS_D.md D1)");
                }
            }
        }
        let _ = &mut accordions;
        let this = Arc::new(DandelionProcessedCategory { category, options, accordions });
        // Bind our processed options to this category
        for option in &this.options {
            *option.dandelion_processed_option_base().category.borrow_mut() = crate::unresolved::ProcessedCategory::of(this.clone());
        }
        this
    }

    /// java: @Override public @Nullable String getDebugDeclarationLocation()
    pub fn get_debug_declaration_location(&self) -> Option<String> {
        Some(self.category.id().to_string())
    }

    /// java: @Override public StructuredText getDisplayName()
    pub fn get_display_name(&self) -> crate::unresolved::StructuredText {
        wrap_component(&self.category.name())
    }

    /// java: @Override public StructuredText getDescription()
    pub fn get_description(&self) -> crate::unresolved::StructuredText {
        wrap_component(&self.category.description())
    }

    /// java: @Override public String getIdentifier()
    pub fn get_identifier(&self) -> String {
        self.category.id().to_string()
    }

    /// java: @Override public @Nullable String getParentCategoryId()
    pub fn get_parent_category_id(&self) -> Option<String> {
        None
    }

    /// java: @Override @Unmodifiable public List<ProcessedOption> getOptions()
    pub fn get_options(&self) -> Vec<Arc<dyn DandelionProcessedOption>> {
        self.options.clone()
    }

    /// java: @Override @Unmodifiable public Map<Integer, ProcessedOption> getAccordionAnchors()
    pub fn get_accordion_anchors(&self) -> HashMap<i32, Arc<dyn DandelionProcessedOption>> {
        self.accordions.clone()
    }
}
