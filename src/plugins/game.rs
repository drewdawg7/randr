use bevy::prelude::*;

use crate::dungeon::{DungeonPlugin, FloorId};
use crate::input::GameAction;
use crate::location::LocationId;
use crate::navigation::NavigationPlugin;
use crate::states::AppState;
use crate::ui::screens::modal::ModalType;

use super::{
    CoreGamePlugins, GameMechanicsPlugins, InfrastructurePlugins, ScreenPlugins,
    UiInfrastructurePlugins, UiWidgetPlugins,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Infrastructure: assets, states, input, camera, tiled
        app.add_plugins(InfrastructurePlugins);

        // Navigation (builder-based, stays inline)
        app.add_plugins(
            NavigationPlugin::new()
                .state(AppState::Dungeon)
                    .on(GameAction::OpenInventory, ModalType::Inventory)
                    .on(GameAction::OpenProfile, ModalType::Profile)
                    .on(GameAction::OpenCompendium, ModalType::MonsterCompendium)
                    .on(GameAction::OpenSkills, ModalType::SkillsModal)
                .global()
                    .on(GameAction::OpenKeybinds, AppState::Keybinds)
                .build(),
        );

        // Dungeon (builder-based, stays inline)
        app.add_plugins(
            DungeonPlugin::new()
                .location(LocationId::Home)
                    .floor(FloorId::HomeFloor)
                .location(LocationId::MainDungeon)
                    .floor(FloorId::MainDungeon1)
                    .floor(FloorId::MainDungeon2)
                    .floor(FloorId::MainDungeon3)
                .build(),
        );

        // Core game systems
        app.add_plugins(CoreGamePlugins);

        // Game mechanics
        app.add_plugins(GameMechanicsPlugins);

        // UI infrastructure
        app.add_plugins(UiInfrastructurePlugins);

        // UI widgets
        app.add_plugins(UiWidgetPlugins);

        // Screens
        app.add_plugins(ScreenPlugins);
    }
}
