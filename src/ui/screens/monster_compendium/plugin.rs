use bevy::prelude::*;

use super::input::{handle_compendium_close, handle_compendium_navigation};
use super::render::{spawn_monster_compendium, update_compendium_mob_sprite, update_monster_list_display};
use super::state::{CompendiumListState, SpawnMonsterCompendium};

/// Plugin that manages the monster compendium system.
pub struct MonsterCompendiumPlugin;

impl Plugin for MonsterCompendiumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CompendiumListState>().add_systems(
            Update,
            (
                handle_compendium_close,
                handle_compendium_navigation,
                update_monster_list_display,
                update_compendium_mob_sprite,
                spawn_monster_compendium.run_if(resource_exists::<SpawnMonsterCompendium>),
            ),
        );
    }
}
