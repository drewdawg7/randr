use std::{fmt::Display, io::{Stdout}};

use crossterm::{cursor, execute, style::{Attribute, Color, Print, Stylize}, terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen}};



pub fn reset_screen(stdout: &mut Stdout) {
    let _ = execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0,0)
    );

}

pub fn print_to_screen<T: Display>(stdout: &mut Stdout, to_print: Print<T>) {
    let _ = execute!(stdout, to_print);
}

pub fn enter_alternate_screen(stdout: &mut Stdout) {
    let _ = execute!(stdout, cursor::Show, EnterAlternateScreen);
}

pub fn leave_alternate_screen(stdout: &mut Stdout) {
    let _ = execute!(stdout, cursor::Hide, LeaveAlternateScreen);
}


pub fn move_up(selected: usize, num_options: usize) -> usize {
    if selected == 0 {
        num_options - 1
    } else {
        selected - 1
    }
}

pub fn move_down(selected: usize, num_options: usize) -> usize {
    (selected + 1) % num_options
}

pub fn menu_cursor(stdout: &mut Stdout) {
    print_to_screen(stdout, Print("> ".with(Color::Yellow)));
}

pub fn select_item(stdout: &mut Stdout, item: &str) {
    menu_cursor(stdout);
    
    print_to_screen(
        stdout,
        Print(
            item
            .with(Color::Yellow)
            .attribute(Attribute::Underlined)
        )
    );
}
