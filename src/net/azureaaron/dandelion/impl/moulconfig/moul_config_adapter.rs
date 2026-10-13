//! Ported from net.azureaaron.dandelion.impl.moulconfig.MoulConfigAdapter.
//!
//! rust: port gap (GAPS_D.md D1) - MoulConfig's editor and screen (`MoulConfigEditor`, `MoulConfigScreenComponent`,
//! `GuiContext`, `GuiElementComponent`) are not ported, so the screen cannot be built; the option processing
//! before it is.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::config_category::ConfigCategory;
use crate::net::azureaaron::dandelion::api::config_manager::AnyConfigManager;
use crate::net::azureaaron::dandelion::api::option::AnyOption;
use crate::net::azureaaron::dandelion::api::platform_links::PlatformLinks;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_category::DandelionProcessedCategory;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_group_option::DandelionProcessedGroupOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::dandelion_processed_option::DandelionProcessedOption;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_definition::MoulConfigDefinition;
use crate::net::azureaaron::dandelion::r#impl::moulconfig::moul_config_editable_option_adapter::EditableOptionFactory;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final int ROOT_ACCORDION = -1; - the id of the accordion which is at the root of each category
pub const ROOT_ACCORDION: i32 = -1;

/// java: public class MoulConfigAdapter
pub struct MoulConfigAdapter {
    /// java: private final Component title;
    pub title: MutableComponent,
    /// java: private final MoulConfigDefinition configDefinition;
    pub config_definition: Arc<MoulConfigDefinition>,
    /// java: private final Map<Option<?>, BiFunction<..>> editableOptionFactories = new LinkedHashMap<>();
    ///
    /// Keyed by the option's identity (an option does not override `equals`), in insertion order.
    pub editable_option_factories: Vec<(Arc<dyn AnyOption>, EditableOptionFactory)>,
}

impl MoulConfigAdapter {
    /// java: public MoulConfigAdapter(ConfigManager<?> manager, Component title, @Nullable PlatformLinks platformLinks)
    pub fn new(manager: Arc<dyn AnyConfigManager>, title: MutableComponent, platform_links: Option<Arc<dyn PlatformLinks>>) -> Self {
        let config_definition = Arc::new(MoulConfigDefinition::new(manager, title.clone(), platform_links));
        Self { title, config_definition, editable_option_factories: Vec::new() }
    }

    /// java: public Screen generateMoulConfigScreen(List<ConfigCategory> categories, @Nullable Screen parent, String search)
    pub fn generate_moul_config_screen(mut self, categories: &[Arc<dyn ConfigCategory>], parent: Option<compat::ScreenRef>, search: &str) -> compat::ScreenRef {
        self.generate_editable_options(categories);
        let processed_categories: Vec<Arc<DandelionProcessedCategory>> = self.generate_processed_categories(categories);
        //   java: MoulConfigEditor<MoulConfigDefinition> editor = new MoulConfigEditor<>(ProcessedCategory.collect(processedCategories), this.configDefinition);
        //   java: MoulConfigScreenComponent moulConfigScreenComponent = new MoulConfigScreenComponent(this.title, new GuiContext(new GuiElementComponent(editor)), parent);
        //   java: moulConfigScreenComponent.getGuiContext().setCloseRequestHandler(() -> {
        //   java:     Screens.getMinecraft(moulConfigScreenComponent).gui.setScreen(moulConfigScreenComponent.getPreviousScreen());
        //   java:     this.configDefinition.saveNow();
        //   java: });
        //   java: if (!search.isEmpty()) { editor.search(search); }
        //   java: return moulConfigScreenComponent;
        let _ = (processed_categories, parent, search);
        todo!("fabric2seer: `new MoulConfigEditor(..)` / `new MoulConfigScreenComponent(..)` - MoulConfig is not ported (GAPS_D.md D1)")
    }

    /// java: private void generateEditableOptions(List<ConfigCategory> categories)
    fn generate_editable_options(&mut self, categories: &[Arc<dyn ConfigCategory>]) {
        let all_root_options = categories.iter().filter_map(|category| category.root_group()).flat_map(|group| group.options());
        let all_grouped_options = categories.iter().flat_map(|category| category.groups()).flat_map(|group| group.options());
        let all_options: Vec<Arc<dyn AnyOption>> = all_root_options.chain(all_grouped_options).collect();
        for option in all_options {
            let factory = option.clone().create_editable_option_factory();
            // java: Map.put - a second put of the same option replaces its factory in place
            match self.editable_option_factories.iter_mut().find(|(o, _)| Arc::ptr_eq(o, &option)) {
                Some(entry) => entry.1 = factory,
                None => self.editable_option_factories.push((option, factory)),
            }
        }
    }

    /// java: private List<DandelionProcessedCategory> generateProcessedCategories(List<ConfigCategory> categories)
    fn generate_processed_categories(&self, categories: &[Arc<dyn ConfigCategory>]) -> Vec<Arc<DandelionProcessedCategory>> {
        categories
            .iter()
            .map(|category| {
                let mut processed_options: Vec<Arc<dyn DandelionProcessedOption>> = Vec::new();
                let mut next_accordion_id: i32 = 1717;
                // Add root group as options to the category under the root accordion
                if let Some(root_group) = category.root_group() {
                    self.add_processed_options_to_group(&mut processed_options, &root_group.options(), ROOT_ACCORDION);
                }
                // For each option group, create an accordion option and then add each of its options to said accordion
                for group in category.groups() {
                    let group_accordion_id = next_accordion_id;
                    next_accordion_id += 1;
                    // Create the accordion/group option under the root accordion
                    processed_options.push(Arc::new(DandelionProcessedGroupOption::new(group.clone(), ROOT_ACCORDION, group_accordion_id, self.config_definition.clone())));
                    // Add the options in this group under the generated accordion (grouping)
                    self.add_processed_options_to_group(&mut processed_options, &group.options(), group_accordion_id);
                }
                // Return the newly created category holding all of our options
                DandelionProcessedCategory::new(category.clone(), processed_options)
            })
            .collect()
    }

    /// java: private void addProcessedOptionsToGroup(List<DandelionProcessedOption> processedOptions,
    /// List<? extends Option<?>> options, int groupAccordionId)
    fn add_processed_options_to_group(&self, processed_options: &mut Vec<Arc<dyn DandelionProcessedOption>>, options: &[Arc<dyn AnyOption>], group_accordion_id: i32) {
        for option in options {
            // Each option should be in the map unless its not supported (at which point an exception would be thrown
            // when attempting to create the factory)
            let factory = self
                .editable_option_factories
                .iter()
                .find(|(o, _)| Arc::ptr_eq(o, option))
                .map(|(_, f)| f.clone())
                .expect("NullPointerException: no editable option factory for the option");
            processed_options.push(factory(group_accordion_id, self.config_definition.clone()));
        }
    }
}
