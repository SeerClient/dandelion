//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionItemEditor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class DandelionItemEditor extends ComponentEditor
pub struct DandelionItemEditor {
    /// java: extends io.github.notenoughupdates.moulconfig.gui.editors.ComponentEditor
    pub base: crate::unresolved::ComponentEditor,
    /// java: private final GuiComponent component;
    pub component: Option<crate::unresolved::GuiComponent>,
    /// java: private String lastText = "";
    pub last_text: String,
}

impl std::ops::Deref for DandelionItemEditor {
    type Target = crate::unresolved::ComponentEditor;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DandelionItemEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DandelionItemEditor {
    /// java: public DandelionItemEditor(ProcessedOption option)
    pub fn new(mut option: crate::unresolved::ProcessedOption) -> Self {
        // TODO(fabric2seer): `new GetSetter(..) {..}` (an anonymous class with state or several methods)
        //   java: GetSetter<IItemStack> itemStackGetter = new GetSetter<>() {
        //   java: 			@Override
        //   java: 			public IItemStack get() {
        //   java: 				Item item = (Item) option.get();
        //   java: 				@SuppressWarnings("deprecation")
        //   java: 				Holder<Item> itemHolder = item.builtInRegistryHolder();
        //   java: 				ItemStack stack;
        //   java:
        //   java: 				if (itemHolder.isBound() && itemHolder.areComponentsBound()) {
        //   java: 					stack = new ItemStack(item);
        //   java: 				} else {
        //   java: 					stack = ItemStack.EMPTY;
        //   java: 				}
        //   java:
        //   java: 				return MoulConfigPlatform.wrap(stack);
        //   java: 			}
        //   java:
        //   java: 			@Override
        //   java: 			public void set(IItemStack newValue) {
        //   java: 				throw new UnsupportedOperationException();
        //   java: 			}
        //   java: 		};
        let mut item_stack_getter: crate::unresolved::GetSetter = todo!("fabric2seer: the initialiser of `itemStackGetter` (TODO above)");
        // TODO(fabric2seer): `new GetSetter(..) {..}` (an anonymous class with state or several methods)
        //   java: GetSetter<String> itemStackSetter = new GetSetter<>() {
        //   java: 			@Override
        //   java: 			public String get() {
        //   java: 				return DandelionItemEditor.this.lastText;
        //   java: 			}
        //   java:
        //   java: 			@Override
        //   java: 			public void set(String newValue) {
        //   java: 				DandelionItemEditor.this.lastText = newValue;
        //   java: 				Identifier id = Identifier.tryParse(newValue);
        //   java:
        //   java: 				//Only update the value when a valid item is has been typed in
        //   java: 				if (id != null) {
        //   java: 					option.set(BuiltInRegistries.ITEM.getValue(id));
        //   java: 				}
        //   java: 			}
        //   java: 		};
        let mut item_stack_setter: crate::unresolved::GetSetter = todo!("fabric2seer: the initialiser of `itemStackSetter` (TODO above)");
        // TODO(fabric2seer): `.get(..)` on a ProcessedOption
        //   java: this.lastText = BuiltInRegistries.ITEM.getKey((Item) option.get()).toString();
        // TODO(fabric2seer): `new ComponentEditor(..)`: the port has no `ComponentEditor::new`
        //   java: super(option);
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): `new ColumnComponent(..)`
        //   java: this.component = this.wrapComponent(
        //   java: 				new ColumnComponent(
        //   java: 						new CenterComponent(new ItemStackComponent(itemStackGetter)),
        //   java: 						new TextFieldComponent(
        //   java: 								itemStackSetter,
        //   java: 								80,
        //   java: 								() -> true,
        //   java: 								"Item ID",
        //   java: 								IMinecraft.INSTANCE.getDefaultFontRenderer(),
        //   java: 								Set.of('§')
        //   java: 								)
        //   java: 						)
        //   java: 				);
        this
    }

    /// java: @Override public GuiComponent getDelegate()
    pub fn get_delegate(&self) -> crate::unresolved::GuiComponent {
        return self.component.clone().unwrap();
    }

}

// NOTE(fabric2seer): getDelegate override(s) of ComponentEditor - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
