//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedEditableOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOptionBase;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_search_tag::DandelionSearchTag;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use crate::net::azureaaron::dandelion::api::option::Option as _;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding as _;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public abstract class DandelionProcessedEditableOption<T> extends DandelionProcessedOption
pub struct DandelionProcessedEditableOptionBase<T> {
    /// java: extends DandelionProcessedOption
    pub base: DandelionProcessedOptionBase,
    /// java: protected final Option<T> option;
    pub option: compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>,
}

impl<T: 'static> std::ops::Deref for DandelionProcessedEditableOptionBase<T> {
    type Target = DandelionProcessedOptionBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<T: 'static> std::ops::DerefMut for DandelionProcessedEditableOptionBase<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<T: Clone + 'static> DandelionProcessedEditableOptionBase<T> {
    /// java: protected DandelionProcessedEditableOption(Option<T> option, int accordionId, Config config) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut option: compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<T>>, mut accordion_id: i32, mut config: crate::unresolved::Config, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn DandelionProcessedEditableOption>, __R)) -> (compat::Shared<dyn DandelionProcessedEditableOption>, __R) {
        let __init_option = option.clone();
        let (_, (__this, __r)) = DandelionProcessedOptionBase::new_in(accordion_id, config.clone(), move |__base| {
            let this = Self {
                base: __base,
                option: __init_option,
            };
            let (__h, __r) = __wrap(this);
            ((__h.clone() as compat::Shared<dyn DandelionProcessedOption>), (__h, __r))
        });
        (__this, __r)
    }

}

/// java: the virtual methods of DandelionProcessedEditableOption (implemented by DandelionProcessedEditableOptionBase and every subclass)
pub trait DandelionProcessedEditableOption<T: Clone + 'static>: DandelionProcessedOption {
    fn dandelion_processed_editable_option_base(&self) -> &DandelionProcessedEditableOptionBase<T>;
    fn dandelion_processed_editable_option_base_mut(&mut self) -> &mut DandelionProcessedEditableOptionBase<T>;

    /// java: @Override public StructuredText getName()
    fn get_name(&self) -> crate::unresolved::StructuredText {
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: return MoulConfigPlatform.wrap(this.option.name());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public StructuredText getDescription()
    fn get_description(&self) -> crate::unresolved::StructuredText {
        let mut concat: MutableComponent = component::empty();
        concat.get_siblings().extend(self.dandelion_processed_editable_option_base().option.lock().borrow().description().into_iter());
        // TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
        //   java: return MoulConfigPlatform.wrap(concat);
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public SearchTag[] getSearchTags()
    fn get_search_tags(&self) -> Vec<crate::unresolved::SearchTag> {
        let mut tags: Vec<crate::unresolved::SearchTag> = self.dandelion_processed_editable_option_base().option.lock().borrow().tags().into_iter().map(|__a0| __a0.get_string()).map(|__a0| DandelionSearchTag::new(__a0.clone())).collect::<Vec<_>>();
        return tags;
    }

    /// java: @Override public Type getType()
    fn get_type(&self) -> crate::unresolved::Type {
        return self.dandelion_processed_editable_option_base().option.lock().borrow().r#type();
    }

    /// java: @Override public Object get()
    fn get(&self) -> compat::Object {
        return (std::sync::Arc::new(self.dandelion_processed_editable_option_base().option.lock().borrow().binding().lock().borrow().get()) as compat::Object);
    }

    /// java: @SuppressWarnings("unchecked") @Override public boolean set(Object value)
    fn set(&self, value: &compat::Object) -> bool {
        // java: try {..} catch (Exception e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.dandelion_processed_editable_option_base_mut().option.lock().borrow().binding().lock().borrow_mut().set(value);
            // TODO(fabric2seer): `explicitNotifyChange(..)` (an overridable method) called where the object has no handle (a `&self` method of a trait: a generic method)
            //   java: this.explicitNotifyChange();
            return true;
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                logger().error(&format!("[Dandelion] Failed to set option! Id: {}, Name: {}", compat::opt_str(&self.dandelion_processed_editable_option_base().option.lock().borrow().id()), compat::jstring(&self.get_name())));
            }
        }
        return false;
    }

    /// java: @Override public @Nullable String getDebugDeclarationLocation()
    fn get_debug_declaration_location(&self) -> Option<String> {
        // TODO(fabric2seer): `.getText(..)` on a StructuredText
        //   java: return this.option.id() != null ? this.option.id().toString() : this.getName().getText();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

// DandelionProcessedEditableOption.explicitNotifyChange implements DandelionProcessedOption.explicitNotifyChange: it is emitted in each concrete subclass's `impl DandelionProcessedOption` (Rust has no inherited impls)
