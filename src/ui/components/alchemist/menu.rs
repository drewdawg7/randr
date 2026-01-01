use ratatui::{
    layout::Rect,
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
    selection_prefix, COIN, RETURN_ARROW,
};
use crate::ui::theme as colors;

use super::StateChange;

pub const FLASK: char = '\u{F0093}';

fn alchemist_header(name: &str, gold: i32) -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::styled(name.to_string(), Style::default().fg(colors::BRIGHT_VIOLET)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().fg(colors::YELLOW)),
            Span::raw(format!("{}", gold)),
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

    let selected = list_state.selected().unwrap_or(0);
    let menu_items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 0),
            Span::styled(format!("{}", FLASK), Style::default().fg(colors::MYSTIC_GLOW)),
            Span::raw(" Brew Potions"),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 1),
            Span::raw(format!("{} Back", RETURN_ARROW)),
        ])),
    ];

    let menu = List::new(menu_items);
    frame.render_stateful_widget(menu, content_area, list_state);
}

pub fn handle(cmd: Cmd, list_state: &mut ListState) -> (CmdResult, Option<StateChange>) {
    const MENU_SIZE: usize = 2;
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
                1 => {
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
