//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedOption.
//!
//! An abstract processed option which will either represent an editable option or a group/accordion option.
//!
//! `public abstract class DandelionProcessedOption implements ProcessedOption`: the class's virtuals are the trait
//! `DandelionProcessedOption`, its fields `DandelionProcessedOptionBase`, and an object is an
//! `Arc<dyn DandelionProcessedOption>`. MoulConfig's `ProcessedOption` (the interface it implements) is implemented for
//! each concrete subclass by `dandelion_processed_option!`, forwarding to these methods; `as_processed_option()` is
//! the object as that interface (Java's widening).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use moul::annotations::search_tag::SearchTag;
use moul::common::text::structured_text::StructuredText;
use moul::gui::gui_option_editor::GuiOptionEditor;
use moul::processor::processed_category::ProcessedCategory;
use moul::processor::processed_option::{ProcessedOption, Type};
use std::cell::RefCell;
use std::sync::Weak;

/// java: public abstract class DandelionProcessedOption implements ProcessedOption - its fields
pub struct DandelionProcessedOptionBase {
    /// java: private final int accordionId;
    pub accordion_id: i32,
    /// java: private final Config config;
    pub config: Arc<MoulConfigDefinition>,
    /// java: protected ProcessedCategory category; - the category holds its options, so this is the back reference
    pub category: RefCell<Option<Weak<dyn ProcessedCategory>>>,
    /// java: protected @Nullable GuiOptionEditor editor;
    pub editor: RefCell<Option<Arc<dyn GuiOptionEditor>>>,
    /// Java's `this` as a `ProcessedOption` (set when the object is built)
    pub this: RefCell<Option<Weak<dyn ProcessedOption>>>,
}

impl DandelionProcessedOptionBase {
    /// java: protected DandelionProcessedOption(int accordionId, Config config)
    pub fn new(accordion_id: i32, config: Arc<MoulConfigDefinition>) -> Self {
        Self { accordion_id, config, category: RefCell::new(None), editor: RefCell::new(None), this: RefCell::new(None) }
    }
}

/// java: the virtual methods of DandelionProcessedOption (and of the `ProcessedOption` members its subclasses
/// override)
pub trait DandelionProcessedOption: compat::DynAny {
    fn dandelion_processed_option_base(&self) -> &DandelionProcessedOptionBase;

    /// `this` as MoulConfig's `ProcessedOption`
    fn as_processed_option(&self) -> Arc<dyn ProcessedOption> {
        self.dandelion_processed_option_base()
            .this
            .borrow()
            .as_ref()
            .and_then(|w| w.upgrade())
            .expect("a processed option is used after it was dropped")
    }

    /// java: @Override public int getAccordionId()
    fn get_accordion_id(&self) -> i32 {
        self.dandelion_processed_option_base().accordion_id
    }

    /// java: protected abstract GuiOptionEditor createEditor();
    fn create_editor(&self) -> Arc<dyn GuiOptionEditor>;

    /// java: @Override public GuiOptionEditor getEditor()
    fn get_editor(&self) -> Arc<dyn GuiOptionEditor> {
        let base = self.dandelion_processed_option_base();
        if base.editor.borrow().is_none() {
            let editor = self.create_editor();
            *base.editor.borrow_mut() = Some(editor);
        }
        base.editor.borrow().clone().expect("the editor was just created")
    }

    /// java: @Override public ProcessedCategory getCategory()
    fn get_category(&self) -> Option<Arc<dyn ProcessedCategory>> {
        self.dandelion_processed_option_base().category.borrow().as_ref().and_then(|w| w.upgrade())
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
    fn get_name(&self) -> Arc<dyn StructuredText>;

    /// java: ProcessedOption.getDescription()
    fn get_description(&self) -> Arc<dyn StructuredText>;

    /// java: ProcessedOption.getSearchTags()
    fn get_search_tags(&self) -> Vec<Arc<dyn SearchTag>>;

    /// java: ProcessedOption.getType()
    fn get_type(&self) -> Type;

    /// java: ProcessedOption.get()
    fn get(&self) -> Arc<dyn std::any::Any>;

    /// java: ProcessedOption.set(Object value)
    fn set(&self, value: Arc<dyn std::any::Any>) -> bool;

    /// java: ProcessedOption.getDebugDeclarationLocation()
    fn get_debug_declaration_location(&self) -> Option<String>;
}

/// `implements ProcessedOption` for a concrete subclass: every member answered by its `DandelionProcessedOption`
/// method (the class's own or the abstract class's).
#[macro_export]
macro_rules! dandelion_processed_option {
    // (the arms that start with a token come first: a `$ty:ty` that fails to parse is an error, not a mismatch)
    (@impl [$($gen:tt)*] $ty:ty [$($w:tt)*]) => {
        impl $($gen)* ::moulconfig::io::github::notenoughupdates::moulconfig::processor::has_debug_location::HasDebugLocation for $ty $($w)* {
            fn get_debug_declaration_location(&self) -> ::core::option::Option<String> {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_debug_declaration_location(self)
            }
        }

        impl $($gen)* ::moulconfig::io::github::notenoughupdates::moulconfig::processor::processed_option::ProcessedOption for $ty $($w)* {
            fn get_search_tags(&self) -> Vec<std::sync::Arc<dyn ::moulconfig::io::github::notenoughupdates::moulconfig::annotations::search_tag::SearchTag>> {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_search_tags(self)
            }

            fn get_accordion_id(&self) -> i32 {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_accordion_id(self)
            }

            fn get_editor(&self) -> ::core::option::Option<std::sync::Arc<dyn ::moulconfig::io::github::notenoughupdates::moulconfig::gui::gui_option_editor::GuiOptionEditor>> {
                ::core::option::Option::Some($crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_editor(self))
            }

            fn get_category(&self) -> ::core::option::Option<std::sync::Arc<dyn ::moulconfig::io::github::notenoughupdates::moulconfig::processor::processed_category::ProcessedCategory>> {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_category(self)
            }

            fn get_name(&self) -> std::sync::Arc<dyn ::moulconfig::io::github::notenoughupdates::moulconfig::common::text::structured_text::StructuredText> {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_name(self)
            }

            fn get_description(&self) -> std::sync::Arc<dyn ::moulconfig::io::github::notenoughupdates::moulconfig::common::text::structured_text::StructuredText> {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_description(self)
            }

            fn get_path(&self) -> String {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_path(self)
            }

            fn get_config(&self) -> std::sync::Arc<dyn ::moulconfig::io::github::notenoughupdates::moulconfig::config::Config> {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_config(self)
            }

            fn get(&self) -> std::sync::Arc<dyn std::any::Any> {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get(self)
            }

            fn get_type(&self) -> ::moulconfig::io::github::notenoughupdates::moulconfig::processor::processed_option::Type {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::get_type(self)
            }

            fn set(&self, value: std::sync::Arc<dyn std::any::Any>) -> bool {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::set(self, value)
            }

            fn explicit_notify_change(&self) {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::explicit_notify_change(self)
            }

            fn this_option(&self) -> std::sync::Arc<dyn ::moulconfig::io::github::notenoughupdates::moulconfig::processor::processed_option::ProcessedOption> {
                $crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption::as_processed_option(self)
            }

            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
    (<$g:ident> $ty:ty where $($w:tt)+) => {
        $crate::dandelion_processed_option!(@impl [<$g>] $ty [where $($w)+]);
    };
    ($ty:ty) => {
        $crate::dandelion_processed_option!(@impl [] $ty []);
    };
}
