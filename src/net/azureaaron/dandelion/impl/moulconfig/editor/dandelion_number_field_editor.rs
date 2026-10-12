//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionNumberFieldEditor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public class DandelionNumberFieldEditor extends ComponentEditor
pub struct DandelionNumberFieldEditor {
    /// java: extends io.github.notenoughupdates.moulconfig.gui.editors.ComponentEditor
    pub base: crate::unresolved::ComponentEditor,
    /// java: private final GuiComponent component;
    pub component: Option<crate::unresolved::GuiComponent>,
    /// java: private String lastText = "";
    pub last_text: String,
}

impl std::ops::Deref for DandelionNumberFieldEditor {
    type Target = crate::unresolved::ComponentEditor;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DandelionNumberFieldEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DandelionNumberFieldEditor {
    /// java: public DandelionNumberFieldEditor(ProcessedOption option, float minValue, float maxValue, float defaultValue)
    pub fn new(mut option: crate::unresolved::ProcessedOption, mut min_value: f32, mut max_value: f32, mut default_value: f32) -> Self {
        // TODO(fabric2seer): `new GetSetter(..) {..}` (an anonymous class with state or several methods)
        //   java: GetSetter<String> getSetter = new GetSetter<>() {
        //   java: 			@Override
        //   java: 			public String get() {
        //   java: 				return DandelionNumberFieldEditor.this.lastText;
        //   java: 			}
        //   java:
        //   java: 			@Override
        //   java: 			public void set(String newValue) {
        //   java: 				DandelionNumberFieldEditor.this.lastText = newValue;
        //   java: 				option.set(parseNumber(newValue, minValue, maxValue, defaultValue));
        //   java: 			}
        //   java: 		};
        let mut get_setter: crate::unresolved::GetSetter = todo!("fabric2seer: the initialiser of `getSetter` (TODO above)");
        // TODO(fabric2seer): `new ComponentEditor(..)`: the port has no `ComponentEditor::new`
        //   java: super(option);
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): `.get(..)` on a ProcessedOption
        //   java: this.lastText = toString((Float) option.get());
        // TODO(fabric2seer): `new TextFieldComponent(..)`
        //   java: this.component = this.wrapComponent(new TextFieldComponent(
        //   java: 				getSetter,
        //   java: 				80,
        //   java: 				() -> true,
        //   java: 				"",
        //   java: 				IMinecraft.INSTANCE.getDefaultFontRenderer(),
        //   java: 				Set.of('§')));
        this
    }

    /// java: @ApiStatus.Experimental protected String toString(Float floatValue)
    pub fn to_string(&self, mut float_value: f32) -> String {
        let mut int_value: i32 = (float_value) as i32;
        let mut converted_back_to_float: f32 = ((int_value) as f32);
        if converted_back_to_float == float_value {
            return int_value.to_string();
        } else {
            return compat::fstr(float_value as f64);
        }
    }

    /// java: @ApiStatus.Experimental protected float parseNumber(String input, float minValue, float maxValue, float defaultValue)
    pub fn parse_number(&self, input: &str, mut min_value: f32, mut max_value: f32, mut default_value: f32) -> f32 {
        // java: try {..} catch (NumberFormatException _) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return (input.trim().parse::<f32>().expect("NumberFormatException")).clamp(min_value, max_value);
        })) {
            Ok(__v) => return __v,
            Err(_) => {
                return default_value;
            }
        }
    }

    /// java: @Override public GuiComponent getDelegate()
    pub fn get_delegate(&self) -> crate::unresolved::GuiComponent {
        return self.component.clone().unwrap();
    }

}

// NOTE(fabric2seer): getDelegate override(s) of ComponentEditor - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
