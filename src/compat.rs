//! The Java-semantics helpers: the ones of the library this mod is built on.
pub use yacl::compat::*;

/// Java's unchecked generic cast `(Foo<U>) fooOfT` where the program knows `T == U` (the `switch` arm proved it):
/// a mismatch is the `ClassCastException` Java would throw at first use.
pub fn java_cast<A: 'static, B: 'static>(value: A) -> B {
    let boxed: Box<dyn std::any::Any> = Box::new(value);
    match boxed.downcast::<B>() {
        Ok(b) => *b,
        Err(_) => panic!(
            "ClassCastException: {} cannot be cast to {}",
            std::any::type_name::<A>(),
            std::any::type_name::<B>()
        ),
    }
}
