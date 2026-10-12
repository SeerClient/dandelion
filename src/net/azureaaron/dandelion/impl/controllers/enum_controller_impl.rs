//! Ported from net.azureaaron.dandelion.impl.controllers.EnumControllerImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::controllers::enum_controller::Builder;
use crate::net::azureaaron::dandelion::api::controllers::enum_controller::EnumController;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final Function<? extends Enum<?>, Component> TO_STRING_FORMATTER = v -> Component.nullToEmpty(v.toString());
pub static TO_STRING_FORMATTER: compat::MainThread<Arc<dyn Fn(crate::unresolved::Enum) -> MutableComponent>> = compat::MainThread::new(|| Arc::new(move |v| component::null_to_empty(Some(&compat::jstring(&v)))));

/// java: public class EnumControllerImpl<T extends Enum<T>> implements EnumController<T>
#[derive(Clone)]
pub struct EnumControllerImpl<T> {
    /// java: private final boolean dropdown;
    pub dropdown: bool,
    /// java: private final Function<T, Component> formatter;
    pub formatter: Arc<dyn Fn(T) -> MutableComponent>,
}

impl<T: Clone + 'static> EnumControllerImpl<T> {
    /// java: protected EnumControllerImpl(boolean dropdown, Function<T, Component> formatter)
    pub fn new(mut dropdown: bool, formatter: Arc<dyn Fn(T) -> MutableComponent>) -> Self {
        let __init_dropdown = dropdown;
        let __init_formatter = formatter.clone();
        let mut this = Self {
            dropdown: __init_dropdown,
            formatter: __init_formatter,
        };
        this
    }

}

impl<T: Clone + 'static> EnumController<T> for EnumControllerImpl<T> {
    /// java: @Override public boolean dropdown()
    fn dropdown(&self) -> bool {
        return self.dropdown;
    }

    /// java: @Override public Function<T, Component> formatter()
    fn formatter(&self) -> Arc<dyn Fn(T) -> MutableComponent> {
        return self.formatter.clone();
    }

}

impl<T: Clone + 'static> Controller<compat::Object> for EnumControllerImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

/// java: public static class EnumControllerBuilderImpl<T extends Enum<T>> implements EnumController.Builder<T>
#[derive(Clone)]
pub struct EnumControllerBuilderImpl<T> {
    /// java: private boolean dropdown = false;
    pub dropdown: bool,
    /// java: @SuppressWarnings("unchecked") private Function<T, Component> formatter = (Function<T, Component>) TO_STRING_FORMATTER;
    pub formatter: Arc<dyn Fn(T) -> MutableComponent>,
}

impl<T: Clone + 'static> EnumControllerBuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        let mut this = Self {
            dropdown: false,
            formatter: TO_STRING_FORMATTER.get_mut().clone(),
        };
        this
    }

}

impl<T: Clone + 'static> Builder<T> for EnumControllerBuilderImpl<T> {
    /// java: @Override public EnumController.Builder<T> dropdown(boolean dropdown)
    fn dropdown(&mut self, mut dropdown: bool) -> compat::Shared<dyn Builder<T>> {
        self.dropdown = dropdown;
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public EnumController.Builder<T> formatter(Function<T, Component> formatter)
    fn formatter(&mut self, formatter: Arc<dyn Fn(T) -> MutableComponent>) -> compat::Shared<dyn Builder<T>> {
        self.formatter = formatter.clone();
        // TODO(fabric2seer): `this` handed on as a reference other objects keep: the object has no shared handle to itself (port by hand)
        //   java: return this;
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl<T: Clone + 'static> crate::net::azureaaron::dandelion::api::controllers::controller::Builder<compat::Object, compat::Object> for EnumControllerBuilderImpl<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public EnumController<T> build()
    fn build(&self) -> compat::Shared<dyn EnumController<T>> {
        return (compat::shared(EnumControllerImpl::new(self.dropdown, self.formatter.clone())) as compat::Shared<dyn EnumController<compat::Object>>);
    }

}
