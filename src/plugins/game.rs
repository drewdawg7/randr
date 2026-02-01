use bevy::prelude::*;

use crate::assets::AssetPlugin as GameAssetPlugin;
use crate::dungeon::{DungeonPlugin, FloorId};
use crate::game::{BlacksmithPlugin, CombatPlugin, CraftingCompletePlugin, CraftingPlugin, ItemPlugin, MerchantPlugin, PlayerPlugin, StoragePlugin, StorageTransactionsPlugin, ToastPlugin};
use crate::input::{GameAction, InputPlugin};
use crate::location::{LocationId, StorePlugin};
use crate::navigation::NavigationPlugin;
use crate::plugins::{EconomyPlugin, MobPlugin, ToastListenersPlugin};
use crate::skills::SkillsPlugin;
use crate::states::{AppState, StateTransitionPlugin};
use crate::ui::screens::modal::ModalType;
use crate::ui::screens::{
    AnvilModalPlugin, DungeonScreenPlugin, FightModalPlugin, ForgeModalPlugin,
    InventoryModalPlugin, KeybindsPlugin, MainMenuPlugin, MerchantModalPlugin, ModalPlugin,
    MonsterCompendiumPlugin, ProfilePlugin, ResultsModalPlugin, SkillsModalPlugin,
};
use crate::ui::widgets::{ItemDetailPanePlugin, ItemDetailDisplayPlugin, ColumnPlugin, GoldDisplayPlugin, IconValueRowPlugin, ItemGridPlugin, ItemStatsDisplayPlugin, OutlinedTextPlugin, PlayerStatsPlugin, RowPlugin, StackPlugin, StatRowPlugin};
use crate::ui::{MobAnimationPlugin, PlayerSpritePlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            StateTransitionPlugin,
            GameAssetPlugin,
            InputPlugin,
            NavigationPlugin::new()
                .state(AppState::Dungeon)
                    .on(GameAction::OpenInventory, ModalType::Inventory)
                    .on(GameAction::OpenProfile, ModalType::Profile)
                    .on(GameAction::OpenCompendium, ModalType::MonsterCompendium)
                    .on(GameAction::OpenSkills, ModalType::SkillsModal)
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
            CraftingCompletePlugin,
            MerchantPlugin,
            StorageTransactionsPlugin,
            MobPlugin,
            EconomyPlugin,
            SkillsPlugin,
        ));

        app.add_plugins((ToastPlugin, ToastListenersPlugin, ModalPlugin, PlayerStatsPlugin, ItemGridPlugin, GoldDisplayPlugin, ItemDetailPanePlugin, ItemDetailDisplayPlugin, ItemStatsDisplayPlugin));
        app.add_plugins((OutlinedTextPlugin, StatRowPlugin, IconValueRowPlugin, MobAnimationPlugin, PlayerSpritePlugin, RowPlugin, ColumnPlugin, StackPlugin));

        app.add_plugins((
            MainMenuPlugin,
            ProfilePlugin,
            InventoryModalPlugin,
            MerchantModalPlugin,
            ForgeModalPlugin,
            AnvilModalPlugin,
            MonsterCompendiumPlugin,
            KeybindsPlugin,
            DungeonScreenPlugin,
            FightModalPlugin,
            ResultsModalPlugin,
            SkillsModalPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
