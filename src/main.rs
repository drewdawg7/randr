use crossterm::terminal;
use game::{game_state, init_game_state, GameState};
use game::Id;

fn main() -> std::io::Result<()> {
    init_game_state(GameState::default());
    let game_state = game_state();
    game_state.initialize();


    loop {
        let current = game_state.current_screen;
        if current == Id::Quit {
            break;
        }
        let _ = game_state.run_current_screen();
    }

    terminal::disable_raw_mode()
}
