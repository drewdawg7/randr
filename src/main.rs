
use crossterm::terminal;
use game::entities::mob::{MobKind};
use game::entities::{player, Player};
use game::item::definition::{ItemKind};
use game::system::{game_state, init_game_state, GameState};
use game::ui::common::{ScreenId};

use game::inventory::{EquipmentSlot, HasInventory};
use game::combat::enter_combat;


fn main() -> std::io::Result<()> {
    init_game_state(GameState::default());
    let game_state = game_state();
    game_state.initialize();
    let sword = game_state.spawn_item(ItemKind::Sword);


    game_state.player.equip_item(sword, EquipmentSlot::Weapon);




    // ---------- MAIN LOOP ----------
    loop {

        let mut current = game_state.current_screen;
        if current == ScreenId::Quit {
            break;
        }
        if current == ScreenId::Fight {
            let mut mob = game_state.spawn_mob(MobKind::Goblin);
            let combat_rounds = enter_combat(&mut game_state.player, &mut mob);
            game_state.init_fight(combat_rounds);
        }
        let _ = game_state.run_current_screen(&mut current);
    }
    // ---------- CLEANUP ----------
    terminal::disable_raw_mode()
}
