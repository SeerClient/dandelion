//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionItemEditor.
//!
//! An `Item` option value is `compat::JavaRef<Item>` (as YACL's `ItemTypeAdapter` has it).

#[allow(unused_imports)]
use crate::prelude::*;
use ::moulconfig::io::github::notenoughupdates::moulconfig as moul;
use ::yacl::dev::isxander::yacl3::gui::utils::item_registry_helper::ItemRegistryHelper;
use mcrust::net::minecraft::core::defaulted_registry::DefaultedRegistry as _;
use mcrust::net::minecraft::core::registries::built_in_registries;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::world::item::item::Item;
use mcrust::net::minecraft::world::item::item_stack;
use mcrust::net::minecraft::world::item::items;
use moul::common::i_item_stack::IItemStack;
use moul::common::i_minecraft;
use moul::gui::component::center_component::CenterComponent;
use moul::gui::component::column_component::ColumnComponent;
use moul::gui::component::item_stack_component::ItemStackComponent;
use moul::gui::component::text_field_component::TextFieldComponent;
use moul::gui::editors::component_editor::{self, ComponentEditor, ComponentEditorBase};
use moul::gui::gui_component::GuiComponent;
use moul::gui::gui_option_editor::GuiOptionEditor;
use moul::observer::get_setter;
use moul::platform::moul_config_platform::MoulConfigPlatform;
use moul::processor::processed_option::ProcessedOption;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashSet;

/// `(Item) option.get()`
fn item_of(option: &Arc<dyn ProcessedOption>) -> compat::JavaRef<Item> {
    option.get().downcast_ref::<compat::JavaRef<Item>>().cloned().expect("ClassCastException: the option's value is not an Item")
}

/// java: public class DandelionItemEditor extends ComponentEditor
pub struct DandelionItemEditor {
    base: ComponentEditorBase,
    /// java: private final GuiComponent component;
    component: Arc<dyn GuiComponent>,
    /// java: private String lastText = "";
    ///
    /// This saves the last input, this is because the text field component expects that the value that gets input
    /// will be written to the backing option/field immediately, and then following that it will pull the field value
    /// into the text box right after. Since we only save something if it is valid, we need to store the last text and
    /// then feed that back to the text field to ensure the input does not get stuck at the default value
    /// (minecraft:air)/become unusable.
    last_text: Arc<RefCell<String>>,
}

impl DandelionItemEditor {
    /// java: public DandelionItemEditor(ProcessedOption option)
    pub fn new(option: Arc<dyn ProcessedOption>) -> Arc<Self> {
        // java: GetSetter<IItemStack> itemStackGetter = new GetSetter<>() {..}
        let get_option = option.clone();
        let item_stack_getter = get_setter::of(
            move || -> Arc<dyn IItemStack> {
                let item = item_of(&get_option);
                // java: @SuppressWarnings("deprecation") Holder<Item> itemHolder = item.builtInRegistryHolder();
                let item_holder = item.built_in_registry_holder();
                let stack = if item_holder.is_bound() && item_holder.are_components_bound() {
                    item_stack::ItemStack::new(&item.0)
                } else {
                    item_stack::empty()
                };
                MoulConfigPlatform::wrap_item(stack)
            },
            |_new_value: Arc<dyn IItemStack>| panic!("UnsupportedOperationException"),
        );
        // java: GetSetter<String> itemStackSetter = new GetSetter<>() {..}
        let last_text = Arc::new(RefCell::new(String::new()));
        let (get_text, set_text) = (last_text.clone(), last_text.clone());
        let set_option = option.clone();
        let item_stack_setter = get_setter::of(
            move || get_text.borrow().clone(),
            move |new_value: String| {
                *set_text.borrow_mut() = new_value.clone();
                let id = Identifier::try_parse(&new_value);

                //Only update the value when a valid item is has been typed in
                if let Some(id) = id {
                    // java: option.set(BuiltInRegistries.ITEM.getValue(id)) - the defaulted registry answers air for an
                    // unknown id
                    let item = ItemRegistryHelper::get_from_registry(&id).unwrap_or_else(|| compat::JavaRef::new(items::air()));
                    set_option.set(Arc::new(item));
                }
            },
        );

        // java: this.lastText = BuiltInRegistries.ITEM.getKey((Item) option.get()).toString();
        *last_text.borrow_mut() = built_in_registries::item().get_key_defaulted(&item_of(&option).erased_obj()).to_string();
        let component = component_editor::wrap_component(
            &option,
            ColumnComponent::new(vec![
                CenterComponent::new(ItemStackComponent::new(item_stack_getter)) as Arc<dyn GuiComponent>,
                TextFieldComponent::new(
                    item_stack_setter,
                    80,
                    Arc::new(|| true),
                    "Item ID",
                    i_minecraft::instance().get_default_font_renderer(),
                    HashSet::from(['\u{00A7}']),
                ) as Arc<dyn GuiComponent>,
            ]),
        );
        Arc::new(Self { base: ComponentEditorBase::new(option), component, last_text })
    }
}

impl ComponentEditor for DandelionItemEditor {
    fn component_editor_base(&self) -> &ComponentEditorBase {
        &self.base
    }

    /// java: @Override public GuiComponent getDelegate()
    fn get_delegate(&self) -> Arc<dyn GuiComponent> {
        self.component.clone()
    }
}

impl GuiOptionEditor for DandelionItemEditor {
    ::moulconfig::component_editor_overrides!();

    fn get_height(&self) -> i32 {
        component_editor::get_height(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
