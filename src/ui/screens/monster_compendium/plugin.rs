use bevy::prelude::*;

use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};

use super::input::{handle_compendium_navigation, handle_compendium_tab};
use super::render::{
    spawn_monster_compendium, update_compendium_mob_sprite, update_drops_display,
    update_drops_list_colors, update_monster_list_display,
};
use super::state::{
    CompendiumListState, DropsListState, MonsterCompendiumModal, SpawnMonsterCompendium,
};

pub struct MonsterCompendiumPlugin;

impl Plugin for MonsterCompendiumPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<MonsterCompendiumModal>()
            .init_resource::<CompendiumListState>()
            .init_resource::<DropsListState>()
            .add_systems(
                Update,
                (
                    modal_close_system::<MonsterCompendiumModal>,
                    handle_compendium_tab,
                    handle_compendium_navigation,
                    update_monster_list_display,
                    update_compendium_mob_sprite,
                    update_drops_display,
                    update_drops_list_colors,
                    spawn_monster_compendium.run_if(resource_exists::<SpawnMonsterCompendium>),
                ),
            );
    }
}
