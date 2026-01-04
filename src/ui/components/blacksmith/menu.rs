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
    inventory::HasInventory,
    item::ItemId,
    system::game_state,
    ui::Id,
};
use crate::ui::components::utilities::{
    blacksmith_header, list_move_down, list_move_up, render_location_header,
    selection_prefix, CROSSED_SWORDS, DOUBLE_ARROW_UP, FIRE,
};
use crate::ui::utilities::HAMMER;
use crate::ui::theme as colors;

use super::StateChange;

pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let player_gold = game_state().player.gold();
    let blacksmith = game_state().blacksmith();
    let stones = game_state().player.find_item_by_id(ItemId::QualityUpgradeStone)
        .map(|inv| inv.quantity).unwrap_or(0);

    let header_lines = blacksmith_header(blacksmith, player_gold, stones);
    let content_area = render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

    // Center the menu vertically and horizontally
    const MENU_HEIGHT: u16 = 4;
    const MENU_WIDTH: u16 = 28;

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

    let selected = list_state.selected().unwrap_or(0);
    let text_style = Style::default().fg(colors::WHITE);
    let menu_items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 0),
            Span::styled(format!("{} Upgrade Items", HAMMER), text_style),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 1),
            Span::styled(format!("{} Upgrade Item Quality", DOUBLE_ARROW_UP), text_style),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 2),
            Span::styled(format!("{}", FIRE), Style::default().fg(colors::FLAME_ORANGE)),
            Span::styled(" Smelt Ores", text_style),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 3),
            Span::styled(format!("{}", CROSSED_SWORDS), Style::default().fg(colors::LIGHT_STONE)),
            Span::styled(" Forge Items", text_style),
        ])),
    ];

    let menu = List::new(menu_items);
    frame.render_stateful_widget(menu, centered_area, list_state);
}

pub fn handle(cmd: Cmd, list_state: &mut ListState) -> (CmdResult, Option<StateChange>) {
    const MENU_SIZE: usize = 4;
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
                0 => Some(StateChange::ToUpgrade),
                1 => Some(StateChange::ToQuality),
                2 => Some(StateChange::ToSmelt),
                3 => Some(StateChange::ToForge),
                _ => None,
            };
            (CmdResult::Submit(tuirealm::State::None), state_change)
        }
        Cmd::Cancel => {
            // Backspace goes back to main menu
            game_state().current_screen = Id::Menu;
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        _ => (CmdResult::None, None),
    }
}
