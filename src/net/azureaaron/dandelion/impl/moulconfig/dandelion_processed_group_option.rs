//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedGroupOption.
//!
//! This class represents a `ProcessedOption` that maps to an `OptionGroup` or accordion (in MoulConfig terms) under
//! which editable options exist.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::{DandelionProcessedOption, DandelionProcessedOptionBase};
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_search_tag::DandelionSearchTag;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::{wrap_component, MoulConfigDefinition};
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use moul::annotations::search_tag::SearchTag;
use moul::common::text::structured_text::StructuredText;
use moul::gui::editors::gui_option_editor_accordion::GuiOptionEditorAccordion;
use moul::gui::gui_option_editor::GuiOptionEditor;
use moul::processor::processed_option::{ProcessedOption, Type};
use std::any::{Any, TypeId};

/// java: public class DandelionProcessedGroupOption extends DandelionProcessedOption
pub struct DandelionProcessedGroupOption {
    /// java: extends DandelionProcessedOption
    pub base: DandelionProcessedOptionBase,
    /// java: private final OptionGroup group;
    pub group: Arc<dyn OptionGroup>,
    /// java: private final int categoryAccordionId;
    pub category_accordion_id: i32,
}

impl DandelionProcessedGroupOption {
    /// java: protected DandelionProcessedGroupOption(OptionGroup group, int rootAccordionId, int categoryAccordionId, Config config)
    pub fn new(group: Arc<dyn OptionGroup>, root_accordion_id: i32, category_accordion_id: i32, config: Arc<MoulConfigDefinition>) -> Arc<Self> {
        let this = Arc::new(Self { base: DandelionProcessedOptionBase::new(root_accordion_id, config), group, category_accordion_id });
        let as_option: Arc<dyn ProcessedOption> = this.clone();
        *this.base.this.borrow_mut() = Some(Arc::downgrade(&as_option));
        this
    }
}

impl DandelionProcessedOption for DandelionProcessedGroupOption {
    fn dandelion_processed_option_base(&self) -> &DandelionProcessedOptionBase {
        &self.base
    }

    /// java: @Override public StructuredText getName()
    fn get_name(&self) -> Arc<dyn StructuredText> {
        wrap_component(&self.group.name())
    }

    /// java: @Override public StructuredText getDescription()
    fn get_description(&self) -> Arc<dyn StructuredText> {
        let mut concat = component::empty();
        for line in self.group.description() {
            concat = concat.append(line);
        }
        wrap_component(&concat)
    }

    /// java: @Override public SearchTag[] getSearchTags()
    fn get_search_tags(&self) -> Vec<Arc<dyn SearchTag>> {
        self.group
            .tags()
            .iter()
            .map(|tag| -> Arc<dyn SearchTag> { Arc::new(DandelionSearchTag::new(tag.get_string())) })
            .collect()
    }

    /// java: @Override public Object get() - `(Void) null`
    fn get(&self) -> Arc<dyn Any> {
        Arc::new(())
    }

    /// java: @Override public Type getType() - `Void.class`
    fn get_type(&self) -> Type {
        TypeId::of::<()>()
    }

    /// java: @Override public boolean set(Object value)
    fn set(&self, value: Arc<dyn Any>) -> bool {
        let _ = value;
        false
    }

    /// java: @Override public @Nullable String getDebugDeclarationLocation()
    fn get_debug_declaration_location(&self) -> Option<String> {
        match self.group.id() {
            Some(id) => Some(id.to_string()),
            None => Some(DandelionProcessedOption::get_name(self).get_text()),
        }
    }

    /// java: @Override protected GuiOptionEditor createEditor() - `new GuiOptionEditorAccordion(this, this.categoryAccordionId)`
    fn create_editor(&self) -> Arc<dyn GuiOptionEditor> {
        GuiOptionEditorAccordion::new(self.as_processed_option(), self.category_accordion_id)
    }
}

crate::dandelion_processed_option!(DandelionProcessedGroupOption);
