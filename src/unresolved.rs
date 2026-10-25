//! Types the mod uses that neither the mod, the JDK mapping nor the port has (fabric2seer).
//! Each is a placeholder: give it the real type (a library crate, or port the class).
//!
//! MoulConfig (`io.github.notenoughupdates.moulconfig`, shaded by Dandelion) is the `moulconfig` library crate
//! (mods-src/moulconfig, GAPS_D.md D1 closed): its classes no longer have placeholders here. What is left are the JDK
//! and game types the converter could not place; nothing in the mod reads them any more.

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
/// TODO(fabric2seer): java java.lang.Class
placeholder_class!(Class);
/// TODO(fabric2seer): java java.lang.Enum
placeholder_class!(Enum);
/// TODO(fabric2seer): java java.lang.reflect.Field
placeholder_class!(Field);
/// TODO(fabric2seer): java com.mojang.blaze3d.platform.InputConstants
placeholder_class!(InputConstants);
/// TODO(fabric2seer): java com.google.gson.JsonDeserializationContext
placeholder_class!(JsonDeserializationContext);
/// TODO(fabric2seer): java com.google.gson.JsonSerializationContext
placeholder_class!(JsonSerializationContext);
/// TODO(fabric2seer): java com.mojang.logging.LogUtils
placeholder_class!(LogUtils);
/// TODO(fabric2seer): java NarrationPriority
placeholder_class!(NarrationPriority);
/// TODO(fabric2seer): java com.ibm.icu.text.NumberFormat
placeholder_class!(NumberFormat);
/// TODO(fabric2seer): java org.apache.commons.lang3.function.TriFunction
placeholder_class!(TriFunction);
/// TODO(fabric2seer): java java.lang.reflect.Type
placeholder_class!(Type);
