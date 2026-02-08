use bevy::prelude::*;

use crate::input::{navigate_compendium, switch_compendium_panel};
use crate::ui::modal_registry::{modal_close_system, RegisterModalExt};
use crate::ui::screens::modal::in_monster_compendium_modal;
use crate::ui::FocusState;

use super::drops::{update_drops_display, update_drops_list_colors};
use super::list::{update_compendium_mob_sprite, update_monster_list_display};
use super::state::{
    CompendiumDropsSection, CompendiumListState, CompendiumMobSprite, CompendiumStatsSection,
    CompendiumViewState, DropsListState, MonsterCompendiumModal,
};
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
                        switch_compendium_panel,
                        navigate_compendium,
                        update_monster_list_display.run_if(
                            resource_changed::<CompendiumListState>
                                .or(
                                    resource_exists::<FocusState>
                                        .and(resource_changed::<FocusState>),
                                ),
                        ),
                        update_compendium_mob_sprite.run_if(
                            resource_changed::<CompendiumListState>
                                .or(any_match_filter::<Added<CompendiumMobSprite>>),
                        ),
                        update_stats_display.run_if(
                            resource_changed::<CompendiumListState>
                                .or(resource_changed::<CompendiumViewState>)
                                .or(any_match_filter::<Added<CompendiumStatsSection>>),
                        ),
                        update_drops_display.run_if(
                            resource_changed::<CompendiumListState>
                                .or(resource_changed::<CompendiumViewState>)
                                .or(any_match_filter::<Added<CompendiumDropsSection>>),
                        ),
                        update_drops_list_colors.run_if(
                            resource_changed::<DropsListState>
                                .or(
                                    resource_exists::<FocusState>
                                        .and(resource_changed::<FocusState>),
                                ),
                        ),
                    )
                        .run_if(in_monster_compendium_modal),
                ),
            );
    }
}
