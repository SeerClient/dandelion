//! Ported from net.azureaaron.dandelion.impl.LabelOptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::label_option::Builder;
use crate::net::azureaaron::dandelion::api::label_option::LabelOption;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::contents::plain_text_contents::PlainTextContents;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;
use mcrust::net::minecraft::network::chat::component::Component as _;

/// java: public class LabelOptionImpl implements LabelOption
#[derive(Clone)]
pub struct LabelOptionImpl {
    /// java: private final Component label;
    pub label: MutableComponent,
}

impl LabelOptionImpl {
    /// java: protected LabelOptionImpl(Component label)
    pub fn new(mut label: MutableComponent) -> compat::Shared<Self> {
        let __init_label = label.clone();
        let __this = compat::shared(Self {
            label: __init_label,
        });
        __this
    }

}

impl LabelOption for LabelOptionImpl {
    /// java: @Override public Component label() (dispatch: the body for this class)
    fn label(&self) -> fn(&compat::Shared<dyn LabelOption>) -> MutableComponent {
        LabelOptionImpl::label_dyn
    }

}

impl crate::net::azureaaron::dandelion::api::option::Option<compat::Object> for LabelOptionImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable Identifier id()
    fn id(&self) -> Option<Identifier> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public Component name()
    fn name(&self) -> MutableComponent {
        return component::empty();
    }

    /// java: @Override public List<Component> description()
    fn description(&self) -> Vec<MutableComponent> {
        return vec![];
    }

    /// java: @Override public List<Component> tags()
    fn tags(&self) -> Vec<MutableComponent> {
        return vec![];
    }

    /// java: @Override public OptionBinding<Component> binding()
    fn binding(&self) -> compat::Shared<dyn OptionBinding<MutableComponent>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public Controller<Component> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<MutableComponent>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public boolean modifiable()
    fn modifiable(&self) -> bool {
        return true;
    }

    /// java: @Override public List<OptionFlag> flags()
    fn flags(&self) -> Vec<compat::Shared<dyn OptionFlag>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public List<OptionListener<Component>> listeners()
    fn listeners(&self) -> Vec<compat::Shared<dyn OptionListener<MutableComponent>>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public Class<Component> type()
    fn r#type(&self) -> compat::ClassOf<MutableComponent> {
        return std::any::TypeId::of::<MutableComponent>();
    }

}

impl LabelOptionImpl {
    /// java: @Override public Component label()
    pub fn label_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.label.clone());
    }

    /// LabelOptionImpl.label as the dispatch of LabelOption.label
    pub fn label_dyn(__this: &compat::Shared<dyn LabelOption>) -> MutableComponent {
        Self::label_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class LabelOptionBuilderImpl implements LabelOption.Builder
#[derive(Clone)]
pub struct LabelOptionBuilderImpl {
    /// java: private List<Component> label = List.of();
    pub label: Vec<MutableComponent>,
}

impl LabelOptionBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            label: vec![],
        });
        __this
    }

}

impl Builder for LabelOptionBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public net.azureaaron.dandelion.api.LabelOption.Builder label(Component... texts) (dispatch: the body for this class)
    fn label(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        LabelOptionBuilderImpl::label_dyn
    }

    /// java: @Override public LabelOption build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn LabelOption> {
        LabelOptionBuilderImpl::build_dyn
    }

}

impl LabelOptionBuilderImpl {
    /// java: @Override public net.azureaaron.dandelion.api.LabelOption.Builder label(Component... texts)
    pub fn label_impl(__this: &compat::Shared<Self>, texts: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        { let __v = texts.to_vec(); compat::write(__this, |__t| __t.label = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// LabelOptionBuilderImpl.label as the dispatch of Builder.label
    pub fn label_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::label_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public LabelOption build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn LabelOption> {
        let mut concatted: MutableComponent = MutableComponent::create(PlainTextContents::Empty);
        for text in compat::read(__this, |__t| __t.label.clone()) {
            concatted.get_siblings().push(text.clone());
        }
        return (LabelOptionImpl::new(concatted.clone()) as compat::Shared<dyn LabelOption>);
    }

    /// LabelOptionBuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn LabelOption> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
