//! Ported from net.azureaaron.dandelion.impl.controllers.EnumControllerImpl.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::{self, Controller, ControllerKind};
use crate::net::azureaaron::dandelion::api::controllers::enum_controller::{self, AnyEnumController, EnumController};
use crate::net::azureaaron::dandelion::r#impl::yacl::yacl_controller_adapter::YACLControllerAdapter;
use ::yacl::dev::isxander::yacl3::api::controller::controller_builder::ControllerBuilder;
use ::yacl::dev::isxander::yacl3::api::option::Option as YOption;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::component::Component as _;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;

/// java: private static final Function<? extends Enum<?>, Component> TO_STRING_FORMATTER = v -> Component.nullToEmpty(v.toString());
///
/// An enum's `toString()` is its `name()` (`JavaEnum::name`, as YACL's `EnumController` uses it).
fn to_string_formatter<T: compat::JavaEnum>() -> Arc<dyn Fn(T) -> MutableComponent> {
    Arc::new(|v: T| component::null_to_empty(Some(&v.name())))
}

/// java: public class EnumControllerImpl<T extends Enum<T>> implements EnumController<T>
pub struct EnumControllerImpl<T: compat::JavaEnum> {
    /// java: private final boolean dropdown;
    pub dropdown: bool,
    /// java: private final Function<T, Component> formatter;
    pub formatter: Arc<dyn Fn(T) -> MutableComponent>,
}

impl<T: compat::JavaEnum> EnumControllerImpl<T> {
    /// java: protected EnumControllerImpl(boolean dropdown, Function<T, Component> formatter)
    pub fn new(dropdown: bool, formatter: Arc<dyn Fn(T) -> MutableComponent>) -> Self {
        Self { dropdown, formatter }
    }
}

impl<T: compat::JavaEnum> Controller<T> for EnumControllerImpl<T> {
    fn kind(&self) -> ControllerKind<'_> {
        ControllerKind::Enum(self)
    }
}

impl<T: compat::JavaEnum> EnumController<T> for EnumControllerImpl<T> {
    /// java: @Override public boolean dropdown()
    fn dropdown(&self) -> bool {
        self.dropdown
    }

    /// java: @Override public Function<T, Component> formatter()
    fn formatter(&self) -> Arc<dyn Fn(T) -> MutableComponent> {
        self.formatter.clone()
    }
}

impl<T: compat::JavaEnum> AnyEnumController for EnumControllerImpl<T> {
    fn dropdown(&self) -> bool {
        self.dropdown
    }

    fn to_yacl_controller_builder_erased(&self, yacl_option: Box<dyn std::any::Any>, r#type: Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // java: `(Option<Enum>) yaclOption` and `.enumClass(type)` - unchecked casts to this controller's `T`
        let yacl_option: Arc<dyn YOption<T>> = *yacl_option
            .downcast::<Arc<dyn YOption<T>>>()
            .unwrap_or_else(|_| panic!("ClassCastException: the YACL option is not an Option of this enum controller's type"));
        let r#type: compat::ClassOf<T> = *r#type
            .downcast::<compat::ClassOf<T>>()
            .unwrap_or_else(|_| panic!("ClassCastException: the option's class is not this enum controller's type"));
        let builder: Box<dyn ControllerBuilder<T>> = YACLControllerAdapter::to_yacl_enum_controller_builder(yacl_option, r#type, self);
        Box::new(builder)
    }

    fn display_values(&self) -> Vec<String> {
        // java: Arrays.stream(option.type().getEnumConstants()).map(enumController.formatter()).map(t -> ((Component) t).getString())
        T::enum_constants().into_iter().map(|constant| (self.formatter)(constant).get_string()).collect()
    }
}

/// java: public static class EnumControllerBuilderImpl<T extends Enum<T>> implements EnumController.Builder<T>
pub struct EnumControllerBuilderImpl<T: compat::JavaEnum> {
    /// java: private boolean dropdown = false;
    pub dropdown: bool,
    /// java: private Function<T, Component> formatter = (Function<T, Component>) TO_STRING_FORMATTER;
    pub formatter: Arc<dyn Fn(T) -> MutableComponent>,
}

impl<T: compat::JavaEnum> EnumControllerBuilderImpl<T> {
    /// java: the implicit no-argument constructor
    pub fn new() -> Self {
        Self { dropdown: false, formatter: to_string_formatter::<T>() }
    }
}

impl<T: compat::JavaEnum> enum_controller::Builder<T> for EnumControllerBuilderImpl<T> {
    /// java: @Override public EnumController.Builder<T> dropdown(boolean dropdown)
    fn dropdown(mut self, dropdown: bool) -> Self {
        self.dropdown = dropdown;
        self
    }

    /// java: @Override public EnumController.Builder<T> formatter(Function<T, Component> formatter)
    fn formatter(mut self, formatter: impl Fn(T) -> MutableComponent + 'static) -> Self {
        self.formatter = Arc::new(formatter);
        self
    }
}

impl<T: compat::JavaEnum> controller::Builder<T, dyn EnumController<T>> for EnumControllerBuilderImpl<T> {
    /// java: @Override public EnumController<T> build()
    fn build(self) -> Arc<dyn EnumController<T>> {
        Arc::new(EnumControllerImpl::new(self.dropdown, self.formatter))
    }
}
