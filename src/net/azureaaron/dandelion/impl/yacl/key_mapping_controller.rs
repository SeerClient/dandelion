//! Ported from net.azureaaron.dandelion.impl.yacl.KeyMappingController by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;
use ::yacl::dev::isxander::yacl3::api::controller::Controller;
use ::yacl::dev::isxander::yacl3::api::utils::dimension::Dimension;
use ::yacl::dev::isxander::yacl3::gui::abstract_widget::AbstractWidget;
use ::yacl::dev::isxander::yacl3::gui::abstract_widget::AbstractWidgetBase;
use ::yacl::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidget;
use ::yacl::dev::isxander::yacl3::gui::controllers::controller_widget::ControllerWidgetBase;
use ::yacl::dev::isxander::yacl3::gui::yacl_screen::YACLScreen;
use mcrust::com::mojang::blaze3d::platform::input_constants::Key;
use mcrust::net::minecraft::chat_formatting::ChatFormatting;
use mcrust::net::minecraft::client::gui::narration::narration_element_output::NarrationElementOutput;
use mcrust::net::minecraft::client::input::mouse_button_event::MouseButtonEvent;
use mcrust::net::minecraft::network::chat::component;
use mcrust::net::minecraft::network::chat::mutable_component::MutableComponent;
use ::yacl::dev::isxander::yacl3::api::option::Option as _;
use ::yacl::dev::isxander::yacl3::api::state_manager::StateManager as _;

