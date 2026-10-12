//! Ported from net.azureaaron.dandelion.impl.moulconfig.editor.DandelionColourEditor by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use mcrust::java::awt::color::Color;

// TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
//   java: private static final MyResourceLocation ALPHA_BUTTON = MoulConfigPlatform.wrap(Dandelion.id("textures/gui/button_alpha.png"));
/// java: private static final MyResourceLocation ALPHA_BUTTON = MoulConfigPlatform.wrap(Dandelion.id("textures/gui/button_alpha.png"));
pub static ALPHA_BUTTON: compat::MainThread<crate::unresolved::MyResourceLocation> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ALPHA_BUTTON` - see the TODO above"));
// TODO(fabric2seer): static call MoulConfigPlatform.wrap(..)
//   java: private static final MyResourceLocation ALPHA_BUTTON_OVERLAY = MoulConfigPlatform.wrap(Dandelion.id("textures/gui/button_alpha_overlay.png"));
/// java: private static final MyResourceLocation ALPHA_BUTTON_OVERLAY = MoulConfigPlatform.wrap(Dandelion.id("textures/gui/button_alpha_overlay.png"));
pub static ALPHA_BUTTON_OVERLAY: compat::MainThread<crate::unresolved::MyResourceLocation> = compat::MainThread::new(|| todo!("fabric2seer: the initialiser of `ALPHA_BUTTON_OVERLAY` - see the TODO above"));

/// java: public class DandelionColourEditor extends ComponentEditor
pub struct DandelionColourEditor {
    /// java: extends io.github.notenoughupdates.moulconfig.gui.editors.ComponentEditor
    pub base: crate::unresolved::ComponentEditor,
    /// java: private final GuiComponent component;
    pub component: Option<crate::unresolved::GuiComponent>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for DandelionColourEditor {
    type Target = crate::unresolved::ComponentEditor;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for DandelionColourEditor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl DandelionColourEditor {
    /// java: @SuppressWarnings("unchecked") public DandelionColourEditor(ProcessedOption option, boolean hasAlpha)
    pub fn new(mut option: crate::unresolved::ProcessedOption, mut has_alpha: bool) -> compat::Shared<Self> {
        // TODO(fabric2seer): `new ComponentEditor(..)`: the port has no `ComponentEditor::new`
        //   java: super(option);
        let __this: compat::Shared<Self> = todo!("constructor");
        // TODO(fabric2seer): `.intoProperty(..)` on a ProcessedOption
        //   java: this.component = this.wrapComponent(new ColourOverlayComponent(this, (GetSetter<Color>) option.intoProperty(), hasAlpha));
        __this
    }

    /// java: @Override public GuiComponent getDelegate()
    pub fn get_delegate(__this: &compat::Shared<Self>) -> crate::unresolved::GuiComponent {
        return compat::read(__this, |__t| __t.component.clone()).unwrap();
    }

}

// NOTE(fabric2seer): getDelegate override(s) of ComponentEditor - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them

/// java: protected static class ColourOverlayComponent extends GuiComponent
pub struct ColourOverlayComponent {
    /// java: extends io.github.notenoughupdates.moulconfig.gui.GuiComponent
    pub base: crate::unresolved::GuiComponent,
    /// java: private final DandelionColourEditor editor;
    pub editor: compat::Shared<DandelionColourEditor>,
    /// java: private final GetSetter<Color> getSetter;
    pub get_setter: crate::unresolved::GetSetter,
    /// java: private final boolean hasAlpha;
    pub has_alpha: bool,
}

impl std::ops::Deref for ColourOverlayComponent {
    type Target = crate::unresolved::GuiComponent;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for ColourOverlayComponent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ColourOverlayComponent {
    /// java: protected ColourOverlayComponent(DandelionColourEditor editor, GetSetter<Color> getSetter, boolean hasAlpha)
    pub fn new(mut editor: compat::Shared<DandelionColourEditor>, mut get_setter: crate::unresolved::GetSetter, mut has_alpha: bool) -> Self {
        let __init_editor = editor.clone();
        let __init_get_setter = get_setter.clone();
        let __init_has_alpha = has_alpha;
        // TODO(fabric2seer): `new GuiComponent(..)`: the port has no `GuiComponent::new`
        //   java:
        let mut this: Self = todo!("constructor");
        this
    }

    /// java: @Override public int getWidth()
    pub fn get_width(&self) -> i32 {
        return 48;
    }

    /// java: @Override public int getHeight()
    pub fn get_height(&self) -> i32 {
        return 16;
    }

    /// java: @Override public void render(GuiImmediateContext context)
    pub fn render(&self, context: &crate::unresolved::GuiImmediateContext) {
        // TODO(fabric2seer): `.get(..)` on a GetSetter
        //   java: int colour = this.getSetter.get().getRGB();
        let mut colour: i32 = todo!("fabric2seer: the initialiser of `colour` (TODO above)");
        // TODO(fabric2seer): `.getRenderContext(..)` on a GuiImmediateContext
        //   java: RenderContext renderContext = context.getRenderContext();
        let mut render_context: crate::unresolved::RenderContext = todo!("fabric2seer: the initialiser of `renderContext` (TODO above)");
        if self.has_alpha {
            // TODO(fabric2seer): `.drawComplexTexture(..)` on a RenderContext
            //   java: renderContext.drawComplexTexture(ALPHA_BUTTON, 0, 0, context.getWidth(), context.getHeight(), block -> {
            //   java: 					block.color(CommonColors.WHITE);
            //   java: 				});
            // TODO(fabric2seer): `.drawComplexTexture(..)` on a RenderContext
            //   java: renderContext.drawComplexTexture(ALPHA_BUTTON_OVERLAY, 0, 0, context.getWidth(), context.getHeight(), block -> {
            //   java: 					block.color(colour);
            //   java: 				});
        } else {
            // TODO(fabric2seer): `.drawComplexTexture(..)` on a RenderContext
            //   java: renderContext.drawComplexTexture(GuiTextures.BUTTON_WHITE, 0, 0, context.getWidth(), context.getHeight(), block -> {
            //   java: 					block.color(ARGB.opaque(colour));
            //   java: 				});
        }
    }

    /// java: @SuppressWarnings("deprecation") @Override public boolean mouseEvent(MouseEvent mouseEvent, GuiImmediateContext context)
    pub fn mouse_event(&self, mouse_event: &crate::unresolved::MouseEvent, context: &crate::unresolved::GuiImmediateContext) -> bool {
        // TODO(fabric2seer): `.getMouseState(..)` on a Click
        //   java: if (mouseEvent instanceof MouseEvent.Click click && click.getMouseState() && click.getMouseButton() == 0 && context.isHovered()) {
        //   java: 				ColorSelectComponent component = new ColorSelectComponent(0, 0, this.toChromaColour().toLegacyString(), this::fromChromaColourString, this.editor::closeOverlay, this.hasAlpha, true);
        //   java: 				//Clamp the Y so that the colour picker can't go off screen
        //   java: 				int scaledHeight = context.getRenderContext().getMinecraft().getScaledHeight();
        //   java: 				int clampedY;
        //   java:
        //   java: 				if (context.getAbsoluteMouseY() + component.getHeight() > scaledHeight) {
        //   java: 					clampedY = scaledHeight - component.getHeight();
        //   java: 				} else {
        //   java: 					clampedY = context.getAbsoluteMouseY();
        //   java: 				}
        //   java:
        //   java: 				this.editor.openOverlay(component, context.getAbsoluteMouseX(), clampedY);
        //   java: 				return true;
        //   java: 			}
        return false;
    }

    /// java: private ChromaColour toChromaColour()
    pub fn to_chroma_colour(&self) -> crate::unresolved::ChromaColour {
        // TODO(fabric2seer): `.get(..)` on a GetSetter
        //   java: Color colour = this.getSetter.get();
        let mut colour: Color = todo!("fabric2seer: the initialiser of `colour` (TODO above)");
        // TODO(fabric2seer): static call ChromaColour.fromStaticRGB(..)
        //   java: return ChromaColour.fromStaticRGB(colour.getRed(), colour.getGreen(), colour.getBlue(), colour.getAlpha());
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: private void fromChromaColourString(String colourString)
    pub fn from_chroma_colour_string(&mut self, colour_string: &str) {
        // TODO(fabric2seer): static call ChromaColour.specialToSimpleRGB(..)
        //   java: @SuppressWarnings("deprecation")
        //   java: 			Color colour = new Color(ChromaColour.specialToSimpleRGB(colourString), true);
        let mut colour: Color = todo!("fabric2seer: the initialiser of `colour` (TODO above)");
        // TODO(fabric2seer): `.set(..)` on a GetSetter
        //   java: this.getSetter.set(colour);
    }

}

// NOTE(fabric2seer): getHeight, getWidth, mouseEvent, render override(s) of GuiComponent - the port dispatches its own calls, so these run only where the mod calls them; hook the port's method to make the game call them
