use std::io::{self, Stdout, Write};


use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self},
    style::Print,

};
use game::{store::Store, ui::{enter_alternate_screen, leave_alternate_screen, move_down, move_up, print_to_screen, reset_screen}};


#[derive(Debug, Clone, Copy)]
pub enum MenuChoice {
    Fight,
    Store,
    Quit
}
pub fn run_menu(store: &Store) -> std::io::Result<MenuChoice> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    enter_alternate_screen(&mut stdout);

    let options = ["Fight", "Store", "Quit"];
    let mut selected: usize = 0;

    loop {
        reset_screen(&mut stdout);
        for (i, opt) in options.iter().enumerate() {
            if i == selected {
                print_to_screen(&mut stdout, Print("> "));
                print_to_screen(&mut stdout, Print(*opt));
                print_to_screen(&mut stdout, Print("\r\n"));
            } else {
                print_to_screen(&mut stdout, Print(" "));
                print_to_screen(&mut stdout, Print(*opt));
                print_to_screen(&mut stdout, Print("\r\n"));
            }
        }
        
        print_to_screen(&mut stdout,  Print("\nUse ↑/↓ and Enter. (Esc to quit)\r\n"));
        stdout.flush()?; // fine to keep
        

        let ev = event::read()?;
        if let Event::Key(key) = ev {
            match key.code {
                KeyCode::Up => {
                    selected = move_up(selected, options.len());
                }
                KeyCode::Down => {
                    selected = move_down(selected, options.len());
                }
                KeyCode::Enter => {
                    match selected {
                        0 => {
                            cleanup_tui(&mut stdout)?;
                            return Ok(MenuChoice::Fight);
                        }
                        1 => {
                            reset_screen(&mut stdout);
                            run_store_screen(&mut stdout, store)?;
                            return Ok(MenuChoice::Store);
                        }
                        _ => {
                            cleanup_tui(&mut stdout)?;
                            return Ok(MenuChoice::Quit);
                        }
                    }
                }
                KeyCode::Esc => {
                    terminal::disable_raw_mode()?;
                    leave_alternate_screen(&mut stdout);
                    return Ok(MenuChoice::Quit);
                }
                _ => {}
            }
        }
    }

}


fn run_store_screen(stdout: &mut Stdout, store: &Store) -> std::io::Result<()> {
    terminal::disable_raw_mode()?;
    print_to_screen(stdout, Print(&store));
    print_to_screen(stdout, Print("\n> Back"));
    print_to_screen(stdout, Print("Press Enter to go back. (Esc also works)"));
    terminal::enable_raw_mode()?;
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter | KeyCode::Esc => {
                    reset_screen(stdout);
                    break
                },
                _ => {}
            }
        }
    }

    Ok(())
}

fn cleanup_tui(stdout: &mut io::Stdout) -> std::io::Result<()> {
    terminal::disable_raw_mode()?;
    Ok(())
}
