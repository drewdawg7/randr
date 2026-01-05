use crossterm::terminal;
use game::{game_state, init_game_state, GameState, ItemId, ManagesItems};
use game::Id;

fn main() -> std::io::Result<()> {
    init_game_state(GameState::default());
    let game_state = game_state();
    game_state.initialize();

    let _ = game_state.player.add_to_inv(ItemId::BonkStick.spawn());
    let _ = game_state.player.add_to_inv(ItemId::ImbaRing.spawn());
    let _ = game_state.player.add_to_inv(ItemId::QualityUpgradeStone.spawn());
    let _ = game_state.player.add_to_inv(ItemId::ApprenticeTome.spawn());



    loop {
        let current = game_state.current_screen;
        if current == Id::Quit {
            break;
        }
        let _ = game_state.run_current_screen();
    }

    terminal::disable_raw_mode()
}
