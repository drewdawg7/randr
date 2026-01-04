use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
    Frame,
};
use tuirealm::command::{Cmd, CmdResult};

use crate::{
    combat::HasGold,
    system::game_state,
    ui::Id,
};
use crate::ui::components::utilities::{
    list_move_down, list_move_up, render_location_header,
    selection_prefix, store_header, RETURN_ARROW,
};
use crate::ui::theme as colors;

pub enum StateChange {
    ToBuy,
    ToSell,
    ToStorage,
    ToMenu,
}

pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let store = game_state().store();
    let player_gold = game_state().player.gold();

    // Render header with store name and gold, get remaining area
    let header_lines = store_header(store, player_gold);
    let content_area =
        render_location_header(frame, area, header_lines, colors::STORE_BG, colors::WOOD_BROWN);

    // Center the menu vertically and horizontally
    const MENU_HEIGHT: u16 = 5;
    const MENU_WIDTH: u16 = 16;

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(2),
            Constraint::Length(MENU_HEIGHT),
            Constraint::Fill(3),
        ])
        .split(content_area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(MENU_WIDTH),
            Constraint::Fill(1),
        ])
        .split(vertical_chunks[1]);

    let centered_area = horizontal_chunks[1];

    // Menu options with explicit foreground colors
    let selected = list_state.selected().unwrap_or(0);
    let text_style = Style::default().fg(colors::WHITE);
    let menu_items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 0),
            Span::styled("Buy", text_style),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 1),
            Span::styled("Sell", text_style),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 2),
            Span::styled("Storage", text_style),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 3),
            Span::styled(format!("{} Back", RETURN_ARROW), text_style),
        ])),
    ];

    let menu = List::new(menu_items);
    frame.render_stateful_widget(menu, centered_area, list_state);
}

pub fn handle(cmd: Cmd, list_state: &mut ListState) -> (CmdResult, Option<StateChange>) {
    const MENU_SIZE: usize = 4; // Buy, Sell, Storage, Back

    match cmd {
        Cmd::Move(tuirealm::command::Direction::Up) => {
            list_move_up(list_state, MENU_SIZE);
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Move(tuirealm::command::Direction::Down) => {
            list_move_down(list_state, MENU_SIZE);
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Submit => {
            let selected = list_state.selected().unwrap_or(0);
            let state_change = match selected {
                0 => Some(StateChange::ToBuy),
                1 => Some(StateChange::ToSell),
                2 => Some(StateChange::ToStorage),
                3 => {
                    game_state().current_screen = Id::Menu;
                    None
                }
                _ => None,
            };
            (CmdResult::Submit(tuirealm::State::None), state_change)
        }
        _ => (CmdResult::None, None),
    }
}
