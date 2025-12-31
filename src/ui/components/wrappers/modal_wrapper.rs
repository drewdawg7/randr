use ratatui::layout::Rect;
use ratatui::Frame;
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

use crate::system::game_state;
use crate::ui::components::widgets::modal::Modal;

/// A wrapper that adds modal overlay capability to any component.
/// When modal is open (tracked via game_state().modal_open), events are blocked
/// and Shift+I toggles the modal.
pub struct ModalWrapper<C: MockComponent> {
    props: Props,
    content: C,
}

impl<C: MockComponent> ModalWrapper<C> {
    pub fn new(content: C) -> Self {
        Self {
            props: Props::default(),
            content,
        }
    }
}

impl<C: MockComponent> MockComponent for ModalWrapper<C> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Always render wrapped content
        self.content.view(frame, area);

        // Render keybind guide modal if open
        if game_state().modal_open {
            let lines = vec![
                "Keybinds".to_string(),
                "".to_string(),
                "Shift+I  Show/hide this guide".to_string(),
                "Shift+E  Equip/unequip item".to_string(),
                "Shift+L  Lock/unlock item".to_string(),
            ];
            let modal = Modal::new(lines);
            modal.render(frame);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr).or_else(|| self.content.query(attr))
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.content.attr(attr, value);
    }

    fn state(&self) -> State {
        self.content.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        if game_state().modal_open {
            CmdResult::None // Block commands when modal is open
        } else {
            self.content.perform(cmd)
        }
    }
}

impl<C: MockComponent + Component<Event<NoUserEvent>, NoUserEvent>> Component<Event<NoUserEvent>, NoUserEvent> for ModalWrapper<C> {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        // Check for Shift+I toggle (capital I means Shift was held)
        if let Event::Keyboard(KeyEvent { code: Key::Char('I'), .. }) = ev {
            game_state().modal_open = !game_state().modal_open;
            return None; // Consume event
        }

        // If modal is open, block all other events
        if game_state().modal_open {
            return None;
        }

        // Pass through to wrapped component
        self.content.on(ev)
    }
}
