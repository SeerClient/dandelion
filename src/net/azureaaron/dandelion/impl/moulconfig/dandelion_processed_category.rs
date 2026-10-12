//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedCategory by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption;

/// java: public class DandelionProcessedCategory implements ProcessedCategory
#[derive(Clone)]
pub struct DandelionProcessedCategory {
    /// java: private final ConfigCategory category;
    pub category: compat::Shared<dyn ConfigCategory>,
    /// java: private final List<DandelionProcessedOption> options;
    pub options: Vec<compat::Shared<dyn DandelionProcessedOption>>,
    /// java: private final Map<Integer, DandelionProcessedOption> accordions;
    pub accordions: std::collections::HashMap<i32, compat::Shared<dyn DandelionProcessedOption>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl DandelionProcessedCategory {
    /// java: protected DandelionProcessedCategory(ConfigCategory category, List<DandelionProcessedOption> options)
    pub fn new(mut category: compat::Shared<dyn ConfigCategory>, mut options: Vec<compat::Shared<dyn DandelionProcessedOption>>) -> compat::Shared<Self> {
        let __init_category = category.clone();
        let __init_options = options.iter().cloned().collect::<Vec<_>>();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            category: __init_category,
            options: __init_options,
            accordions: Default::default(),
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        // TODO(fabric2seer): `.getAccordionId(..)` on a GuiOptionEditorAccordion
        //   java: this.accordions = this.options.stream()
        //   java: 				.filter(opt -> opt.editor instanceof GuiOptionEditorAccordion)
        //   java: 				.collect(Collectors.toUnmodifiableMap(opt -> ((GuiOptionEditorAccordion) opt.editor).getAccordionId(), Function.identity()));
        for option in compat::read(&__this, |__t| __t.options.clone()) {
            { let __v = Some(__this); compat::write(&option, |__t| __t.dandelion_processed_option_base_mut().category = __v); }
        }
        __this
    }

    /// java: @Override public @Nullable String getDebugDeclarationLocation()
    pub fn get_debug_declaration_location(__this: &compat::Shared<Self>) -> Option<String> {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: return Some(({ let __r = &compat::read(__this, |__t| __t.category.clone()); let __f = compat::dispatch(__r, |__t| __t.id()); __f(__r) }).fmt());
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public StructuredText getDisplayName()
    pub fn get_display_name(__this: &compat::Shared<Self>) -> crate::unresolved::StructuredText {
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: return MoulConfigPlatform.wrap(this.category.name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public StructuredText getDescription()
    pub fn get_description(__this: &compat::Shared<Self>) -> crate::unresolved::StructuredText {
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: return MoulConfigPlatform.wrap(this.category.description());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public String getIdentifier()
    pub fn get_identifier(__this: &compat::Shared<Self>) -> String {
        // TODO(fabric2seer): `fmt` takes 1 argument(s) in the port (`f: &mut fmt::Formatter<'_>`), Java passes 0
        // rust: return ({ let __r = &compat::read(__this, |__t| __t.category.clone()); let __f = compat::dispatch(__r, |__t| __t.id()); __f(__r) }).fmt();
        return todo!("fabric2seer: see the TODO above");
    }

    /// java: @Override public @Nullable String getParentCategoryId()
    pub fn get_parent_category_id(__this: &compat::Shared<Self>) -> Option<String> {
        return None;
    }

    /// java: @SuppressWarnings("unchecked") @Override @Unmodifiable public List<ProcessedOption> getOptions()
    pub fn get_options(__this: &compat::Shared<Self>) -> Vec<crate::unresolved::ProcessedOption> {
        // TODO(fabric2seer): `.cast(..)` on a Class
        //   java: return List.class.cast(this.options);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @SuppressWarnings("unchecked") @Override @Unmodifiable public Map<Integer, ProcessedOption> getAccordionAnchors()
    pub fn get_accordion_anchors(__this: &compat::Shared<Self>) -> std::collections::HashMap<i32, crate::unresolved::ProcessedOption> {
        // TODO(fabric2seer): `.cast(..)` on a Class
        //   java: return Map.class.cast(this.accordions);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
