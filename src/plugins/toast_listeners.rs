use bevy::{ecs::system::SystemParam, prelude::*};

use crate::game::{
    BrewingResult, GoldChanged, ItemDeposited, ItemDropped, ItemEquipped,
    ItemPickedUp, ItemUnequipped, ItemUsed, ItemWithdrawn, PlayerHealed,
    PlayerLeveledUp, ShowToast,
};
use super::{GoldEarned, GoldSpent, LootCollected, MobDefeated, TransactionCompleted};

/// SystemParam grouping all item-related event readers to reduce parameter count.
#[derive(SystemParam)]
struct ItemEventReaders<'w, 's> {
    picked_up: EventReader<'w, 's, ItemPickedUp>,
    equipped: EventReader<'w, 's, ItemEquipped>,
    unequipped: EventReader<'w, 's, ItemUnequipped>,
    used: EventReader<'w, 's, ItemUsed>,
    dropped: EventReader<'w, 's, ItemDropped>,
    deposited: EventReader<'w, 's, ItemDeposited>,
    withdrawn: EventReader<'w, 's, ItemWithdrawn>,
}

/// Plugin that listens to game events and triggers toast notifications
pub struct ToastListenersPlugin;

impl Plugin for ToastListenersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_player_events,
                listen_item_events,
                listen_combat_events,
                listen_economy_events,
                listen_brewing_events,
            ),
        );
    }
}

/// Listen to player-related events
fn listen_player_events(
    mut level_up_events: EventReader<PlayerLeveledUp>,
    mut healed_events: EventReader<PlayerHealed>,
    mut gold_changed_events: EventReader<GoldChanged>,
    mut toast_writer: EventWriter<ShowToast>,
) {
    // Level up notifications
    for event in level_up_events.read() {
        toast_writer.send(ShowToast::success(format!(
            "Level Up! You are now level {}",
            event.new_level
        )));
    }

    // Healing notifications (only for significant heals)
    for event in healed_events.read() {
        if event.amount >= 20 {
            toast_writer.send(ShowToast::success(format!("Healed {} HP", event.amount)));
        }
    }

    // Gold change notifications (only for significant amounts)
    for event in gold_changed_events.read() {
        if event.amount.abs() >= 50 {
            if event.amount > 0 {
                toast_writer.send(ShowToast::success(format!("Gained {} gold", event.amount)));
            } else {
                toast_writer.send(ShowToast::info(format!("Spent {} gold", -event.amount)));
            }
        }
    }
}

/// Listen to item-related events
fn listen_item_events(mut events: ItemEventReaders, mut toast_writer: EventWriter<ShowToast>) {
    // Item pickup notifications
    for event in events.picked_up.read() {
        let item_name = &event.item_id.spec().name;
        if event.quantity > 1 {
            toast_writer.send(ShowToast::success(format!(
                "Picked up {} x{}",
                item_name, event.quantity
            )));
        } else {
            toast_writer.send(ShowToast::success(format!("Picked up {}", item_name)));
        }
    }

    // Equipment notifications
    for event in events.equipped.read() {
        toast_writer.send(ShowToast::info(format!(
            "Equipped {} to {:?}",
            event.item_id.spec().name,
            event.slot
        )));
    }

    for event in events.unequipped.read() {
        toast_writer.send(ShowToast::info(format!(
            "Unequipped {} from {:?}",
            event.item_id.spec().name,
            event.slot
        )));
    }

    // Item use notifications
    for event in events.used.read() {
        toast_writer.send(ShowToast::info(format!("Used {}", event.item_id.spec().name)));
    }

    // Item drop notifications
    for event in events.dropped.read() {
        if event.quantity > 1 {
            toast_writer.send(ShowToast::warning(format!(
                "Dropped {} x{}",
                event.item_id.spec().name,
                event.quantity
            )));
        } else {
            toast_writer.send(ShowToast::warning(format!(
                "Dropped {}",
                event.item_id.spec().name
            )));
        }
    }

    // Storage notifications
    for event in events.deposited.read() {
        toast_writer.send(ShowToast::info(format!("Deposited {}", event.item_name)));
    }

    for event in events.withdrawn.read() {
        toast_writer.send(ShowToast::info(format!("Withdrew {}", event.item_name)));
    }
}

/// Listen to combat-related events
fn listen_combat_events(
    mut mob_defeated_events: EventReader<MobDefeated>,
    mut toast_writer: EventWriter<ShowToast>,
) {
    for event in mob_defeated_events.read() {
        toast_writer.send(ShowToast::success(format!(
            "Defeated {}!",
            event.mob_id.spec().name
        )));
    }
}

/// Listen to economy-related events
fn listen_economy_events(
    mut gold_earned_events: EventReader<GoldEarned>,
    mut gold_spent_events: EventReader<GoldSpent>,
    mut loot_collected_events: EventReader<LootCollected>,
    mut transaction_completed_events: EventReader<TransactionCompleted>,
    mut toast_writer: EventWriter<ShowToast>,
) {
    // Gold earned
    for event in gold_earned_events.read() {
        if event.amount >= 50 {
            toast_writer.send(ShowToast::success(format!("Earned {} gold", event.amount)));
        }
    }

    // Gold spent
    for event in gold_spent_events.read() {
        if event.amount >= 100 {
            toast_writer.send(ShowToast::info(format!("Spent {} gold", event.amount)));
        }
    }

    // Loot collected
    for event in loot_collected_events.read() {
        if event.total_items >= 3 {
            toast_writer.send(ShowToast::success(format!(
                "Collected {} items",
                event.total_items
            )));
        }
    }

    // Transaction completed
    for event in transaction_completed_events.read() {
        if event.price >= 100 {
            let action = if event.is_purchase { "Purchased" } else { "Sold" };
            toast_writer.send(ShowToast::info(format!(
                "{} {} for {} gold",
                action, event.item.name, event.price
            )));
        }
    }
}

/// Listen to brewing-related events
fn listen_brewing_events(
    mut brewing_events: EventReader<BrewingResult>,
    mut toast_writer: EventWriter<ShowToast>,
) {
    for event in brewing_events.read() {
        match event {
            BrewingResult::Success { item_name } => {
                toast_writer.send(ShowToast::success(format!("Crafted {}!", item_name)));
            }
            BrewingResult::InsufficientIngredients { recipe_name } => {
                toast_writer.send(ShowToast::warning(format!(
                    "Missing ingredients for {}",
                    recipe_name
                )));
            }
            BrewingResult::InventoryFull { item_name } => {
                toast_writer.send(ShowToast::error(format!(
                    "Inventory full - could not add {}",
                    item_name
                )));
            }
            BrewingResult::CraftingFailed { recipe_name } => {
                toast_writer.send(ShowToast::error(format!(
                    "Failed to craft {}",
                    recipe_name
                )));
            }
        }
    }
}
