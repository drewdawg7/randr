use ratatui::{layout::Rect, widgets::ListState, Frame};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use super::{brew, lab_stone_art, menu};
use crate::ui::components::widgets::tab_state::{StatefulTab, TabState};

pub enum StateChange {
    ToMenu,
    ToBrew,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) enum AlchemistState {
    #[default]
    Menu,
    Brew,
}

impl TabState for AlchemistState {
    type Change = StateChange;

    fn apply_change(_current: Self, change: Self::Change) -> Self {
        match change {
            StateChange::ToMenu => AlchemistState::Menu,
            StateChange::ToBrew => AlchemistState::Brew,
        }
    }
}

impl StatefulTab for AlchemistTab {
    type State = AlchemistState;

    fn current_state(&self) -> AlchemistState {
        self.state
    }

    fn set_state(&mut self, state: AlchemistState) {
        self.state = state;
    }

    fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
    }
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
            StatefulTab::apply_state_change(self, change);
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
            Event::Keyboard(KeyEvent { code: Key::Backspace, .. }) => {
                self.perform(Cmd::Cancel);
                None
            }
            // Pass unhandled events back to parent (for tab switching)
            _ => Some(ev),
        }
    }
}
