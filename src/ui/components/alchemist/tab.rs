use ratatui::{layout::Rect, widgets::ListState, Frame};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use super::{brew, lab_stone_art, menu};

pub enum StateChange {
    ToMenu,
    ToBrew,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum AlchemistState {
    Menu,
    Brew,
}

pub struct AlchemistTab {
    props: Props,
    state: AlchemistState,
    list_state: ListState,
}

impl AlchemistTab {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            props: Props::default(),
            state: AlchemistState::Menu,
            list_state,
        }
    }

    fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
    }

    fn apply_state_change(&mut self, change: StateChange) {
        match change {
            StateChange::ToMenu => self.state = AlchemistState::Menu,
            StateChange::ToBrew => self.state = AlchemistState::Brew,
        }
        self.reset_selection();
    }
}

impl MockComponent for AlchemistTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        match self.state {
            AlchemistState::Menu => {
                // Render background first, then menu on top
                lab_stone_art::render_lab_stone(frame, area);
                menu::render(frame, area, &mut self.list_state);
            }
            AlchemistState::Brew => brew::render(frame, area, &mut self.list_state),
        }
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.list_state.selected().unwrap_or(0)))
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        let (result, state_change) = match self.state {
            AlchemistState::Menu => menu::handle(cmd, &mut self.list_state),
            AlchemistState::Brew => brew::handle(cmd, &mut self.list_state),
        };

        if let Some(change) = state_change {
            self.apply_state_change(change);
        }

        result
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for AlchemistTab {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(tuirealm::command::Direction::Up));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                self.perform(Cmd::Move(tuirealm::command::Direction::Down));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                self.perform(Cmd::Submit);
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                self.perform(Cmd::Cancel);
                None
            }
            // Pass unhandled events back to parent (for tab switching)
            _ => Some(ev),
        }
    }
}
