use bevy::prelude::*;

use crate::assets::AssetPlugin as GameAssetPlugin;
use crate::game::{BlacksmithPlugin, CombatPlugin, CraftingPlugin, ItemPlugin, PlayerPlugin, StoragePlugin, StorageTransactionsPlugin, ToastPlugin};
use crate::location::StorePlugin;
use crate::input::InputPlugin;
use crate::plugins::{EconomyPlugin, MobPlugin, ToastListenersPlugin};
use crate::save_load::SaveLoadPlugin;
use crate::ui::screens::{
    DungeonPlugin, MonsterCompendiumPlugin, FightPlugin, InventoryModalPlugin, KeybindsPlugin,
    MainMenuPlugin, MinePlugin, ModalPlugin, ProfileModalPlugin, ProfilePlugin,
    TownPlugin,
};
use crate::states::StateTransitionPlugin;
use crate::ui::widgets::{CentralDetailPanelPlugin, GoldDisplayPlugin, IconValueRowPlugin, ItemGridPlugin, ItemStatsDisplayPlugin, PlayerStatsPlugin, StatRowPlugin};
use crate::ui::{MobAnimationPlugin, PlayerSpritePlugin};

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
            StorePlugin,
            ItemPlugin,
            CombatPlugin,
            CraftingPlugin,
            BlacksmithPlugin,
            StorageTransactionsPlugin,
            MobPlugin,
            EconomyPlugin,
            SaveLoadPlugin,
        ));

        // Additional core plugins
        app.add_plugins((ToastPlugin, ToastListenersPlugin, ModalPlugin, PlayerStatsPlugin, ItemGridPlugin, GoldDisplayPlugin, CentralDetailPanelPlugin, ItemStatsDisplayPlugin, StatRowPlugin, IconValueRowPlugin, MobAnimationPlugin, PlayerSpritePlugin));

        // Screens and modals
        app.add_plugins((
            MainMenuPlugin,
            ProfilePlugin,
            ProfileModalPlugin,
            InventoryModalPlugin,
            MonsterCompendiumPlugin,
            KeybindsPlugin,
            TownPlugin,
            DungeonPlugin,
            MinePlugin,
            FightPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
