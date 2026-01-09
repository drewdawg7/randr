use bevy::prelude::*;

use crate::assets::AssetPlugin as GameAssetPlugin;
use crate::game::{BlacksmithPlugin, CombatPlugin, CraftingPlugin, DungeonPlugin, ItemPlugin, PlayerPlugin, StoragePlugin, StoreTransactionsPlugin, ToastPlugin};
use crate::input::InputPlugin;
use crate::plugins::{EconomyPlugin, MobPlugin, ToastListenersPlugin};
use crate::save_load::SaveLoadPlugin;
use crate::screens::{
    DungeonScreenPlugin, FightPlugin, InventoryModalPlugin, KeybindsPlugin, MainMenuPlugin,
    MinePlugin, ModalPlugin, ProfileModalPlugin, ProfilePlugin, SpellTestModalPlugin, TownPlugin,
};
use crate::states::StateTransitionPlugin;
use crate::ui::widgets::{ItemGridPlugin, PlayerStatsPlugin};

/// Core game plugin that bundles all game systems.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Core systems
        app.add_plugins((
            StateTransitionPlugin,
            GameAssetPlugin,
            InputPlugin,
            PlayerPlugin,
            StoragePlugin,
            DungeonPlugin,
            ItemPlugin,
            CombatPlugin,
            CraftingPlugin,
            BlacksmithPlugin,
            StoreTransactionsPlugin,
            MobPlugin,
            EconomyPlugin,
            SaveLoadPlugin,
        ));

        // Additional core plugins
        app.add_plugins((ToastPlugin, ToastListenersPlugin, ModalPlugin, PlayerStatsPlugin, ItemGridPlugin));

        // Screens and modals
        app.add_plugins((
            MainMenuPlugin,
            ProfilePlugin,
            ProfileModalPlugin,
            InventoryModalPlugin,
            SpellTestModalPlugin,
            KeybindsPlugin,
            TownPlugin,
            MinePlugin,
            DungeonScreenPlugin,
            FightPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
