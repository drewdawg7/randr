use crossterm::terminal;
use game::{game_state, init_game_state, GameState};
use game::Id;
use game::{EquipmentSlot, HasInventory};
use game::ItemKind;

fn main() -> std::io::Result<()> {
    init_game_state(GameState::default());
    let game_state = game_state();
    game_state.initialize();
    let mut sword = game_state.spawn_item(ItemKind::Sword);
    let mut shield = game_state.spawn_item(ItemKind::BasicShield);

    game_state.player.equip_item(&mut sword, EquipmentSlot::Weapon);
    game_state.player.equip_item(&mut shield, EquipmentSlot::OffHand);

    loop {
        let current = game_state.current_screen;
        if current == Id::Quit {
            break;
        }
        let _ = game_state.run_current_screen();
    }

    terminal::disable_raw_mode()
}