/// java: public class KeyMappingController implements Controller<InputConstants.Key>
#[derive(Clone)]
pub struct KeyMappingController {
    /// java: private final Option<InputConstants.Key> option;
    pub option: compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option::Option<Key>>,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl KeyMappingController {
    /// java: protected KeyMappingController(Option<InputConstants.Key> option)
    pub fn new(mut option: compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option::Option<Key>>) -> compat::Shared<Self> {
        let __init_option = option.clone();
        let __this = compat::shared(Self {
            self_ref: std::sync::Weak::new(),
            option: __init_option,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

}

impl Controller<mcrust::com::mojang::blaze3d::platform::input_constants::Key> for KeyMappingController {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public Option<InputConstants.Key> option()
    fn option(&self) -> compat::Shared<dyn ::yacl::dev::isxander::yacl3::api::option::Option<Key>> {
        return self.option.clone();
    }

    /// java: @Override public Component formatValue()
    fn format_value(&self) -> MutableComponent {
        // TODO(fabric2seer): `.getDisplayName(..)` on a T
        //   java: return this.option().stateManager().get().getDisplayName();
        todo!("fabric2seer: the value of this function is in a TODO above")
    }

    /// java: @Override public AbstractWidget provideWidget(YACLScreen screen, Dimension<Integer> widgetDimension)
    fn provide_widget(&self, screen: &compat::Shared<YACLScreen>, widget_dimension: &compat::Shared<dyn Dimension<i32>>) -> compat::Shared<dyn AbstractWidget> {
        return (KeyCodeControllerElement::new(self.self_ref.upgrade().unwrap(), screen.clone(), widget_dimension.clone()) as compat::Shared<dyn AbstractWidget>);
    }

}

/// java: protected static class KeyCodeControllerElement extends ControllerWidget<KeyMappingController>
pub struct KeyCodeControllerElement {
    /// java: extends ControllerWidget
    pub base: ControllerWidgetBase<compat::Shared<KeyMappingController>>,
    /// java: private boolean awaitingKeyPress = false;
    pub awaiting_key_press: bool,
    /// the object's own shared handle: Java's `this` handed to other objects
    pub self_ref: compat::WeakShared<Self>,
}

impl std::ops::Deref for KeyCodeControllerElement {
    type Target = ControllerWidgetBase<compat::Shared<KeyMappingController>>;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for KeyCodeControllerElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl KeyCodeControllerElement {
    /// java: protected KeyCodeControllerElement(KeyMappingController control, YACLScreen screen, Dimension<Integer> dim)
    pub fn new(mut control: compat::Shared<KeyMappingController>, mut screen: compat::Shared<YACLScreen>, mut dim: compat::Shared<dyn Dimension<i32>>) -> compat::Shared<Self> {
        let __this = compat::shared(Self {
            base: ControllerWidgetBase::<compat::Shared<KeyMappingController>>::new(control.clone(), screen.clone(), dim.clone()),
            self_ref: std::sync::Weak::new(),
            awaiting_key_press: false,
        });
        compat::write(&__this, |__t| __t.self_ref = std::sync::Arc::downgrade(&__this));
        __this
    }

    /// java: @Override public boolean mouseClicked(MouseButtonEvent event, boolean doubleClick)
    pub fn mouse_clicked(__this: &compat::Shared<Self>, event: &MouseButtonEvent, mut double_click: bool) -> bool {
        if !({ let __f = compat::dispatch(__this, |__t| __t.is_mouse_over()); __f(&compat::view(__this, |a| a as compat::Shared<dyn AbstractWidget>), event.x(), event.y()) }) || !(__this.lock().borrow().is_available()) {
            return false;
        }
        if compat::read(__this, |__t| __t.awaiting_key_press) {
            // TODO(fabric2seer): `.option(..)` on a T
            //   java: this.control.option().requestSet(InputConstants.Type.MOUSE.getOrCreate(event.button()));
        }
        { let __v = !(compat::read(__this, |__t| __t.awaiting_key_press)); compat::write(__this, |__t| __t.awaiting_key_press = __v); }
        return true;
    }

    /// java: @Override public boolean keyPressed(KeyEvent event)
    pub fn key_pressed(__this: &compat::Shared<Self>, event: &mcrust::net::minecraft::client::input::key_event::KeyEvent) -> bool {
        if compat::read(__this, |__t| __t.awaiting_key_press) {
            // TODO(fabric2seer): `.option(..)` on a T
            //   java: this.control.option().requestSet(InputConstants.Type.KEYSYM.getOrCreate(event.key()));
            { let __v = false; compat::write(__this, |__t| __t.awaiting_key_press = __v); }
            return true;
        }
        return false;
    }

}

impl ControllerWidget<compat::Shared<crate::net::azureaaron::dandelion::r#impl::yacl::key_mapping_controller::KeyMappingController>> for KeyCodeControllerElement {
    fn controller_widget_base(&self) -> &ControllerWidgetBase<compat::Shared<crate::net::azureaaron::dandelion::r#impl::yacl::key_mapping_controller::KeyMappingController>> {
        &self.base
    }
    fn controller_widget_base_mut(&mut self) -> &mut ControllerWidgetBase<compat::Shared<crate::net::azureaaron::dandelion::r#impl::yacl::key_mapping_controller::KeyMappingController>> {
        &mut self.base
    }

    /// java: @Override protected Component getValueText()
    fn get_value_text(&self) -> MutableComponent {
        if self.awaiting_key_press {
            return component::translatable("text.dandelion.option.keyMapping.awaitingPress").with_style_format(ChatFormatting::Italic);
        }
        return self.base.get_value_text();
    }

    /// java: @Override protected int getHoveredControlWidth()
    fn get_hovered_control_width(&self) -> i32 {
        return self.get_unhovered_control_width();
    }

}

impl AbstractWidget for KeyCodeControllerElement {
    fn abstract_widget_base(&self) -> &AbstractWidgetBase {
        &self.base.base
    }
    fn abstract_widget_base_mut(&mut self) -> &mut AbstractWidgetBase {
        &mut self.base.base
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    /// java: @Override public boolean canReset() (dispatch: the body for this class)
    fn can_reset(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> bool {
        <dyn ControllerWidget>::can_reset_from::<Self>
    }

    /// java: @Override public NarrationPriority narrationPriority() (dispatch: the body for this class)
    fn narration_priority(&self) -> fn(&compat::Shared<dyn AbstractWidget>) -> crate::unresolved::NarrationPriority {
        <dyn ControllerWidget>::narration_priority_from::<Self>
    }

    /// java: @Override public void unfocus() (dispatch: the body for this class)
    fn unfocus(&self) -> fn(&compat::Shared<dyn AbstractWidget>) {
        <dyn ControllerWidget>::unfocus_from::<Self>
    }

    /// java: @Override public boolean matchesSearch(String query) (dispatch: the body for this class)
    fn matches_search(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &str) -> bool {
        <dyn ControllerWidget>::matches_search_from::<Self>
    }

    /// java: @Override public void updateNarration(NarrationElementOutput builder) (dispatch: the body for this class)
    fn update_narration(&self) -> fn(&compat::Shared<dyn AbstractWidget>, &mut dyn NarrationElementOutput) {
        <dyn ControllerWidget>::update_narration_from::<Self>
    }

}
