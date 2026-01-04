use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
    Frame,
};
use tuirealm::command::{Cmd, CmdResult};

use crate::{
    system::game_state,
    ui::components::utilities::{
        list_move_down, list_move_up, render_location_header, selection_prefix,
    },
    ui::theme as colors,
};

/// Dungeon entrance icon
pub const DUNGEON_ICON: char = '\u{f0787}'; // crossed swords (can be changed)

pub enum StateChange {
    EnterDungeon,
}

fn dungeon_header() -> Vec<Line<'static>> {
    let gs = game_state();

    let text_style = Style::default().fg(colors::WHITE);

    // Show dungeon status
    let status = if let Some(dungeon) = gs.dungeon() {
        if dungeon.is_completed() {
            "Completed"
        } else {
            let cleared = dungeon.cleared_count();
            let total = dungeon.room_count();
            return vec![
                Line::from(vec![
                    Span::styled("Dungeon", Style::default().fg(colors::LIGHT_STONE)),
                ]),
                Line::from(vec![
                    Span::styled(format!("Progress: {}/{} rooms", cleared, total), text_style),
                ]),
            ];
        }
    } else {
        "Unexplored"
    };

    vec![
        Line::from(vec![
            Span::styled("Dungeon", Style::default().fg(colors::LIGHT_STONE)),
        ]),
        Line::from(vec![
            Span::styled(format!("Status: {}", status), text_style),
        ]),
    ]
}

pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let header_lines = dungeon_header();
    let content_area = render_location_header(
        frame,
        area,
        header_lines,
        colors::MINE_BG, // Using stone/cave colors
        colors::LIGHT_STONE,
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

    // Menu options
    let selected = list_state.selected().unwrap_or(0);
    let text_style = Style::default().fg(colors::WHITE);

    let gs = game_state();
    let enter_text = if gs.dungeon().is_some() {
        "Continue"
    } else {
        "Enter"
    };

    let menu_items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 0),
            Span::styled(format!("{}", DUNGEON_ICON), Style::default().fg(colors::LIGHT_STONE)),
            Span::styled(format!(" {}", enter_text), text_style),
        ])),
    ];

    let menu = List::new(menu_items);
    frame.render_stateful_widget(menu, centered_area, list_state);
}

pub fn handle(cmd: Cmd, list_state: &mut ListState) -> (CmdResult, Option<StateChange>) {
    const MENU_SIZE: usize = 1; // Enter only

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
                0 => Some(StateChange::EnterDungeon),
                _ => None,
            };
            (CmdResult::Submit(tuirealm::State::None), state_change)
        }
        Cmd::Cancel => {
            // Handled by tab layer for go-back
            (CmdResult::None, None)
        }
        _ => (CmdResult::None, None),
    }
}
