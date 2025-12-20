use std::io::{self, Write};


use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    style::Print,
    terminal::{ EnterAlternateScreen, LeaveAlternateScreen}

};


#[derive(Debug, Clone, Copy)]
pub enum MenuChoice {
    Fight,
    Quit
}
pub fn run_menu() -> std::io::Result<MenuChoice> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let options = ["Fight", "Quit"];
    let mut selected: usize = 0;

    loop {
        // redraw
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0),
            Print("== Main Menu ==\r\n\r\n"),
        )?;

        for (i, opt) in options.iter().enumerate() {
            if i == selected {
                execute!(stdout, Print("> "), Print(*opt), Print("\r\n"))?;
            } else {
                execute!(stdout, Print("  "), Print(*opt), Print("\r\n"))?;
            }
        }

        execute!(stdout, Print("\nUse ↑/↓ and Enter. (Esc to quit)\r\n"))?;
        stdout.flush()?; // fine to keep

        if let Event::Key(key) = event::read()? { match key.code {
            KeyCode::Up => {
                selected = if selected == 0 { options.len() - 1 } else { selected - 1 };
            }
            KeyCode::Down => {
                selected = (selected + 1) % options.len();
            }
            KeyCode::Enter => {
                terminal::disable_raw_mode()?;
                execute!(stdout, cursor::Show, LeaveAlternateScreen)?;
                return Ok(match selected {
                    0 => MenuChoice::Fight,
                    _ => MenuChoice::Quit,
                });
            }
            KeyCode::Esc => {
                terminal::disable_raw_mode()?;
                execute!(stdout, cursor::Show, LeaveAlternateScreen)?;
                return Ok(MenuChoice::Quit);
            }
            _ => {}
        } }
    }

}
