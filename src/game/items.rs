use bevy::prelude::*;
use uuid::Uuid;

use crate::inventory::EquipmentSlot;
use crate::item::ItemId;

/// Event fired when an item is equipped
#[derive(Event, Debug, Clone)]
pub struct ItemEquipped {
    pub item_uuid: Uuid,
    pub item_id: ItemId,
    pub slot: EquipmentSlot,
}

/// Event fired when an item is unequipped
#[derive(Event, Debug, Clone)]
pub struct ItemUnequipped {
    pub item_uuid: Uuid,
    pub item_id: ItemId,
    pub slot: EquipmentSlot,
}

/// Event fired when an item is used (consumables, etc.)
#[derive(Event, Debug, Clone)]
pub struct ItemUsed {
    pub item_uuid: Uuid,
    pub item_id: ItemId,
}

/// Event fired when an item is dropped from inventory
#[derive(Event, Debug, Clone)]
pub struct ItemDropped {
    pub item_uuid: Uuid,
    pub item_id: ItemId,
    pub quantity: u32,
}

/// Event fired when an item is picked up into inventory
#[derive(Event, Debug, Clone)]
pub struct ItemPickedUp {
    pub item_uuid: Uuid,
    pub item_id: ItemId,
    pub quantity: u32,
    pub was_stacked: bool,
}

/// Plugin that registers item-related events
///
/// The inventory system is accessed through Player (player.inventory).
/// This plugin provides events for the UI to react to item-related actions.
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ItemEquipped>()
            .add_event::<ItemUnequipped>()
            .add_event::<ItemUsed>()
            .add_event::<ItemDropped>()
            .add_event::<ItemPickedUp>();
    }
}
