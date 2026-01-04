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
    selection_prefix, COIN,
};
use crate::ui::theme as colors;

use super::StateChange;

pub const FLASK: char = '\u{F0093}';

fn alchemist_header(name: &str, gold: i32) -> Vec<Line<'static>> {
    let text_style = Style::default().fg(colors::WHITE);
    vec![
        Line::from(vec![
            Span::styled(name.to_string(), Style::default().fg(colors::BRIGHT_VIOLET)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().fg(colors::YELLOW)),
            Span::styled(format!("{}", gold), text_style),
        ]),
    ]
}

pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let player_gold = game_state().player.gold();
    let alchemist = game_state().alchemist();

    let header_lines = alchemist_header(&alchemist.name, player_gold);
    let content_area = render_location_header(
        frame, area, header_lines, colors::ALCHEMIST_BG, colors::MYSTIC_PURPLE
    );

    // Center the menu vertically and horizontally
    const MENU_HEIGHT: u16 = 2;
    const MENU_WIDTH: u16 = 20;

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
            Span::styled(format!("{}", FLASK), Style::default().fg(colors::MYSTIC_GLOW)),
            Span::styled(" Brew Potions", text_style),
        ])),
    ];

    let menu = List::new(menu_items);
    frame.render_stateful_widget(menu, centered_area, list_state);
}

pub fn handle(cmd: Cmd, list_state: &mut ListState) -> (CmdResult, Option<StateChange>) {
    const MENU_SIZE: usize = 1;
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
                0 => Some(StateChange::ToBrew),
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
