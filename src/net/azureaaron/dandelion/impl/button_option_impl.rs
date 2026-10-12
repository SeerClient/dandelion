//! Ported from net.azureaaron.dandelion.impl.ButtonOptionImpl by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::net::azureaaron::dandelion::api::button_option::Builder;
use crate::net::azureaaron::dandelion::api::button_option::ButtonOption;
use crate::net::azureaaron::dandelion::api::controllers::controller::Controller;
use crate::net::azureaaron::dandelion::api::option_binding::OptionBinding;
use crate::net::azureaaron::dandelion::api::option_flag::OptionFlag;
use crate::net::azureaaron::dandelion::api::option_listener::OptionListener;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use mcrust::net::minecraft::resources::identifier::Identifier;

/// java: public class ButtonOptionImpl implements ButtonOption
#[derive(Clone)]
pub struct ButtonOptionImpl {
    /// java: private final @Nullable Identifier id;
    pub id: Option<Identifier>,
    /// java: private final Component name;
    pub name: MutableComponent,
    /// java: private final List<Component> description;
    pub description: Vec<MutableComponent>,
    /// java: private final Component prompt;
    pub prompt: MutableComponent,
    /// java: private final Consumer<Screen> action;
    pub action: Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>,
    /// java: private final List<Component> tags;
    pub tags: Vec<MutableComponent>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl ButtonOptionImpl {
    /// java: protected ButtonOptionImpl(@Nullable Identifier id, Component name, List<Component> description, List<Component> tags, Component prompt, Consumer<Screen> action)
    pub fn new(mut id: Option<Identifier>, mut name: MutableComponent, mut description: Vec<MutableComponent>, mut tags: Vec<MutableComponent>, mut prompt: MutableComponent, action: Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>) -> compat::Shared<Self> {
        let __init_id = id.clone();
        let __init_name = name.clone();
        let __init_description = description.clone();
        let __init_prompt = prompt.clone();
        let __init_action = action.clone();
        let __init_tags = tags.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            id: __init_id,
            name: __init_name,
            description: __init_description,
            prompt: __init_prompt,
            action: __init_action,
            tags: __init_tags,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl ButtonOption for ButtonOptionImpl {
    /// java: @Override public Component prompt() (dispatch: the body for this class)
    fn prompt(&self) -> fn(&compat::Shared<dyn ButtonOption>) -> MutableComponent {
        ButtonOptionImpl::prompt_dyn
    }

    /// java: @Override public Consumer<Screen> action() (dispatch: the body for this class)
    fn action(&self) -> fn(&compat::Shared<dyn ButtonOption>) -> Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)> {
        ButtonOptionImpl::action_dyn
    }

}

impl crate::net::azureaaron::dandelion::api::option::Option<compat::Object> for ButtonOptionImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public @Nullable Identifier id()
    fn id(&self) -> Option<Identifier> {
        return self.id.clone();
    }

    /// java: @Override public Component name()
    fn name(&self) -> MutableComponent {
        return self.name.clone();
    }

    /// java: @Override public List<Component> description()
    fn description(&self) -> Vec<MutableComponent> {
        return self.description.clone();
    }

    /// java: @Override public List<Component> tags()
    fn tags(&self) -> Vec<MutableComponent> {
        return self.tags.clone();
    }

    /// java: @Override public OptionBinding<Consumer<Screen>> binding()
    fn binding(&self) -> compat::Shared<dyn OptionBinding<Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @Override public Controller<Consumer<Screen>> controller()
    fn controller(&self) -> compat::Shared<dyn Controller<Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>>> {
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

    /// java: @Override public List<OptionListener<Consumer<Screen>>> listeners()
    fn listeners(&self) -> Vec<compat::Shared<dyn OptionListener<Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>>>> {
        panic!("UnsupportedOperationException");
    }

    /// java: @SuppressWarnings("unchecked") @Override public Class<Consumer<Screen>> type()
    fn r#type(&self) -> compat::ClassOf<Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>> {
        // TODO(fabric2seer): `getClass` on a Consumer
        //   java: return (Class<Consumer<Screen>>) this.action.getClass();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

}

impl ButtonOptionImpl {
    /// java: @Override public Component prompt()
    pub fn prompt_impl(__this: &compat::Shared<Self>) -> MutableComponent {
        return compat::read(__this, |__t| __t.prompt.clone());
    }

    /// ButtonOptionImpl.prompt as the dispatch of ButtonOption.prompt
    pub fn prompt_dyn(__this: &compat::Shared<dyn ButtonOption>) -> MutableComponent {
        Self::prompt_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

    /// java: @Override public Consumer<Screen> action()
    pub fn action_impl(__this: &compat::Shared<Self>) -> Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)> {
        return compat::read(__this, |__t| __t.action.clone());
    }

    /// ButtonOptionImpl.action as the dispatch of ButtonOption.action
    pub fn action_dyn(__this: &compat::Shared<dyn ButtonOption>) -> Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)> {
        Self::action_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}

/// java: public static class ButtonOptionBuilderImpl implements ButtonOption.Builder
#[derive(Clone)]
pub struct ButtonOptionBuilderImpl {
    /// java: private @Nullable Identifier id = null;
    pub id: Option<Identifier>,
    /// java: private Component name = Component.empty();
    pub name: MutableComponent,
    /// java: private List<Component> description = List.of();
    pub description: Vec<MutableComponent>,
    /// java: private List<Component> tags = List.of();
    pub tags: Vec<MutableComponent>,
    /// java: private Component prompt = Component.nullToEmpty("Execute");
    pub prompt: MutableComponent,
    /// java: private Consumer<Screen> action = Consumers.nop();
    pub action: Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>,
}

impl ButtonOptionBuilderImpl {
    /// java: the implicit no-argument constructor
    pub fn new() -> compat::Shared<Self> {
        // TODO(fabric2seer): the initialiser of `action`: static call Consumers.nop(..)
        //   java: private Consumer<Screen> action = Consumers.nop();
        let __this = compat::shared(Self {
            id: None,
            name: component::empty(),
            description: vec![],
            tags: vec![],
            prompt: component::null_to_empty(Some("Execute")),
            action: Default::default(),
        });
        __this
    }

}

impl Builder for ButtonOptionBuilderImpl {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder id(Identifier id) (dispatch: the body for this class)
    fn id(&self) -> fn(&compat::Shared<dyn Builder>, Identifier) -> compat::Shared<dyn Builder> {
        ButtonOptionBuilderImpl::id_dyn
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder name(Component name) (dispatch: the body for this class)
    fn name(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        ButtonOptionBuilderImpl::name_dyn
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder description(Component... texts) (dispatch: the body for this class)
    fn description(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        ButtonOptionBuilderImpl::description_dyn
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder tags(Component... tags) (dispatch: the body for this class)
    fn tags(&self) -> fn(&compat::Shared<dyn Builder>, &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        ButtonOptionBuilderImpl::tags_dyn
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder prompt(Component prompt) (dispatch: the body for this class)
    fn prompt(&self) -> fn(&compat::Shared<dyn Builder>, MutableComponent) -> compat::Shared<dyn Builder> {
        ButtonOptionBuilderImpl::prompt_dyn
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder action(Consumer<Screen> action) (dispatch: the body for this class)
    fn action(&self) -> fn(&compat::Shared<dyn Builder>, Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>) -> compat::Shared<dyn Builder> {
        ButtonOptionBuilderImpl::action_dyn
    }

    /// java: @Override public ButtonOption build() (dispatch: the body for this class)
    fn build(&self) -> fn(&compat::Shared<dyn Builder>) -> compat::Shared<dyn ButtonOption> {
        ButtonOptionBuilderImpl::build_dyn
    }

}

impl ButtonOptionBuilderImpl {
    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder id(Identifier id)
    pub fn id_impl(__this: &compat::Shared<Self>, mut id: Identifier) -> compat::Shared<dyn Builder> {
        { let __v = Some(id.clone()); compat::write(__this, |__t| __t.id = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ButtonOptionBuilderImpl.id as the dispatch of Builder.id
    pub fn id_dyn(__this: &compat::Shared<dyn Builder>, __p0: Identifier) -> compat::Shared<dyn Builder> {
        Self::id_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder name(Component name)
    pub fn name_impl(__this: &compat::Shared<Self>, mut name: MutableComponent) -> compat::Shared<dyn Builder> {
        { let __v = name.clone(); compat::write(__this, |__t| __t.name = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ButtonOptionBuilderImpl.name as the dispatch of Builder.name
    pub fn name_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::name_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder description(Component... texts)
    pub fn description_impl(__this: &compat::Shared<Self>, texts: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        { let __v = texts.to_vec(); compat::write(__this, |__t| __t.description = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ButtonOptionBuilderImpl.description as the dispatch of Builder.description
    pub fn description_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::description_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder tags(Component... tags)
    pub fn tags_impl(__this: &compat::Shared<Self>, tags: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        { let __v = tags.to_vec(); compat::write(__this, |__t| __t.tags = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ButtonOptionBuilderImpl.tags as the dispatch of Builder.tags
    pub fn tags_dyn(__this: &compat::Shared<dyn Builder>, __p0: &Vec<MutableComponent>) -> compat::Shared<dyn Builder> {
        Self::tags_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder prompt(Component prompt)
    pub fn prompt_impl(__this: &compat::Shared<Self>, mut prompt: MutableComponent) -> compat::Shared<dyn Builder> {
        { let __v = prompt.clone(); compat::write(__this, |__t| __t.prompt = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ButtonOptionBuilderImpl.prompt as the dispatch of Builder.prompt
    pub fn prompt_dyn(__this: &compat::Shared<dyn Builder>, __p0: MutableComponent) -> compat::Shared<dyn Builder> {
        Self::prompt_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public net.azureaaron.dandelion.api.ButtonOption.Builder action(Consumer<Screen> action)
    pub fn action_impl(__this: &compat::Shared<Self>, action: Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>) -> compat::Shared<dyn Builder> {
        { let __v = action.clone(); compat::write(__this, |__t| __t.action = __v); }
        return (__this.clone() as compat::Shared<dyn Builder>);
    }

    /// ButtonOptionBuilderImpl.action as the dispatch of Builder.action
    pub fn action_dyn(__this: &compat::Shared<dyn Builder>, __p0: Arc<dyn Fn(mcrust::net::minecraft::client::gui::screens::screen::Screen)>) -> compat::Shared<dyn Builder> {
        Self::action_impl(&compat::view_as::<Self, _, Self>(__this, |a| a), __p0)
    }

    /// java: @Override public ButtonOption build()
    pub fn build_impl(__this: &compat::Shared<Self>) -> compat::Shared<dyn ButtonOption> {
        return (ButtonOptionImpl::new(compat::read(__this, |__t| __t.id.clone()), compat::read(__this, |__t| __t.name.clone()), compat::read(__this, |__t| __t.description.clone()), compat::read(__this, |__t| __t.tags.clone()), compat::read(__this, |__t| __t.prompt.clone()), compat::read(__this, |__t| __t.action.clone())) as compat::Shared<dyn ButtonOption>);
    }

    /// ButtonOptionBuilderImpl.build as the dispatch of Builder.build
    pub fn build_dyn(__this: &compat::Shared<dyn Builder>) -> compat::Shared<dyn ButtonOption> {
        Self::build_impl(&compat::view_as::<Self, _, Self>(__this, |a| a))
    }

}
