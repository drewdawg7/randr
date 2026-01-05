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
    entities::progression::{HasProgression, Progression},
    stats::HasStats,
    system::game_state,
    ui::Id,
};
use crate::ui::components::utilities::{
    list_move_down, list_move_up, render_location_header,
    selection_prefix, COIN, CROSSED_SWORDS, DOUBLE_ARROW_UP, HEART, PICKAXE,
};
use crate::ui::theme as colors;

pub enum StateChange {
    ToFight,
    ToMine,
}

fn field_header() -> Vec<Line<'static>> {
    let gs = game_state();
    let field = &gs.town.field;
    let player = &gs.player;

    // Get player stats
    let current_hp = player.hp();
    let max_hp = player.max_hp();
    let gold = player.gold();
    let progression = player.progression();
    let level = progression.level;
    let current_xp = progression.xp;
    let xp_to_next = Progression::xp_to_next_level(level);

    let text_style = Style::default().fg(colors::WHITE);
    vec![
        // Line 1: Field name
        Line::from(vec![
            Span::styled(field.name.clone(), Style::default().fg(colors::FOREST_GREEN)),
        ]),
        // Line 2: HP | Level XP | Gold
        Line::from(vec![
            Span::styled(format!("{} ", HEART), Style::default().fg(colors::RED)),
            Span::styled(format!("{}/{}", current_hp, max_hp), text_style),
            Span::styled("  |  ", text_style),
            Span::styled(format!("{} ", DOUBLE_ARROW_UP), Style::default().fg(colors::CYAN)),
            Span::styled(format!("{} ", level), text_style),
            Span::styled(format!("{}/{}", current_xp, xp_to_next), text_style),
            Span::styled("  |  ", text_style),
            Span::styled(format!("{} ", COIN), Style::default().fg(colors::YELLOW)),
            Span::styled(format!("{}", gold), text_style),
        ]),
    ]
}

pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let gs = game_state();

    // Render header with field name and get remaining area
    let header_lines = field_header();
    let content_area = render_location_header(frame, area, header_lines, colors::FIELD_BG, colors::FOREST_GREEN);

    // Center the menu vertically and horizontally
    const MENU_HEIGHT: u16 = 3;
    const MENU_WIDTH: u16 = 24; // Wider to fit mine timer (X:XX)

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
    let dim_style = Style::default().fg(colors::GRANITE);

    // Get mine regeneration timer
    let mine = &gs.town.mine;
    let regen_secs = mine.time_until_regeneration();
    let regen_mins = regen_secs / 60;
    let regen_secs_remainder = regen_secs % 60;
    let timer_text = format!(" ({}:{:02})", regen_mins, regen_secs_remainder);

    let menu_items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 0),
            Span::styled(format!("{}", CROSSED_SWORDS), Style::default().fg(colors::LIGHT_STONE)),
            Span::styled(" Fight", text_style),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 1),
            Span::styled(format!("{}", PICKAXE), Style::default().fg(colors::GRANITE)),
            Span::styled(" Mine", text_style),
            Span::styled(timer_text, dim_style),
        ])),
    ];

    let menu = List::new(menu_items);
    frame.render_stateful_widget(menu, centered_area, list_state);
}

pub fn handle(cmd: Cmd, list_state: &mut ListState) -> (CmdResult, Option<StateChange>) {
    const MENU_SIZE: usize = 2; // Fight, Mine

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
                0 => Some(StateChange::ToFight),
                1 => Some(StateChange::ToMine),
                _ => None,
            };
            (CmdResult::Submit(tuirealm::State::None), state_change)
        }
        Cmd::Cancel => {
            // Backspace goes back to main menu
            game_state().ui.current_screen = Id::Menu;
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        _ => (CmdResult::None, None),
    }
}
