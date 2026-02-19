//! Plugin groups for organized game initialization.

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_ecs_tiled::prelude::TiledPlugin;

use crate::assets::AssetPlugin as GameAssetPlugin;
use crate::camera::CameraPlugin;
use crate::crafting_station::CraftingStationPlugin;
use crate::combat::ActionCombatPlugin;
use crate::game::{
    BlacksmithPlugin, CombatPlugin, CraftingCompletePlugin, CraftingPlugin, ItemPlugin,
    MerchantPlugin, MiningPlugin, NpcInteractionsPlugin, PlayerPlugin, StoragePlugin,
    StorageTransactionsPlugin, ToastPlugin,
};
use crate::input::InputPlugin;
use crate::location::StorePlugin;
use crate::skills::SkillsPlugin;
use crate::states::StateTransitionPlugin;
use crate::ui::screens::{
    AnvilModalPlugin, DungeonScreenPlugin, ForgeModalPlugin, InventoryModalPlugin, KeybindsPlugin,
    MainMenuPlugin, MerchantModalPlugin, ModalPlugin, MonsterCompendiumPlugin, ProfilePlugin,
    ResultsModalPlugin, SkillsModalPlugin,
};
use crate::ui::widgets::{
    ColumnPlugin, GoldDisplayPlugin, IconValueRowPlugin, ItemDetailDisplayPlugin,
    ItemDetailPanePlugin, ItemGridPlugin, ItemStatsDisplayPlugin, OutlinedTextPlugin,
    PlayerStatsPlugin, RowPlugin, SelectorPlugin, StackPlugin, StatRowPlugin,
};
use crate::ui::{MobAnimationPlugin, PlayerSpritePlugin};

use super::{EconomyPlugin, MobPlugin, PhysicsDebugTogglePlugin, ToastListenersPlugin};

/// Infrastructure plugins: assets, states, input, camera, tiled maps.
pub struct InfrastructurePlugins;

impl PluginGroup for InfrastructurePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(AsepriteUltraPlugin)
            .add(TiledPlugin::default())
            .add(StateTransitionPlugin)
            .add(GameAssetPlugin)
            .add(InputPlugin)
            .add(CameraPlugin)
    }
}

/// Core game plugins: player, storage, items, combat, crafting.
pub struct CoreGamePlugins;

impl PluginGroup for CoreGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerPlugin)
            .add(StoragePlugin)
            .add(StorePlugin)
            .add(ItemPlugin)
            .add(CombatPlugin)
            .add(ActionCombatPlugin)
            .add(CraftingPlugin)
            .add(SkillsPlugin)
    }
}

/// Game mechanics plugins: NPCs, merchants, mining, economy.
pub struct GameMechanicsPlugins;

impl PluginGroup for GameMechanicsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(BlacksmithPlugin)
            .add(CraftingCompletePlugin)
            .add(CraftingStationPlugin)
            .add(MerchantPlugin)
            .add(MiningPlugin)
            .add(NpcInteractionsPlugin)
            .add(StorageTransactionsPlugin)
            .add(MobPlugin)
            .add(EconomyPlugin)
            .add(PhysicsDebugTogglePlugin)
    }
}

/// UI infrastructure plugins: toasts, modals.
pub struct UiInfrastructurePlugins;

impl PluginGroup for UiInfrastructurePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ToastPlugin)
            .add(ToastListenersPlugin)
            .add(ModalPlugin)
    }
}

/// UI widget plugins: reusable UI components.
pub struct UiWidgetPlugins;

impl PluginGroup for UiWidgetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ColumnPlugin)
            .add(GoldDisplayPlugin)
            .add(IconValueRowPlugin)
            .add(ItemDetailDisplayPlugin)
            .add(ItemDetailPanePlugin)
            .add(ItemGridPlugin)
            .add(ItemStatsDisplayPlugin)
            .add(MobAnimationPlugin)
            .add(OutlinedTextPlugin)
            .add(PlayerSpritePlugin)
            .add(PlayerStatsPlugin)
            .add(RowPlugin)
            .add(SelectorPlugin)
            .add(StackPlugin)
            .add(StatRowPlugin)
    }
}

/// Screen plugins: main menu, modals, game screens.
pub struct ScreenPlugins;

impl PluginGroup for ScreenPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(MainMenuPlugin)
            .add(ProfilePlugin)
            .add(InventoryModalPlugin)
            .add(MerchantModalPlugin)
            .add(ForgeModalPlugin)
            .add(AnvilModalPlugin)
            .add(MonsterCompendiumPlugin)
            .add(KeybindsPlugin)
            .add(DungeonScreenPlugin)
            .add(ResultsModalPlugin)
            .add(SkillsModalPlugin)
    }
}
