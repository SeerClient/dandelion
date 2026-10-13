//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedEditableOption.
//!
//! Represents an actual option which can be edited in the config, this option will either lie within a group or under
//! the root determined by its `accordionId`.
//!
//! Java's subclasses are anonymous classes (`MoulConfigEditableOptionAdapter`): `createEditor()` and optionally
//! `getType()` / `get()` / `set(..)`. Here they are the `EditableOptionClass` the object is built with; an override
//! that calls `super.get()` calls `DandelionProcessedEditableOption::super_get`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::option::{AnyOption, Option};
use crate::net::azureaaron::dandelion::api::option_listener::UpdateType;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::{DandelionProcessedOption, DandelionProcessedOptionBase};
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_search_tag::DandelionSearchTag;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::{wrap_component, MoulConfigDefinition};
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use std::any::Any;
use std::sync::Weak;

type This<T> = DandelionProcessedEditableOption<T>;

/// The members an anonymous subclass of `DandelionProcessedEditableOption` overrides.
pub struct EditableOptionClass<T: Clone + PartialEq + 'static> {
    /// java: protected GuiOptionEditor createEditor()
    pub create_editor: Box<dyn Fn(&This<T>) -> crate::unresolved::GuiOptionEditor>,
    /// java: @Override public Type getType()
    pub get_type: std::option::Option<Box<dyn Fn(&This<T>) -> crate::unresolved::Type>>,
    /// java: @Override public Object get()
    pub get: std::option::Option<Box<dyn Fn(&This<T>) -> Arc<dyn Any>>>,
    /// java: @Override public boolean set(Object value)
    pub set: std::option::Option<Box<dyn Fn(&This<T>, Arc<dyn Any>) -> bool>>,
}

/// java: public abstract class DandelionProcessedEditableOption<T> extends DandelionProcessedOption
pub struct DandelionProcessedEditableOption<T: Clone + PartialEq + 'static> {
    /// java: extends DandelionProcessedOption
    pub base: DandelionProcessedOptionBase,
    /// java: protected final Option<T> option;
    pub option: Arc<dyn Option<T>>,
    /// the anonymous subclass's overrides
    pub class: EditableOptionClass<T>,
    /// Java's `this` handed to the editors (`new DandelionLabelEditor(this, ..)`)
    pub this: Weak<This<T>>,
}

impl<T: Clone + PartialEq + 'static> DandelionProcessedEditableOption<T> {
    /// java: protected DandelionProcessedEditableOption(Option<T> option, int accordionId, Config config)
    pub fn new(option: Arc<dyn Option<T>>, accordion_id: i32, config: Arc<MoulConfigDefinition>, class: EditableOptionClass<T>) -> Arc<This<T>> {
        Arc::new_cyclic(|this| Self { base: DandelionProcessedOptionBase::new(accordion_id, config), option, class, this: this.clone() })
    }

    /// `this` as MoulConfig's `ProcessedOption` (what the editors take)
    pub fn as_processed_option(&self) -> crate::unresolved::ProcessedOption {
        crate::unresolved::ProcessedOption::of(self.this.upgrade().expect("a processed option is used after it was dropped"))
    }

    /// java: `super.get()` - DandelionProcessedEditableOption.get()
    pub fn super_get(&self) -> Arc<dyn Any> {
        Arc::new(self.option.binding().get())
    }

    /// java: `super.set(value)` - DandelionProcessedEditableOption.set(Object)
    pub fn super_set(&self, value: Arc<dyn Any>) -> bool {
        // java: try { this.option.binding().set((T) value); .. } catch (Exception e) - a failed cast is the exception
        match value.downcast_ref::<T>() {
            Some(value) => {
                self.option.binding().set(value.clone());
                DandelionProcessedOption::explicit_notify_change(self);
                true
            }
            None => {
                logger().error(&format!(
                    "[Dandelion] Failed to set option! Id: {}, Name: {} ClassCastException",
                    compat::opt_str(&self.option.id()),
                    self.option.name().get_string()
                ));
                false
            }
        }
    }
}

impl<T: Clone + PartialEq + 'static> DandelionProcessedOption for DandelionProcessedEditableOption<T> {
    fn dandelion_processed_option_base(&self) -> &DandelionProcessedOptionBase {
        &self.base
    }

    fn create_editor(&self) -> crate::unresolved::GuiOptionEditor {
        (self.class.create_editor)(self)
    }

    /// java: @Override public StructuredText getName()
    fn get_name(&self) -> crate::unresolved::StructuredText {
        wrap_component(&self.option.name())
    }

    /// java: @Override public StructuredText getDescription()
    fn get_description(&self) -> crate::unresolved::StructuredText {
        // java: Component concat = Component.empty(); concat.getSiblings().addAll(this.option.description());
        let mut concat = component::empty();
        for line in self.option.description() {
            concat = concat.append(line);
        }
        wrap_component(&concat)
    }

    /// java: @Override public SearchTag[] getSearchTags()
    fn get_search_tags(&self) -> Vec<crate::unresolved::SearchTag> {
        self.option
            .tags()
            .iter()
            .map(|tag| crate::unresolved::SearchTag::of(Arc::new(DandelionSearchTag::new(tag.get_string()))))
            .collect()
    }

    /// java: @Override public Type getType()
    fn get_type(&self) -> crate::unresolved::Type {
        match &self.class.get_type {
            Some(get_type) => get_type(self),
            // java: return this.option.type();
            None => crate::unresolved::Type::of(Arc::new(self.option.r#type())),
        }
    }

    /// java: @Override public Object get()
    fn get(&self) -> Arc<dyn Any> {
        match &self.class.get {
            Some(get) => get(self),
            None => self.super_get(),
        }
    }

    /// java: @SuppressWarnings("unchecked") @Override public boolean set(Object value)
    fn set(&self, value: Arc<dyn Any>) -> bool {
        match &self.class.set {
            Some(set) => set(self, value),
            None => self.super_set(value),
        }
    }

    /// java: @Override public void explicitNotifyChange()
    fn explicit_notify_change(&self) {
        for listener in self.option.listeners() {
            listener.on_update(&*self.option, UpdateType::ValueChange);
        }
        // java: ((MoulConfigDefinition) this.getConfig()).queueOptionFlags(this.option.flags());
        self.get_config().queue_option_flags(self.option.flags());
    }

    /// java: @Override public @Nullable String getDebugDeclarationLocation()
    fn get_debug_declaration_location(&self) -> std::option::Option<String> {
        match self.option.id() {
            Some(id) => Some(id.to_string()),
            // java: this.getName().getText()
            None => {
                let _ = self.get_name();
                todo!("fabric2seer: StructuredText.getText() - MoulConfig is not ported (GAPS_D.md D1)")
            }
        }
    }
}
