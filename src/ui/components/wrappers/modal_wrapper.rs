use ratatui::layout::Rect;
use ratatui::Frame;
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

use crate::system::{game_state, ModalType};
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

        // Render active modal
        match game_state().active_modal {
            ModalType::None => {}
            ModalType::Keybinds => {
                let lines = vec![
                    "Keybinds".to_string(),
                    "".to_string(),
                    "i        Open inventory".to_string(),
                    "p        View character profile".to_string(),
                    "d        Toggle item details".to_string(),
                    "Shift+I  Show/hide this guide".to_string(),
                    "e        Equip/unequip item".to_string(),
                    "l        Lock/unlock item".to_string(),
                ];
                let modal = Modal::new(lines);
                modal.render(frame);
            }
            ModalType::Inventory => {
                game_state().inventory_modal.render(frame);
            }
            ModalType::Profile => {
                game_state().profile_modal.render(frame);
            }
            ModalType::SpellTest => {
                game_state().spell_test_modal.render(frame);
            }
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
        if game_state().active_modal != ModalType::None {
            CmdResult::None // Block commands when any modal is open
        } else {
            self.content.perform(cmd)
        }
    }
}

impl<C: MockComponent + Component<Event<NoUserEvent>, NoUserEvent>> Component<Event<NoUserEvent>, NoUserEvent> for ModalWrapper<C> {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        // IMPORTANT: SpellTest modal captures ALL keyboard input (for text entry)
        // Must be checked FIRST before any other keybinds
        if game_state().active_modal == ModalType::SpellTest {
            if let Event::Keyboard(KeyEvent { code, .. }) = ev {
                let should_close = game_state().spell_test_modal.handle_input(code);
                if should_close {
                    game_state().active_modal = ModalType::None;
                }
            }
            return None;
        }

        // Global Esc handler: close any open modal
        if let Event::Keyboard(KeyEvent { code: Key::Esc, .. }) = ev {
            if game_state().active_modal != ModalType::None {
                game_state().active_modal = ModalType::None;
                return None;
            }
            // If no modal is open, Esc does nothing (not used for navigation)
            return None;
        }

        // Handle Shift+I for keybinds modal toggle
        if let Event::Keyboard(KeyEvent { code: Key::Char('I'), .. }) = ev {
            let gs = game_state();
            gs.active_modal = if gs.active_modal == ModalType::Keybinds {
                ModalType::None
            } else {
                ModalType::Keybinds
            };
            return None;
        }

        // Handle lowercase 'i' for inventory modal
        if let Event::Keyboard(KeyEvent { code: Key::Char('i'), .. }) = ev {
            let gs = game_state();
            if gs.active_modal == ModalType::Inventory {
                gs.active_modal = ModalType::None;
            } else {
                gs.active_modal = ModalType::Inventory;
                gs.inventory_modal.reset();
            }
            return None;
        }

        // Handle 'p' for profile modal
        if let Event::Keyboard(KeyEvent { code: Key::Char('p'), .. }) = ev {
            let gs = game_state();
            if gs.active_modal == ModalType::Profile {
                gs.active_modal = ModalType::None;
            } else {
                gs.active_modal = ModalType::Profile;
                gs.profile_modal.reset();
            }
            return None;
        }

        // Handle 't' for spell test modal (god mode)
        if let Event::Keyboard(KeyEvent { code: Key::Char('t'), .. }) = ev {
            let gs = game_state();
            if gs.active_modal == ModalType::SpellTest {
                gs.active_modal = ModalType::None;
            } else {
                gs.active_modal = ModalType::SpellTest;
                gs.spell_test_modal.reset();
            }
            return None;
        }

        // If profile modal is open, handle its input
        if game_state().active_modal == ModalType::Profile {
            if let Event::Keyboard(KeyEvent { code, .. }) = ev {
                let should_close = game_state().profile_modal.handle_input(code);
                if should_close {
                    game_state().active_modal = ModalType::None;
                }
            }
            return None;
        }

        // If inventory modal is open, forward events to it
        if game_state().active_modal == ModalType::Inventory {
            if let Event::Keyboard(KeyEvent { code, .. }) = ev {
                let should_close = game_state().inventory_modal.handle_input(code);
                if should_close {
                    game_state().active_modal = ModalType::None;
                }
            }
            return None;
        }

        // If keybinds modal is open, block all events (except Esc, handled above)
        if game_state().active_modal == ModalType::Keybinds {
            return None;
        }

        // Pass through to wrapped component
        self.content.on(ev)
    }
}
