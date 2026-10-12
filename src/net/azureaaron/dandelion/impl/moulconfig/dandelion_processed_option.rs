//! Ported from net.azureaaron.dandelion.impl.moulconfig.DandelionProcessedOption by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public abstract class DandelionProcessedOption implements ProcessedOption
pub struct DandelionProcessedOptionBase {
    /// java: private final int accordionId;
    pub accordion_id: i32,
    /// java: private final Config config;
    pub config: crate::unresolved::Config,
    /// java: protected ProcessedCategory category;
    pub category: Option<crate::unresolved::ProcessedCategory>,
    /// java: protected @Nullable GuiOptionEditor editor;
    pub editor: Option<crate::unresolved::GuiOptionEditor>,
}

impl DandelionProcessedOptionBase {
    /// java: protected DandelionProcessedOption(int accordionId, Config config) (built by the most-derived class's `wrap`, then run on its handle)
    pub fn new_in<__R>(mut accordion_id: i32, mut config: crate::unresolved::Config, __wrap: impl FnOnce(Self) -> (compat::Shared<dyn DandelionProcessedOption>, __R)) -> (compat::Shared<dyn DandelionProcessedOption>, __R) {
        let __init_accordion_id = accordion_id;
        let __init_config = config.clone();
        let this = Self {
            accordion_id: __init_accordion_id,
            config: __init_config,
            category: None,
            editor: None,
        };
        let (__h, __r) = __wrap(this);
        let __this = __h;
        (__this, __r)
    }

}

/// java: the virtual methods of DandelionProcessedOption (implemented by DandelionProcessedOptionBase and every subclass)
pub trait DandelionProcessedOption: compat::DynAny {
    fn dandelion_processed_option_base(&self) -> &DandelionProcessedOptionBase;
    fn dandelion_processed_option_base_mut(&mut self) -> &mut DandelionProcessedOptionBase;
    fn as_any(&self) -> &dyn std::any::Any;

    /// java: @Override public int getAccordionId() (dispatch: the body for this class)
    fn get_accordion_id(&self) -> fn(&compat::Shared<dyn DandelionProcessedOption>) -> i32 {
        <dyn DandelionProcessedOption>::get_accordion_id_body
    }

    /// java: protected abstract GuiOptionEditor createEditor(); (dispatch: the body for this class)
    fn create_editor(&self) -> fn(&compat::Shared<dyn DandelionProcessedOption>) -> crate::unresolved::GuiOptionEditor;

    /// java: @Override public GuiOptionEditor getEditor() (dispatch: the body for this class)
    fn get_editor(&self) -> fn(&compat::Shared<dyn DandelionProcessedOption>) -> crate::unresolved::GuiOptionEditor {
        <dyn DandelionProcessedOption>::get_editor_body
    }

    /// java: @Override public ProcessedCategory getCategory() (dispatch: the body for this class)
    fn get_category(&self) -> fn(&compat::Shared<dyn DandelionProcessedOption>) -> crate::unresolved::ProcessedCategory {
        <dyn DandelionProcessedOption>::get_category_body
    }

    /// java: @Override public String getPath() (dispatch: the body for this class)
    fn get_path(&self) -> fn(&compat::Shared<dyn DandelionProcessedOption>) -> String {
        <dyn DandelionProcessedOption>::get_path_body
    }

    /// java: @Override public Config getConfig() (dispatch: the body for this class)
    fn get_config(&self) -> fn(&compat::Shared<dyn DandelionProcessedOption>) -> crate::unresolved::Config {
        <dyn DandelionProcessedOption>::get_config_body
    }

    /// java: @Override public void explicitNotifyChange() (dispatch: the body for this class)
    fn explicit_notify_change(&self) -> fn(&compat::Shared<dyn DandelionProcessedOption>) {
        <dyn DandelionProcessedOption>::explicit_notify_change_body
    }

}

impl dyn DandelionProcessedOption {
    /// java: @Override public int getAccordionId()
    pub fn get_accordion_id_body(__this: &compat::Shared<dyn DandelionProcessedOption>) -> i32 {
        return compat::read(__this, |__t| __t.dandelion_processed_option_base().accordion_id);
    }

    /// java: @Override public GuiOptionEditor getEditor()
    pub fn get_editor_body(__this: &compat::Shared<dyn DandelionProcessedOption>) -> crate::unresolved::GuiOptionEditor {
        if compat::read(__this, |__t| __t.dandelion_processed_option_base().editor.clone()).is_none() {
            { let __v = Some(({ let __f = compat::dispatch(__this, |__t| __t.create_editor()); __f(__this) })); compat::write(__this, |__t| __t.dandelion_processed_option_base_mut().editor = __v); }
        }
        return compat::read(__this, |__t| __t.dandelion_processed_option_base().editor.clone()).unwrap();
    }

    /// java: @Override public ProcessedCategory getCategory()
    pub fn get_category_body(__this: &compat::Shared<dyn DandelionProcessedOption>) -> crate::unresolved::ProcessedCategory {
        return compat::read(__this, |__t| __t.dandelion_processed_option_base().category.clone()).unwrap();
    }

    /// java: @Override public String getPath()
    pub fn get_path_body(__this: &compat::Shared<dyn DandelionProcessedOption>) -> String {
        return "NYI".to_string();
    }

    /// java: @Override public Config getConfig()
    pub fn get_config_body(__this: &compat::Shared<dyn DandelionProcessedOption>) -> crate::unresolved::Config {
        return compat::read(__this, |__t| __t.dandelion_processed_option_base().config.clone());
    }

    /// java: @Override public void explicitNotifyChange()
    pub fn explicit_notify_change_body(__this: &compat::Shared<dyn DandelionProcessedOption>) {
    }

}
