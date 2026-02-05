use bevy::prelude::*;

use super::state::ForgeSlotIndex;

pub const SLOT_SIZE: f32 = 48.0;
pub const SLOT_GAP: f32 = 8.0;
pub const LABEL_FONT_SIZE: f32 = 12.0;

#[derive(Component)]
pub struct ForgeSlotCell {
    pub slot_type: ForgeSlotIndex,
}

#[derive(Component)]
pub struct ForgeSlotItemSprite;

#[derive(Component)]
pub struct ForgeSlotQuantityText;
