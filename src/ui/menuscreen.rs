use std::io::Stdout;

use crossterm::{style::{Attribute, Color, Print, Stylize}, terminal};

use crate::ui::{move_down, move_up, print_to_screen, reset_screen, Screen, ScreenId, UIAction};


pub struct MenuScreen {
    selected: usize,
    options: Vec<String>,
}

impl MenuScreen {
    pub fn new(options: Vec<String>) -> Self {
        Self { selected: 0, options }
    }
}

impl Screen for MenuScreen {
    fn draw(&self, stdout: &mut Stdout) {
        reset_screen(stdout);
        let _ = terminal::disable_raw_mode();
        let options = ["Fight", "Store", "Quit"];
        for (i, opt) in options.iter().enumerate() {
            if i == self.selected {
                print_to_screen(
                    stdout,
                    Print(
                        "> "
                        .with(Color::Yellow)
                    )
                );
                print_to_screen(
                    stdout,
                    Print(
                        opt
                        .with(Color::Yellow)
                        .attribute(Attribute::Underlined)
                    )
                );
            } else {
                print_to_screen(stdout, Print("  "));
                print_to_screen(stdout, Print(*opt));
            }
            print_to_screen(stdout, Print("\n"));
        }
        print_to_screen(stdout, Print("\nUse ↑/↓ and Enter. (Esc to quit)\n"));
        let _ = terminal::enable_raw_mode();
    }

    fn handle(&mut self, action: UIAction) -> ScreenId{
        match action {
            UIAction::Up => {
                self.selected = move_up(self.selected, self.options.len());
                ScreenId::Menu
            },
            UIAction::Down => {
                self.selected = move_down(self.selected, self.options.len());
                ScreenId::Menu
            },
            UIAction::Activate => match self.selected {
                0 => ScreenId::Fight,
                1 => ScreenId::Store,
                _ => ScreenId::Quit,
            },
            UIAction::Back | UIAction::Quit => ScreenId::Quit,
        }
    }
}
