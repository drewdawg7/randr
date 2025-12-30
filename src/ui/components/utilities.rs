use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::theme::{self as colors, ColorExt};

use crate::{blacksmith::Blacksmith, store::Store, system::game_state, ui::Id};
use super::widgets::menu::{Menu, MenuItem};

pub const HEART: char           = '\u{F004}';
pub const COIN: char            = '\u{EDE8}';
pub const CROSSED_SWORDS: char  = '\u{f0787}';
pub const CHECKED: char         = '\u{F14A}';
pub const UNCHECKED: char       = '\u{F0C8}';
pub const STORE: char           = '\u{ee17}';
pub const PERSON: char          = '\u{F415}';
pub const SHIRT: char           = '\u{EE1C}';
pub const OPEN_DOOR: char       = '\u{F081C}';
pub const SHIELD: char          = '\u{F132}';
pub const ANVIL: char           = '\u{F089B}';
pub const DOUBLE_ARROW_UP: char = '\u{F102}';
pub const HOUSE: char           = '\u{F015}';
pub const RETURN_ARROW: char    = '\u{F17B1}';

pub fn blacksmith_header(blacksmith: &Blacksmith, gold: i32) -> Line<'static> {
    Line::from(vec![
        Span::styled(blacksmith.name.to_string(), Style::default().color(colors::CYAN)),
        Span::raw("  |  "),
        Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
        Span::raw(format!("{}", gold)),
        Span::raw("  |  "),
        Span::styled(format!("{} ", DOUBLE_ARROW_UP), Style::default().color(colors::BLUE)),
        Span::raw(format!("{}", blacksmith.max_upgrades)),
    ])
}

pub fn store_header(store: &Store, gold: i32) -> Line<'static> {
    Line::from(vec![
        Span::styled(store.name.to_string(), Style::default().color(colors::CYAN)),
        Span::raw("  |  "),
        Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
        Span::raw(format!("{}", gold)),
    ])
}

pub fn back_button(back_screen: Id) -> Menu {
    Menu::new(vec![
        MenuItem {
            label: format!("{} Back", RETURN_ARROW),
            action: Box::new(move || {
                game_state().current_screen = back_screen;
            }),
        },
    ])
}
