use ratatui::{
    layout::Rect,
    widgets::ListState,
    Frame,
};

use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::{
    system::game_state,
    ui::Id,
};

use super::{grass_art, menu, StateChange};

pub struct FieldTab {
    props: Props,
    list_state: ListState,
}

impl FieldTab {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            props: Props::default(),
            list_state,
        }
    }
}

impl Default for FieldTab {
    fn default() -> Self {
        Self::new()
    }
}

impl MockComponent for FieldTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Render background first, then menu on top
        grass_art::render_grass_field(frame, area);
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
        let (result, state_change) = menu::handle(cmd, &mut self.list_state);

        if let Some(change) = state_change {
            match change {
                StateChange::ToFight => {
                    let gs = game_state();
                    let field = &gs.town.field;
                    if let Ok(mob) = field.spawn_mob() {
                        gs.start_combat(mob);
                        gs.current_screen = Id::Fight;
                    }
                }
                StateChange::ToMine => {
                    game_state().current_screen = Id::Mine;
                }
            }
        }

        result
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for FieldTab {
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
