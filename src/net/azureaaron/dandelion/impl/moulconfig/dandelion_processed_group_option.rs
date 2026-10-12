//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedGroupOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOptionBase;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_search_tag::DandelionSearchTag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class DandelionProcessedGroupOption extends DandelionProcessedOption
pub struct DandelionProcessedGroupOption {
    /// java: extends DandelionProcessedOption
    pub base: DandelionProcessedOptionBase,
    /// java: private final OptionGroup group;
    pub group: compat::Shared<dyn OptionGroup>,
    /// java: private final int categoryAccordionId;
    pub category_accordion_id: i32,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for DandelionProcessedGroupOption {
    type Target = DandelionProcessedOptionBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DandelionProcessedGroupOption {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DandelionProcessedGroupOption {
    /// java: protected DandelionProcessedGroupOption(OptionGroup group, int rootAccordionId, int categoryAccordionId, Config config)
    pub fn new(mut group: compat::Shared<dyn OptionGroup>, mut root_accordion_id: i32, mut category_accordion_id: i32, mut config: crate::unresolved::Config) -> compat::Shared<Self> {
        let __init_group = group.clone();
        let __init_category_accordion_id = category_accordion_id;
        let (_, __this) = DandelionProcessedOptionBase::new_in(root_accordion_id, config.clone(), move |__base| {
            let __h = compat::shared(Self {
                base: __base,
                self_ref: std::sync::Weak::new(),
                group: __init_group,
                category_accordion_id: __init_category_accordion_id,
            });
            compat::write(&__h, |__t| __t.self_ref = std::sync::Arc::downgrade(&__h));
            ((__h.clone() as compat::Shared<dyn DandelionProcessedOption>), __h)
        });
        __this
    }

    /// java: @Override public StructuredText getName()
    pub fn get_name(__this: &compat::Shared<Self>) -> crate::unresolved::StructuredText {
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: return MoulConfigPlatform.wrap(this.group.name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public StructuredText getDescription()
    pub fn get_description(__this: &compat::Shared<Self>) -> crate::unresolved::StructuredText {
        let mut concat: MutableComponent = component::empty();
        concat.get_siblings().extend(({ let __r = &compat::read(__this, |__t| __t.group.clone()); let __f = compat::dispatch(__r, |__t| __t.description()); __f(__r) }).into_iter());
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: return MoulConfigPlatform.wrap(concat);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public SearchTag[] getSearchTags()
    pub fn get_search_tags(__this: &compat::Shared<Self>) -> Vec<crate::unresolved::SearchTag> {
        let mut tags: Vec<crate::unresolved::SearchTag> = ({ let __r = &compat::read(__this, |__t| __t.group.clone()); let __f = compat::dispatch(__r, |__t| __t.tags()); __f(__r) }).into_iter().map(|__a0| __a0.get_string()).map(|__a0| DandelionSearchTag::new(__a0.clone())).collect::<Vec<_>>();
        return tags;
    }

    /// java: @Override public Object get()
    pub fn get(__this: &compat::Shared<Self>) -> compat::Object {
        return (std::sync::Arc::new(((None) as ())) as compat::Object);
    }

    /// java: @Override public Type getType()
    pub fn get_type(__this: &compat::Shared<Self>) -> crate::unresolved::Type {
        return std::any::TypeId::of::<()>();
    }

    /// java: @Override public boolean set(Object value)
    pub fn set(__this: &compat::Shared<Self>, value: &compat::Object) -> bool {
        return false;
    }

    /// java: @Override public @Nullable String getDebugDeclarationLocation()
    pub fn get_debug_declaration_location(__this: &compat::Shared<Self>) -> Option<String> {
        // TODO(fabric2seer): `.getText(..)` on a StructuredText
        //   java: return this.group.id() != null ? this.group.id().toString() : this.getName().getText();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl DandelionProcessedOption for DandelionProcessedGroupOption {
    fn dandelion_processed_option_base(&self) -> &DandelionProcessedOptionBase {
        &self.base
    }
    fn dandelion_processed_option_base_mut(&mut self) -> &mut DandelionProcessedOptionBase {
        &mut self.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override protected GuiOptionEditor createEditor() (dispatch: the body for this class)
    fn create_editor(&self) -> fn(&compat::Shared<dyn DandelionProcessedOption>) -> crate::unresolved::GuiOptionEditor {
        DandelionProcessedGroupOption::create_editor_dyn
    }

}

impl DandelionProcessedGroupOption {
    /// java: @Override protected GuiOptionEditor createEditor()
    pub fn create_editor_impl(__this: &compat::Shared<Self>) -> crate::unresolved::GuiOptionEditor {
        // TODO(fabric2seer): `new GuiOptionEditorAccordion(..)`
        //   java: return new GuiOptionEditorAccordion(this, this.categoryAccordionId);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// DandelionProcessedGroupOption.createEditor as the dispatch of DandelionProcessedOption.createEditor
    pub fn create_editor_dyn(__this: &compat::Shared<dyn DandelionProcessedOption>) -> crate::unresolved::GuiOptionEditor {
        Self::create_editor_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
