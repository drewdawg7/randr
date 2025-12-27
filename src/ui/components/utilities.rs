
use crate::{system::game_state, ui::{common::ScreenId, menu_component::{MenuComponent, MenuItem}}};

pub const HEART: char          = '\u{2764}';
pub const COIN: char           = '\u{26C3}';
pub const CROSSED_SWORDS: char = '\u{2694}';


pub fn back_button(back_screen: ScreenId ) -> MenuComponent {
        MenuComponent::new(vec![
            MenuItem {
                label: "Back".to_string(),

                action: Box::new(move || {
                    game_state().current_screen = back_screen;
                })
            },
        ])
}
