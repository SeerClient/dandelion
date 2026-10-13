//! Types the mod uses that neither the mod, the JDK mapping nor the port has (fabric2seer).
//! Each is a placeholder: give it the real type (a library crate, or port the class).
//!
//! rust: port gap (GAPS_D.md D1) - MoulConfig (`io.github.notenoughupdates.moulconfig`, shaded by Dandelion) is not
//! ported. A placeholder is a reference to an object of the Java class or of any subclass (`of`), so the Dandelion
//! classes that extend one (`DandelionBlockedOptionEditor extends GuiOptionEditor`, ..) can be handed where Java
//! widens to it; `as_any` is the `instanceof` test.

/// A reference to an object of an unported Java class (or of a subclass of it); `Default` is `null`.
macro_rules! placeholder_class {
    ($name:ident) => {
        #[derive(Clone, Default)]
        pub struct $name(pub Option<std::sync::Arc<dyn std::any::Any>>);

        impl $name {
            /// the object as a reference of this class (Java's widening)
            pub fn of<T: std::any::Any>(object: std::sync::Arc<T>) -> $name {
                $name(Some(object))
            }

            /// `instanceof` / a cast: the object behind the reference (`()` for `null`)
            pub fn as_any(&self) -> &dyn std::any::Any {
                match &self.0 {
                    Some(object) => &**object,
                    None => &(),
                }
            }
        }
    };
}

/// TODO(fabric2seer): java java.lang.annotation.Annotation
placeholder_class!(Annotation);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.ChromaColour
placeholder_class!(ChromaColour);
/// TODO(fabric2seer): java java.lang.Class
placeholder_class!(Class);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.MouseEvent.Click
placeholder_class!(Click);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.editors.ComponentEditor
placeholder_class!(ComponentEditor);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.Config
placeholder_class!(Config);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.DescriptionRendereringBehaviour
placeholder_class!(DescriptionRendereringBehaviour);
/// TODO(fabric2seer): java java.lang.Enum
placeholder_class!(Enum);
/// TODO(fabric2seer): java java.lang.reflect.Field
placeholder_class!(Field);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.observer.GetSetter
placeholder_class!(GetSetter);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.GuiComponent
placeholder_class!(GuiComponent);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.GuiImmediateContext
placeholder_class!(GuiImmediateContext);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.GuiOptionEditor
placeholder_class!(GuiOptionEditor);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.editors.GuiOptionEditorAccordion
placeholder_class!(GuiOptionEditorAccordion);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.common.IFontRenderer
placeholder_class!(IFontRenderer);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.common.IItemStack
placeholder_class!(IItemStack);
/// TODO(fabric2seer): java com.mojang.blaze3d.platform.InputConstants
placeholder_class!(InputConstants);
/// TODO(fabric2seer): java com.google.gson.JsonDeserializationContext
placeholder_class!(JsonDeserializationContext);
/// TODO(fabric2seer): java com.google.gson.JsonSerializationContext
placeholder_class!(JsonSerializationContext);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.KeyboardEvent.KeyPressed
placeholder_class!(KeyPressed);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.KeyboardEvent
placeholder_class!(KeyboardEvent);
/// TODO(fabric2seer): java com.mojang.logging.LogUtils
placeholder_class!(LogUtils);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.MoulConfigEditor
placeholder_class!(MoulConfigEditor);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.platform.MoulConfigScreenComponent
placeholder_class!(MoulConfigScreenComponent);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.gui.MouseEvent
placeholder_class!(MouseEvent);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.common.MyResourceLocation
placeholder_class!(MyResourceLocation);
/// TODO(fabric2seer): java NarrationPriority
placeholder_class!(NarrationPriority);
/// TODO(fabric2seer): java com.ibm.icu.text.NumberFormat
placeholder_class!(NumberFormat);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.processor.ProcessedCategory
placeholder_class!(ProcessedCategory);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.processor.ProcessedOption
placeholder_class!(ProcessedOption);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.common.RenderContext
placeholder_class!(RenderContext);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.annotations.SearchTag
placeholder_class!(SearchTag);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.Social
placeholder_class!(Social);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.common.text.StructuredText
placeholder_class!(StructuredText);
/// TODO(fabric2seer): java io.github.notenoughupdates.moulconfig.TitleRenderingBehaviour
placeholder_class!(TitleRenderingBehaviour);
/// TODO(fabric2seer): java org.apache.commons.lang3.function.TriFunction
placeholder_class!(TriFunction);
/// TODO(fabric2seer): java java.lang.reflect.Type
placeholder_class!(Type);
