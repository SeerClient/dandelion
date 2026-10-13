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
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use std::any::Any;

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
    pub fn new(group: Arc<dyn OptionGroup>, root_accordion_id: i32, category_accordion_id: i32, config: Arc<MoulConfigDefinition>) -> Self {
        Self { base: DandelionProcessedOptionBase::new(root_accordion_id, config), group, category_accordion_id }
    }
}

impl DandelionProcessedOption for DandelionProcessedGroupOption {
    fn dandelion_processed_option_base(&self) -> &DandelionProcessedOptionBase {
        &self.base
    }

    /// java: @Override public StructuredText getName()
    fn get_name(&self) -> crate::unresolved::StructuredText {
        wrap_component(&self.group.name())
    }

    /// java: @Override public StructuredText getDescription()
    fn get_description(&self) -> crate::unresolved::StructuredText {
        let mut concat = component::empty();
        for line in self.group.description() {
            concat = concat.append(line);
        }
        wrap_component(&concat)
    }

    /// java: @Override public SearchTag[] getSearchTags()
    fn get_search_tags(&self) -> Vec<crate::unresolved::SearchTag> {
        self.group
            .tags()
            .iter()
            .map(|tag| crate::unresolved::SearchTag::of(Arc::new(DandelionSearchTag::new(tag.get_string()))))
            .collect()
    }

    /// java: @Override public Object get() - `(Void) null`
    fn get(&self) -> Arc<dyn Any> {
        Arc::new(())
    }

    /// java: @Override public Type getType() - `Void.class`
    fn get_type(&self) -> crate::unresolved::Type {
        crate::unresolved::Type::of(Arc::new(std::any::TypeId::of::<()>()))
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
            None => {
                let _ = self.get_name();
                todo!("fabric2seer: StructuredText.getText() - MoulConfig is not ported (GAPS_D.md D1)")
            }
        }
    }

    /// java: @Override protected GuiOptionEditor createEditor()
    fn create_editor(&self) -> crate::unresolved::GuiOptionEditor {
        // java: return new GuiOptionEditorAccordion(this, this.categoryAccordionId);
        todo!("fabric2seer: `new GuiOptionEditorAccordion(..)` - MoulConfig is not ported (GAPS_D.md D1)")
    }
}
