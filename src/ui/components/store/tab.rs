use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::ui::theme::{self as colors, ColorExt};
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::{combat::HasGold, system::game_state, ui::Id};
use super::store::StoreDisplay;
use crate::ui::components::utilities::store_header;

pub struct StoreTab {
    props: Props,
    store_display: StoreDisplay,
    list_state: ListState,
}

impl StoreTab {
    pub fn new() -> Self {
        let store = game_state().store();
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            props: Props::default(),
            store_display: StoreDisplay::new(store),
            list_state,
        }
    }
}

impl MockComponent for StoreTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Calculate content height: header (1) + rows + borders (2)
        let store = game_state().store();
        let content_height = (store.inventory.len() + 3) as u16;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(content_height),
                Constraint::Length(2),
            ])
            .split(area);

        // Render header with store name and gold
        let player_gold = game_state().player.gold();
        let header_line = store_header(store, player_gold);
        frame.render_widget(Paragraph::new(header_line), chunks[0]);

        // Render the store table
        self.store_display.view(frame, chunks[1]);

        // Render back button
        let selected = self.list_state.selected().unwrap_or(0) == 0;
        let back_style = if selected {
            Style::default().color(colors::YELLOW)
        } else {
            Style::default()
        };
        let back_prefix = if selected { "> " } else { "  " };
        let back_items = vec![ListItem::new(format!("{}{} Back", back_prefix, crate::ui::utilities::RETURN_ARROW)).style(back_style)];
        let back_list = List::new(back_items);
        frame.render_stateful_widget(back_list, chunks[2], &mut self.list_state);
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
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) |
            Cmd::Move(tuirealm::command::Direction::Down) => {
                // Only one item (back button), so no movement needed
                CmdResult::None
            }
            Cmd::Submit => {
                // Back button selected
                game_state().current_screen = Id::Menu;
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::None
        }
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for StoreTab {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        None
    }
}
