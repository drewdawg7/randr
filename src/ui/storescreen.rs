
use std::io::Stdout;
use crossterm::{style::{Attribute, Color, Print, Stylize}, terminal};
use crate::{store::Store, ui::{ print_to_screen, reset_screen, Header, Screen, ScreenId, Table, UIAction}};

pub struct StoreScreen {
    store: Store,
}

impl StoreScreen {
    pub fn new(store: Store) -> Self {
        Self { store }
    }
}

impl Screen for StoreScreen {

    fn draw(&self, stdout: &mut Stdout) {
        let _ = terminal::disable_raw_mode();
        reset_screen(stdout);
        print_to_screen(
            stdout,
            Print(
                self.store.name.clone()
                .with(Color::Green)
                .attribute(Attribute::Bold)
                .attribute(Attribute::Underlined)
            )
        );
        print_to_screen(stdout, Print("\n"));
        let table = Table::from_items(
            [
                Header::new("Item"),
                Header::new("Price"),
                Header::new("Quantity"),
            ],
            &self.store.inventory,
            |si| [
                si.item.name.to_string(), 
                format!("{}g", si.price),
                si.quantity.to_string()
            ]
        );
        table.print(stdout);
        print_to_screen(stdout, Print("\n"));
        print_to_screen(stdout, Print("> Back"));
        print_to_screen(stdout, Print("\n"));
        let _ = terminal::enable_raw_mode();
    }

    fn handle(&mut self, action: UIAction) -> ScreenId {
        match action {
            UIAction::Activate | UIAction::Back => ScreenId::Menu,
            _ => ScreenId::Store,
        }
    }
}

