//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionNumberSliderEditor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

// TODO(fabric2seer): static call NumberFormat.getInstance(..)
//   java: private static final NumberFormat FORMATTER = Util.make(NumberFormat.getInstance(Locale.CANADA), nf -> nf.setMaximumFractionDigits(2));
/// java: private static final NumberFormat FORMATTER = Util.make(NumberFormat.getInstance(Locale.CANADA), nf -> nf.setMaximumFractionDigits(2));
pub static FORMATTER: compat::MainThread<crate::unresolved::NumberFormat> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `FORMATTER` - see the TODO above"));

/// java: public class DandelionNumberSliderEditor extends ComponentEditor
pub struct DandelionNumberSliderEditor {
    /// java: extends io.github.notenoughupdates.moulconfig.gui.editors.ComponentEditor
    pub base: crate::unresolved::ComponentEditor,
    /// java: protected final GuiComponent component;
    pub component: Option<crate::unresolved::GuiComponent>,
}

impl std::ops::Deref for DandelionNumberSliderEditor {
    type Target = crate::unresolved::ComponentEditor;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DandelionNumberSliderEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DandelionNumberSliderEditor {
    /// java: @SuppressWarnings("unchecked") public DandelionNumberSliderEditor(ProcessedOption option, float minValue, float maxValue, float minStep)
    pub fn new(mut option: crate::unresolved::ProcessedOption, mut min_value: f32, mut max_value: f32, mut min_step: f32) -> Self {
        // TODO(fabric2seer): `new ComponentEditor(..)`: the port has no `ComponentEditor::new`
        //   java: super(option);
        let mut this: Self = todo!("constructor");
        // TODO(fabric2seer): `new RowComponent(..)`
        //   java: this.component = this.wrapComponent(
        //   java: 				new RowComponent(
        //   java: 						new AlignComponent(
        //   java: 								new TextComponent(IMinecraft.INSTANCE.getDefaultFontRenderer(),
        //   java: 										() -> StructuredText.of(FORMATTER.format(((Float) option.get()).doubleValue())),
        //   java: 										20,
        //   java: 										TextComponent.TextAlignment.CENTER, false, false),
        //   java: 								GetSetter.constant(HorizontalAlign.CENTER),
        //   java: 								GetSetter.constant(VerticalAlign.CENTER)
        //   java: 								),
        //   java: 						new SliderComponent((GetSetter<Float>) option.intoProperty(), minValue, maxValue, minStep, 40)
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
