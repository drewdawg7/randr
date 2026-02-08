mod actions;
mod anvil;
mod combat;
mod compendium;
mod forge;
mod inventory;
mod merchant;
mod navigation;
mod results;
mod systems;

pub use actions::{GameAction, HeldDirection, NavigationDirection};
pub use systems::{clear_game_action_events, InputPlugin};

pub use anvil::{craft_anvil_recipe, navigate_anvil_grid, sync_anvil_recipes};
pub use combat::trigger_player_attack;
pub use compendium::{navigate_compendium, switch_compendium_panel};
pub use forge::{navigate_forge_ui, transfer_forge_items};
pub use inventory::{navigate_inventory_grid, toggle_equipment};
pub use merchant::{navigate_merchant_grid, process_transaction};
pub use navigation::{emit_move_intent, request_menu_transition};
pub use results::close_results_modal;
