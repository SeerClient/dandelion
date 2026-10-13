//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedOption.
//!
//! An abstract processed option which will either represent an editable option or a group/accordion option.
//! rust: MoulConfig's `ProcessedOption` interface is not ported (GAPS_D.md D1); the class's members are this trait,
//! its fields `DandelionProcessedOptionBase` (PATTERNS_Y2 P3), and an object is an `Arc<dyn DandelionProcessedOption>`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;
use std::cell::RefCell;

/// java: public abstract class DandelionProcessedOption implements ProcessedOption - its fields
pub struct DandelionProcessedOptionBase {
    /// java: private final int accordionId;
    pub accordion_id: i32,
    /// java: private final Config config;
    pub config: Arc<MoulConfigDefinition>,
    /// java: protected ProcessedCategory category;
    pub category: RefCell<crate::unresolved::ProcessedCategory>,
    /// java: protected @Nullable GuiOptionEditor editor;
    pub editor: RefCell<Option<crate::unresolved::GuiOptionEditor>>,
}

impl DandelionProcessedOptionBase {
    /// java: protected DandelionProcessedOption(int accordionId, Config config)
    pub fn new(accordion_id: i32, config: Arc<MoulConfigDefinition>) -> Self {
        Self { accordion_id, config, category: RefCell::new(Default::default()), editor: RefCell::new(None) }
    }
}

/// java: the virtual methods of DandelionProcessedOption (and of the `ProcessedOption` members its subclasses
/// override)
pub trait DandelionProcessedOption: compat::DynAny {
    fn dandelion_processed_option_base(&self) -> &DandelionProcessedOptionBase;

    /// java: @Override public int getAccordionId()
    fn get_accordion_id(&self) -> i32 {
        self.dandelion_processed_option_base().accordion_id
    }

    /// java: protected abstract GuiOptionEditor createEditor();
    fn create_editor(&self) -> crate::unresolved::GuiOptionEditor;

    /// java: @Override public GuiOptionEditor getEditor()
    fn get_editor(&self) -> crate::unresolved::GuiOptionEditor {
        let base = self.dandelion_processed_option_base();
        if base.editor.borrow().is_none() {
            let editor = self.create_editor();
            *base.editor.borrow_mut() = Some(editor);
        }
        base.editor.borrow().clone().expect("the editor was just created")
    }

    /// java: @Override public ProcessedCategory getCategory()
    fn get_category(&self) -> crate::unresolved::ProcessedCategory {
        self.dandelion_processed_option_base().category.borrow().clone()
    }

    /// java: @Override public String getPath()
    fn get_path(&self) -> String {
        "NYI".to_string()
    }

    /// java: @Override public Config getConfig()
    fn get_config(&self) -> Arc<MoulConfigDefinition> {
        self.dandelion_processed_option_base().config.clone()
    }

    /// java: @Override public void explicitNotifyChange() {}
    fn explicit_notify_change(&self) {}

    /// java: ProcessedOption.getName()
    fn get_name(&self) -> crate::unresolved::StructuredText;

    /// java: ProcessedOption.getDescription()
    fn get_description(&self) -> crate::unresolved::StructuredText;

    /// java: ProcessedOption.getSearchTags()
    fn get_search_tags(&self) -> Vec<crate::unresolved::SearchTag>;

    /// java: ProcessedOption.getType()
    fn get_type(&self) -> crate::unresolved::Type;

    /// java: ProcessedOption.get()
    fn get(&self) -> Arc<dyn std::any::Any>;

    /// java: ProcessedOption.set(Object value)
    fn set(&self, value: Arc<dyn std::any::Any>) -> bool;

    /// java: ProcessedOption.getDebugDeclarationLocation()
    fn get_debug_declaration_location(&self) -> Option<String>;
}
