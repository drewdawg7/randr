use crossterm::terminal;
use game::{game_state, init_game_state, GameState, HasInventory, ItemId};
use game::Id;

fn main() -> std::io::Result<()> {
    init_game_state(GameState::default());
    let game_state = game_state();
    game_state.initialize();

    if let Some(sword) = game_state.spawn_item(ItemId::BonkStick) {
        let _ = game_state.player.add_to_inv(sword);
    }
    if let Some(ring) = game_state.spawn_item(ItemId::ImbaRing) {
        let _ = game_state.player.add_to_inv(ring);
    }
    if let Some(upgrade_stone) = game_state.spawn_item(ItemId::QualityUpgradeStone) {
        let _ = game_state.player.add_to_inv(upgrade_stone);
    }
    if let Some(tome) = game_state.spawn_item(ItemId::ApprenticeTome) {
        let _ = game_state.player.add_to_inv(tome);
    }



    loop {
        let current = game_state.current_screen;
        if current == Id::Quit {
            break;
        }
        let _ = game_state.run_current_screen();
    }

    terminal::disable_raw_mode()
}
