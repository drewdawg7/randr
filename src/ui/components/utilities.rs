use crate::{system::game_state, ui::{Id, menu_component::{MenuComponent, MenuItem}}};

pub const HEART: char          = '\u{F004}';
pub const COIN: char           = '\u{EDE8}';
pub const CROSSED_SWORDS: char = '\u{f0787}';
pub const CHECKED: char        = '\u{F14A}';
pub const UNCHECKED: char      = '\u{F0C8}';
pub const W_DIAMOND: char      = '\u{25C6}';
pub const B_DIAMOND: char      = '\u{25C7}';
pub const STORE: char          = '\u{ee17}';
pub const PERSON: char         = '\u{F415}';
pub const SHIRT: char          = '\u{EE1C}';
pub const OPEN_DOOR: char      = '\u{F081C}';
pub fn back_button(back_screen: Id) -> MenuComponent {
    MenuComponent::new(vec![
        MenuItem {
            label: "Back".to_string(),
            action: Box::new(move || {
                game_state().current_screen = back_screen;
            }),
        },
    ])
}
