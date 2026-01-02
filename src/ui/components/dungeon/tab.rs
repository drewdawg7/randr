use ratatui::{layout::Rect, widgets::ListState, Frame};

use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::{system::game_state, ui::Id};
use crate::ui::components::backgrounds::render_stone_wall;

use super::{menu, StateChange};

pub struct DungeonTab {
    props: Props,
    list_state: ListState,
}

impl DungeonTab {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            props: Props::default(),
            list_state,
        }
    }
}

impl Default for DungeonTab {
    fn default() -> Self {
        Self::new()
    }
}

impl MockComponent for DungeonTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Render stone wall background first
        render_stone_wall(frame, area);
        // Then render the dungeon entrance menu on top
        menu::render(frame, area, &mut self.list_state);
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
        let (result, state_change) = menu::handle(cmd.clone(), &mut self.list_state);

        if let Some(change) = state_change {
            match change {
                StateChange::EnterDungeon => {
                    game_state().enter_dungeon();
                }
            }
        } else if matches!(cmd, Cmd::Submit) {
            // Handle "Back" selection (index 1)
            let selected = self.list_state.selected().unwrap_or(0);
            if selected == 1 {
                game_state().current_screen = Id::Menu;
            }
        }

        result
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for DungeonTab {
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
            _ => None,
        }
    }
}
