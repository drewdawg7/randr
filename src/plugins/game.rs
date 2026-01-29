use bevy::prelude::*;

use crate::assets::AssetPlugin as GameAssetPlugin;
use crate::dungeon::{DungeonPlugin, FloorId};
use crate::game::{BlacksmithPlugin, CombatPlugin, CraftingPlugin, ItemPlugin, PlayerPlugin, StoragePlugin, StorageTransactionsPlugin, ToastPlugin};
use crate::input::{GameAction, InputPlugin};
use crate::location::{LocationId, StorePlugin};
use crate::navigation::NavigationPlugin;
use crate::plugins::{EconomyPlugin, MobPlugin, ToastListenersPlugin};
use crate::save_load::SaveLoadPlugin;
use crate::states::{AppState, StateTransitionPlugin};
use crate::ui::screens::modal::ModalType;
use crate::ui::screens::{
    AnvilModalPlugin, DungeonScreenPlugin, FightModalPlugin, ForgeModalPlugin,
    InventoryModalPlugin, KeybindsPlugin, MainMenuPlugin, MerchantModalPlugin, MinePlugin,
    ModalPlugin, MonsterCompendiumPlugin, ProfileModalPlugin, ProfilePlugin, ResultsModalPlugin,
    TownPlugin,
};
use crate::ui::widgets::{ItemDetailPanePlugin, ColumnPlugin, GoldDisplayPlugin, IconValueRowPlugin, ItemGridPlugin, ItemStatsDisplayPlugin, OutlinedTextPlugin, PlayerStatsPlugin, RowPlugin, StackPlugin, StatRowPlugin};
use crate::ui::{MobAnimationPlugin, PlayerSpritePlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            StateTransitionPlugin,
            GameAssetPlugin,
            InputPlugin,
            NavigationPlugin::new()
                .state(AppState::Town)
                    .on(GameAction::OpenInventory, ModalType::Inventory)
                    .on(GameAction::OpenProfile, ModalType::Profile)
                    .on(GameAction::OpenCompendium, ModalType::MonsterCompendium)
                .state(AppState::Dungeon)
                    .on(GameAction::OpenInventory, ModalType::Inventory)
                    .on(GameAction::OpenProfile, ModalType::Profile)
                    .on(GameAction::OpenCompendium, ModalType::MonsterCompendium)
                .global()
                    .on(GameAction::OpenKeybinds, AppState::Keybinds)
                .build(),
            DungeonPlugin::new()
                .location(LocationId::Home)
                    .floor(FloorId::HomeFloor)
                .location(LocationId::MainDungeon)
                    .floor(FloorId::MainDungeon1)
                    .floor(FloorId::MainDungeon2)
                    .floor(FloorId::MainDungeon3)
                .build(),
            PlayerPlugin,
            StoragePlugin,
            StorePlugin,
            ItemPlugin,
            CombatPlugin,
            CraftingPlugin,
        ));

        app.add_plugins((
            BlacksmithPlugin,
            StorageTransactionsPlugin,
            MobPlugin,
            EconomyPlugin,
            SaveLoadPlugin,
        ));

        app.add_plugins((ToastPlugin, ToastListenersPlugin, ModalPlugin, PlayerStatsPlugin, ItemGridPlugin, GoldDisplayPlugin, ItemDetailPanePlugin, ItemStatsDisplayPlugin));
        app.add_plugins((OutlinedTextPlugin, StatRowPlugin, IconValueRowPlugin, MobAnimationPlugin, PlayerSpritePlugin, RowPlugin, ColumnPlugin, StackPlugin));

        app.add_plugins((
            MainMenuPlugin,
            ProfilePlugin,
            ProfileModalPlugin,
            InventoryModalPlugin,
            MerchantModalPlugin,
            ForgeModalPlugin,
            AnvilModalPlugin,
            MonsterCompendiumPlugin,
            KeybindsPlugin,
            TownPlugin,
            DungeonScreenPlugin,
            MinePlugin,
            FightModalPlugin,
            ResultsModalPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
