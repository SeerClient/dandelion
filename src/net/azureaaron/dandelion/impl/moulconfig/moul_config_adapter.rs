//! Ported from net.azureaaron.dandelion.impl.moulconfig.MoulConfigAdapter by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::ConfigManager;
use crate::net::azureaaron::dandelion::api::option_group::OptionGroup;
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_category::DandelionProcessedCategory;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_editable_option::DandelionProcessedEditableOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_group_option::DandelionProcessedGroupOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_editable_option_adapter::MoulConfigEditableOptionAdapter;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int ROOT_ACCORDION = -1;
pub const ROOT_ACCORDION: i32 = -1;

/// java: public class MoulConfigAdapter
#[derive(Clone)]
pub struct MoulConfigAdapter {
    /// java: private final Component title;
    pub title: MutableComponent,
    /// java: private final MoulConfigDefinition configDefinition;
    pub config_definition: compat::Shared<MoulConfigDefinition>,
    /// java: private final Map<Option<?>, BiFunction<Integer, MoulConfigDefinition, DandelionProcessedEditableOption<?>>> editableOptionFactories = new LinkedHashMap<>();
    pub editable_option_factories: std::collections::HashMap<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>, Arc<dyn Fn(i32, compat::Shared<MoulConfigDefinition>) -> compat::Shared<dyn DandelionProcessedEditableOption<compat::Object>>>>,
}

impl MoulConfigAdapter {
    /// java: public MoulConfigAdapter(ConfigManager<?> manager, Component title, @Nullable PlatformLinks platformLinks)
    pub fn new(mut manager: compat::Shared<dyn ConfigManager<compat::Object>>, mut title: MutableComponent, mut platform_links: Option<compat::Shared<dyn PlatformLinks>>) -> Self {
        let __init_title = title.clone();
        let __init_config_definition = MoulConfigDefinition::new(manager.clone(), title.clone(), platform_links.clone());
        let mut this = Self {
            title: __init_title,
            config_definition: __init_config_definition,
            editable_option_factories: std::collections::HashMap::new(),
        };
        this
    }

    /// java: public Screen generateMoulConfigScreen(List<ConfigCategory> categories, @Nullable Screen parent, String search)
    pub fn generate_moul_config_screen(&mut self, categories: &Vec<compat::Shared<dyn ConfigCategory>>, mut parent: Option<mcrust::net::minecraft::client::gui::screens::screen::Screen>, search: &str) -> mcrust::net::minecraft::client::gui::screens::screen::Screen {
        self.generate_editable_options(categories);
        let mut processed_categories: Vec<compat::Shared<DandelionProcessedCategory>> = self.generate_processed_categories(categories);
        // TODO(fabric2seer): `new MoulConfigEditor(..)`
        //   java: MoulConfigEditor<MoulConfigDefinition> editor = new MoulConfigEditor<>(ProcessedCategory.collect(processedCategories), this.configDefinition);
        let mut editor: crate::unresolved::MoulConfigEditor = todo!("fabric2seer: the initialiser of `editor` (TODO above)");
        // TODO(fabric2seer): `new MoulConfigScreenComponent(..)`
        //   java: MoulConfigScreenComponent moulConfigScreenComponent = new MoulConfigScreenComponent(this.title, new GuiContext(new GuiElementComponent(editor)), parent);
        let mut moul_config_screen_component: crate::unresolved::MoulConfigScreenComponent = todo!("fabric2seer: the initialiser of `moulConfigScreenComponent` (TODO above)");
        // TODO(fabric2seer): `.getGuiContext(..)` on a MoulConfigScreenComponent
        //   java: moulConfigScreenComponent.getGuiContext().setCloseRequestHandler(() -> {
        //   java: 			Screens.getMinecraft(moulConfigScreenComponent).gui.setScreen(moulConfigScreenComponent.getPreviousScreen());
        //   java: 			this.configDefinition.saveNow();
        //   java: 		});
        if !search.is_empty() {
            // TODO(fabric2seer): `.search(..)` on a MoulConfigEditor
            //   java: editor.search(search);
        }
        return moul_config_screen_component;
    }

    /// java: private void generateEditableOptions(List<ConfigCategory> categories)
    pub fn generate_editable_options(&mut self, categories: &Vec<compat::Shared<dyn ConfigCategory>>) {
        let mut all_root_options = categories.iter().cloned().map(|__a0| ({ let __f = compat::dispatch(&__a0, |__t| __t.root_group()); __f(&__a0) })).filter(|__a0| true).flat_map(|group| ({ let __f = compat::dispatch(&group, |__t| __t.options()); __f(&group) }).into_iter());
        let mut all_grouped_options = categories.iter().cloned().flat_map(|category| ({ let __f = compat::dispatch(&category, |__t| __t.groups()); __f(&category) }).into_iter()).flat_map(|group| ({ let __f = compat::dispatch(&group, |__t| __t.options()); __f(&group) }).into_iter());
        let mut all_options: Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>> = all_root_options.chain(all_grouped_options).collect::<Vec<_>>();
        for option in all_options.iter() {
            self.editable_option_factories.insert(option.clone(), MoulConfigEditableOptionAdapter::create_editable_option_factory(option));
        }
    }

    /// java: private List<DandelionProcessedCategory> generateProcessedCategories(List<ConfigCategory> categories)
    pub fn generate_processed_categories(&self, categories: &Vec<compat::Shared<dyn ConfigCategory>>) -> Vec<compat::Shared<DandelionProcessedCategory>> {
        return categories.iter().cloned().map(|category| {
            let mut processed_options: Vec<compat::Shared<dyn DandelionProcessedOption>> = Vec::new();
            let mut next_accordion_id: i32 = 1717;
            if ({ let __f = compat::dispatch(&category, |__t| __t.root_group()); __f(&category) }).is_some() {
                self.add_processed_options_to_group(&mut processed_options, &({ let __r = &({ let __f = compat::dispatch(&category, |__t| __t.root_group()); __f(&category) }).unwrap(); let __f = compat::dispatch(__r, |__t| __t.options()); __f(__r) }), ROOT_ACCORDION);
            }
            for group in ({ let __f = compat::dispatch(&category, |__t| __t.groups()); __f(&category) }) {
                let mut group_accordion_id: i32 = { let __v = next_accordion_id; next_accordion_id += 1; __v };
                processed_options.push((DandelionProcessedGroupOption::new(group.clone(), ROOT_ACCORDION, group_accordion_id, self.config_definition.clone()) as compat::Shared<dyn DandelionProcessedOption>));
                self.add_processed_options_to_group(&mut processed_options, &({ let __f = compat::dispatch(&group, |__t| __t.options()); __f(&group) }), group_accordion_id);
            }
            return DandelionProcessedCategory::new(category.clone(), processed_options.clone());
        }).collect::<Vec<_>>();
    }

    /// java: private void addProcessedOptionsToGroup(List<DandelionProcessedOption> processedOptions, List<? extends Option<?>> options, int groupAccordionId)
    pub fn add_processed_options_to_group(&self, processed_options: &mut Vec<compat::Shared<dyn DandelionProcessedOption>>, options: &Vec<compat::Shared<dyn crate::net::azureaaron::dandelion::api::option::Option<compat::Object>>>, mut group_accordion_id: i32) {
        for option in options.iter() {
            let mut processed_option: compat::Shared<dyn DandelionProcessedEditableOption<compat::Object>> = (self.editable_option_factories.get(option).cloned().unwrap())(group_accordion_id, self.config_definition.clone());
            processed_options.push((processed_option.clone() as compat::Shared<dyn DandelionProcessedOption>));
        }
    }

}
