//! Ported from net.azureaaron.dandelion.impl.utils.ReflectionUtils by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public final class ReflectionUtils
#[derive(Clone)]
pub struct ReflectionUtils {}

impl ReflectionUtils {
    /// java: private ReflectionUtils()
    pub fn new() -> Self {
        let mut this = Self {
        };
        this
    }

    /// java: public static <T> T createNewDefaultInstance(Class<T> clazz)
    pub fn create_new_default_instance<T: 'static>(clazz: &compat::ClassOf<T>) -> T {
        // java: try {..} catch (ReflectiveOperationException e) - an exception here is a panic, caught by `catch_unwind`; the body's `return` is the closure's value
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            return (clazz.new)();
        })) {
            Ok(__v) => return __v,
            Err(e) => {
                panic!("RuntimeException: {}", format!("[Dandelion] Class {} must have a no args constructor!", compat::jstring(&clazz)).to_string());
            }
        }
    }

    /// java: @SuppressWarnings("unchecked") public static <T> Class<? extends T> getActualClass(T object)
    pub fn get_actual_class<T: 'static>(object: &T) -> compat::ClassOf<T> {
        let mut original_class: compat::ClassOf<T> = compat::type_name_of(&object);
        // TODO(fabric2seer): `.getEnclosingClass(..)` on a Class
        //   java: Class<? extends T> enclosingClass = (Class<? extends T>) originalClass.getEnclosingClass();
        let mut enclosing_class: compat::ClassOf<T> = todo!("fabric2seer: the initialiser of `enclosingClass` (TODO above)");
        if !original_class.is_enum && true /* java: enclosing_class != null: never null here */ && enclosing_class.is_enum {
            return enclosing_class;
        } else {
            return original_class;
        }
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}
