use bevy::prelude::*;
use bevy_ecs_tiled::prelude::TilePos;

use crate::combat::ActiveCombat;
use crate::mob::{Health, MobId};
use crate::ui::focus::SelectionState;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;
use crate::ui::sprite_marker::{SpriteData, SpriteMarker};
use crate::ui::{MobSpriteSheets, PlayerSpriteSheet};

use super::render::do_spawn_fight_modal;

/// Marker component for the fight modal root entity.
#[derive(Component)]
pub struct FightModalRoot;

/// Marker component for the player sprite in the fight modal.
#[derive(Component)]
pub struct FightModalPlayerSprite;

impl SpriteMarker for FightModalPlayerSprite {
    type Resources = Res<'static, PlayerSpriteSheet>;

    fn resolve(&self, sheet: &Res<PlayerSpriteSheet>) -> Option<SpriteData> {
        if !sheet.is_loaded() {
            return None;
        }
        Some(SpriteData {
            texture: sheet.texture.as_ref()?.clone(),
            layout: sheet.layout.as_ref()?.clone(),
            animation: sheet.animation.clone().into(),
            flip_x: false,
        })
    }
}

/// Marker component for the mob sprite in the fight modal.
#[derive(Component)]
pub struct FightModalMobSprite {
    pub mob_id: MobId,
}

impl SpriteMarker for FightModalMobSprite {
    type Resources = Res<'static, MobSpriteSheets>;

    fn resolve(&self, sheets: &Res<MobSpriteSheets>) -> Option<SpriteData> {
        let sheet = sheets.get(self.mob_id)?;
        Some(SpriteData {
            texture: sheet.texture.clone(),
            layout: sheet.layout.clone(),
            animation: sheet.animation.clone().into(),
            flip_x: true, // Flip to face left toward the player
        })
    }
}

#[derive(Resource)]
pub struct FightModalMob {
    pub mob_id: MobId,
    pub pos: TilePos,
    pub entity: Entity,
}

/// Type-safe handle for the fight modal.
pub struct FightModal;

impl RegisteredModal for FightModal {
    type Root = FightModalRoot;
    const MODAL_TYPE: ModalType = ModalType::FightModal;

    fn spawn(world: &mut World) {
        world.init_resource::<FightModalButtonSelection>();
        world.run_system_cached(do_spawn_fight_modal).ok();
    }

    fn cleanup(world: &mut World) {
        if let Some(fight_mob) = world.get_resource::<FightModalMob>() {
            let entity = fight_mob.entity;
            if let Some(mut health) = world.get_mut::<Health>(entity) {
                health.current = health.max;
            }
        }
        world.remove_resource::<FightModalMob>();
        world.remove_resource::<FightModalButtonSelection>();
        world.remove_resource::<ActiveCombat>();
    }
}

/// Button selection options in the fight modal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FightModalButton {
    #[default]
    Ok,
    Cancel,
}

/// Resource tracking which button is selected in the fight modal.
#[derive(Resource, Default)]
pub struct FightModalButtonSelection {
    pub selected: FightModalButton,
}

impl SelectionState for FightModalButtonSelection {
    fn selected(&self) -> usize {
        match self.selected {
            FightModalButton::Ok => 0,
            FightModalButton::Cancel => 1,
        }
    }

    fn count(&self) -> usize {
        2
    }

    fn set_selected(&mut self, index: usize) {
        self.selected = match index {
            0 => FightModalButton::Ok,
            _ => FightModalButton::Cancel,
        };
    }
}

/// Marker component for the OK button.
#[derive(Component)]
pub struct FightModalOkButton;

/// Marker component for the Cancel button.
#[derive(Component)]
pub struct FightModalCancelButton;

/// Marker for player health bar in fight modal.
#[derive(Component)]
pub struct FightModalPlayerHealthBar;

/// Marker for mob health bar in fight modal.
#[derive(Component)]
pub struct FightModalMobHealthBar;
