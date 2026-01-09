mod alchemist;
mod blacksmith;
mod dungeon;
mod field;
mod store;

use bevy::prelude::*;

pub use alchemist::{AlchemistTabPlugin, AlchemistTabState, spawn_alchemist_ui};
pub use blacksmith::{BlacksmithTabPlugin, BlacksmithTabState, spawn_blacksmith_ui};
pub use dungeon::{DungeonTabPlugin, DungeonTabState, spawn_dungeon_ui};
pub use field::{FieldTabPlugin, FieldTabState, spawn_field_ui};
pub use store::{StoreTabPlugin, StoreTabState, spawn_store_ui};

/// Plugin that bundles all tab plugins together.
pub struct TabsPlugin;

impl Plugin for TabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            StoreTabPlugin,
            BlacksmithTabPlugin,
            AlchemistTabPlugin,
            FieldTabPlugin,
            DungeonTabPlugin,
        ));
    }
}
