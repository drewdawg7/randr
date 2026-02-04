use bevy::prelude::*;

use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_monster_compendium_modal;

use super::drops::{update_drops_display, update_drops_list_colors};
use super::input::{handle_compendium_navigation, handle_compendium_tab};
use super::list::{update_compendium_mob_sprite, update_monster_list_display};
use super::state::{CompendiumListState, CompendiumViewState, DropsListState, MonsterCompendiumModal};
use super::stats::update_stats_display;

pub struct MonsterCompendiumPlugin;

impl Plugin for MonsterCompendiumPlugin {
    fn build(&self, app: &mut App) {
        app.register_modal::<MonsterCompendiumModal>()
            .init_resource::<CompendiumListState>()
            .init_resource::<DropsListState>()
            .init_resource::<CompendiumViewState>()
            .add_systems(
                Update,
                (
                    modal_close_system::<MonsterCompendiumModal>,
                    (
                        handle_compendium_tab,
                        handle_compendium_navigation,
                        update_monster_list_display,
                        update_compendium_mob_sprite,
                        update_stats_display,
                        update_drops_display,
                        update_drops_list_colors,
                    )
                        .run_if(in_monster_compendium_modal),
                ),
            );
    }
}
