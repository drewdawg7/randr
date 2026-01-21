//! Fight modal state and components.

use bevy::prelude::*;

use crate::mob::MobId;

/// Marker component for the fight modal root entity.
#[derive(Component)]
pub struct FightModalRoot;

/// Marker component for the player sprite in the fight modal.
#[derive(Component)]
pub struct FightModalPlayerSprite;

/// Marker component for the mob sprite in the fight modal.
#[derive(Component)]
pub struct FightModalMobSprite {
    pub mob_id: MobId,
}

/// Resource storing the mob encountered for the fight modal.
#[derive(Resource)]
pub struct FightModalMob {
    pub mob_id: MobId,
}

/// Marker resource to trigger spawning the fight modal.
#[derive(Resource)]
pub struct SpawnFightModal;
