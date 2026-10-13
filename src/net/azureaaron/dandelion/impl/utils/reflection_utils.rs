//! Ported from net.azureaaron.dandelion.impl.utils.ReflectionUtils.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public final class ReflectionUtils
pub struct ReflectionUtils {}

/// The members of `Class<T>` a value's class has no generated form for (`getClass()` of an option's default value:
/// `Boolean`, `Integer`, `Color`, an enum, ..): the port answers its name, and the reflective operations Dandelion
/// never asks of such a class throw.
fn no_reflection<T>() -> T {
    panic!("UnsupportedOperationException: no reflective constructor for {}", std::any::type_name::<T>())
}

fn no_to_gson<T>(_value: &T) -> compat::GsonValue {
    panic!("UnsupportedOperationException: no Gson form for {}", std::any::type_name::<T>())
}

fn no_read_gson<T>(_json: &serde_json::Value) -> Option<T> {
    panic!("UnsupportedOperationException: no Gson form for {}", std::any::type_name::<T>())
}

fn no_patch<T>(_value: &mut T, _path: &[&str], _patch: &compat::PatchValue) -> bool {
    false
}

fn no_enum_constants() -> Vec<String> {
    Vec::new()
}

impl ReflectionUtils {
    /// java: public static <T> T createNewDefaultInstance(Class<T> clazz)
    pub fn create_new_default_instance<T: 'static>(clazz: &compat::ClassOf<T>) -> T {
        // java: try {..} catch (ReflectiveOperationException e) - an exception here is a panic, caught by `catch_unwind`
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| clazz.new_instance())) {
            Ok(instance) => instance,
            Err(_e) => panic!("RuntimeException: [Dandelion] Class {} must have a no args constructor!", compat::jstring(clazz)),
        }
    }

    /// java: @SuppressWarnings("unchecked") public static <T> Class<? extends T> getActualClass(T object)
    ///
    /// Returns the actual class representing an `object`: for an enum constant declared with a constant-specific
    /// class body Java answers the enclosing enum class. A Rust value's class IS its type (an enum's variants have
    /// no classes of their own), so the answer is `T`'s class; `isEnum()` is `false` unless the type is known to be
    /// an enum (`get_actual_enum_class`).
    pub fn get_actual_class<T: 'static>(object: &T) -> compat::ClassOf<T> {
        let _ = object;
        compat::ClassOf {
            name: std::any::type_name::<T>(),
            new: no_reflection::<T>,
            to_gson: no_to_gson::<T>,
            read_gson: no_read_gson::<T>,
            patch: no_patch::<T>,
            is_enum: false,
            enum_constants: no_enum_constants,
        }
    }

    /// `getActualClass(object)` for a value whose type is a Java enum: `isEnum()` and `getEnumConstants()` answer.
    pub fn get_actual_enum_class<T: compat::JavaEnum>(object: &T) -> compat::ClassOf<T> {
        fn enum_constants<T: compat::JavaEnum>() -> Vec<String> {
            T::enum_constants().iter().map(|c| c.name()).collect()
        }
        compat::ClassOf { is_enum: true, enum_constants: enum_constants::<T>, ..ReflectionUtils::get_actual_class(object) }
    }
}
