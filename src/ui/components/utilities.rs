use std::{cell::RefCell, rc::Rc};

use crate::ui::{common::ScreenId, menu_component::{MenuComponent, MenuItem}};



pub fn back_button(back_screen: Rc<RefCell<Option<ScreenId>>> ) -> MenuComponent {

        MenuComponent::new(vec![
            MenuItem {
                label: "Back".to_string(),
                action: Box::new(move || {*back_screen.borrow_mut() = Some(ScreenId::Menu);}),
            },
        ])
}
