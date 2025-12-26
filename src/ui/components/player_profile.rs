use std::{cell::RefCell, rc::Rc};

use crate::{entities::Player, ui::{common::ScreenId, menu_component::{MenuComponent, MenuItem}}};




pub struct PlayerProfileComponent{
    player: Player,
    back_menu: MenuComponent,
}

impl PlayerProfileComponent {
    pub fn new(player: &Player, back_screen: Rc<RefCell<Option<ScreenId>>>) -> Self {
        let back_menu = MenuComponent::new(vec![
            MenuItem {
                label: "Back".to_string(),
                action: Box::new(move || { *back_screen.borrow_mut() = Some(ScreenId::Menu); }),
            },
        ]);
        Self {
            player: player.clone(),
            back_menu
        }
    }
}
