use std::io::Stdout;

use crossterm::{event::{Event, KeyCode, KeyEvent}};
pub enum ScreenId {
    Menu,
    Store,
    Fight,
    Quit
}

pub enum UIAction {
    Up,
    Down,
    Activate,
    Back,
    Quit,
}

pub trait Screen {
    fn draw(&self, stdout: &mut Stdout);
    fn handle(&mut self, action: UIAction) -> ScreenId;
}



pub fn key_to_action(ev: Event) -> Option<UIAction> {
    if let Event::Key(KeyEvent {code, ..}) = ev {
        Some(match code {
            KeyCode::Up    => UIAction::Up,
            KeyCode::Down  => UIAction::Down,
            KeyCode::Enter => UIAction::Activate,
            KeyCode::Esc   => UIAction::Back,
            _ => return None,

        })
    } else {
        None
    }
}
