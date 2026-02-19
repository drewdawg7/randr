use bevy::{ecs::system::SystemParam, prelude::*};

use crate::combat::{GoldGained, LootDropped, XpGained};
use crate::dungeon::{MineableEntityType, MiningResult};
use crate::game::{
    BrewingResult, GoldChanged, ItemDeposited, ItemDropped, ItemEquipped,
    ItemPickedUp, ItemUnequipped, ItemUsed, ItemWithdrawn, PlayerHealed,
    PlayerLeveledUp, ShowToast,
};
use crate::skills::SkillLeveledUp;
use super::{GoldEarned, GoldSpent, LootCollected, MobDefeated, TransactionCompleted};

/// Thresholds for toast notifications.
/// Values below these thresholds will not trigger toasts.
#[derive(Resource, Clone, Debug)]
pub struct ToastThresholds {
    /// Minimum heal amount to show toast. Default: 20
    pub heal_min: i32,
    /// Minimum gold change to show toast. Default: 50
    pub gold_change: i32,
    /// Minimum gold earned to show toast. Default: 50
    pub gold_earned: i32,
    /// Minimum gold spent to show toast. Default: 50 (symmetric with earned)
    pub gold_spent: i32,
    /// Minimum loot items to show toast. Default: 3
    pub loot_items: i32,
    /// Minimum transaction price to show toast. Default: 100
    pub transaction: i32,
}

impl Default for ToastThresholds {
    fn default() -> Self {
        Self {
            heal_min: 20,
            gold_change: 50,
            gold_earned: 50,
            gold_spent: 50, // Symmetric with gold_earned
            loot_items: 3,
            transaction: 100,
        }
    }
}

/// SystemParam grouping all item-related event readers to reduce parameter count.
#[derive(SystemParam)]
struct ItemEventReaders<'w, 's> {
    picked_up: MessageReader<'w, 's, ItemPickedUp>,
    equipped: MessageReader<'w, 's, ItemEquipped>,
    unequipped: MessageReader<'w, 's, ItemUnequipped>,
    used: MessageReader<'w, 's, ItemUsed>,
    dropped: MessageReader<'w, 's, ItemDropped>,
    deposited: MessageReader<'w, 's, ItemDeposited>,
    withdrawn: MessageReader<'w, 's, ItemWithdrawn>,
}

/// Plugin that listens to game events and triggers toast notifications
pub struct ToastListenersPlugin;

impl Plugin for ToastListenersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToastThresholds>()
            .add_systems(
            Update,
            (
                listen_player_events.run_if(
                    on_message::<PlayerLeveledUp>
                        .or(on_message::<PlayerHealed>)
                        .or(on_message::<GoldChanged>),
                ),
                listen_item_events.run_if(
                    on_message::<ItemPickedUp>
                        .or(on_message::<ItemEquipped>)
                        .or(on_message::<ItemUnequipped>)
                        .or(on_message::<ItemUsed>)
                        .or(on_message::<ItemDropped>)
                        .or(on_message::<ItemDeposited>)
                        .or(on_message::<ItemWithdrawn>),
                ),
                listen_combat_events.run_if(on_message::<MobDefeated>),
                listen_action_combat_events.run_if(
                    on_message::<GoldGained>
                        .or(on_message::<XpGained>)
                        .or(on_message::<LootDropped>),
                ),
                listen_economy_events.run_if(
                    on_message::<GoldEarned>
                        .or(on_message::<GoldSpent>)
                        .or(on_message::<LootCollected>)
                        .or(on_message::<TransactionCompleted>),
                ),
                listen_mining_events.run_if(on_message::<MiningResult>),
                listen_brewing_events.run_if(on_message::<BrewingResult>),
                listen_skill_events.run_if(on_message::<SkillLeveledUp>),
            ),
        );
    }
}

/// Listen to player-related events
fn listen_player_events(
    mut level_up_events: MessageReader<PlayerLeveledUp>,
    mut healed_events: MessageReader<PlayerHealed>,
    mut gold_changed_events: MessageReader<GoldChanged>,
    thresholds: Res<ToastThresholds>,
    mut toast_writer: MessageWriter<ShowToast>,
) {
    // Level up notifications
    for event in level_up_events.read() {
        toast_writer.write(ShowToast::success(format!(
            "Level Up! You are now level {}",
            event.new_level
        )));
    }

    // Healing notifications (only for significant heals)
    for event in healed_events.read() {
        if event.amount >= thresholds.heal_min {
            toast_writer.write(ShowToast::success(format!("Healed {} HP", event.amount)));
        }
    }

    // Gold change notifications (only for significant amounts)
    for event in gold_changed_events.read() {
        if event.amount.abs() >= thresholds.gold_change {
            if event.amount > 0 {
                toast_writer.write(ShowToast::success(format!("Gained {} gold", event.amount)));
            } else {
                toast_writer.write(ShowToast::info(format!("Spent {} gold", -event.amount)));
            }
        }
    }
}

/// Listen to item-related events
fn listen_item_events(mut events: ItemEventReaders, mut toast_writer: MessageWriter<ShowToast>) {
    // Item pickup notifications
    for event in events.picked_up.read() {
        let item_name = &event.item_id.spec().name;
        if event.quantity > 1 {
            toast_writer.write(ShowToast::success(format!(
                "Picked up {} x{}",
                item_name, event.quantity
            )));
        } else {
            toast_writer.write(ShowToast::success(format!("Picked up {}", item_name)));
        }
    }

    // Equipment notifications
    for event in events.equipped.read() {
        toast_writer.write(ShowToast::info(format!(
            "Equipped {} to {:?}",
            event.item_id.spec().name,
            event.slot
        )));
    }

    for event in events.unequipped.read() {
        toast_writer.write(ShowToast::info(format!(
            "Unequipped {} from {:?}",
            event.item_id.spec().name,
            event.slot
        )));
    }

    // Item use notifications
    for event in events.used.read() {
        toast_writer.write(ShowToast::info(format!("Used {}", event.item_id.spec().name)));
    }

    // Item drop notifications
    for event in events.dropped.read() {
        if event.quantity > 1 {
            toast_writer.write(ShowToast::warning(format!(
                "Dropped {} x{}",
                event.item_id.spec().name,
                event.quantity
            )));
        } else {
            toast_writer.write(ShowToast::warning(format!(
                "Dropped {}",
                event.item_id.spec().name
            )));
        }
    }

    // Storage notifications
    for event in events.deposited.read() {
        toast_writer.write(ShowToast::info(format!("Deposited {}", event.item_name)));
    }

    for event in events.withdrawn.read() {
        toast_writer.write(ShowToast::info(format!("Withdrew {}", event.item_name)));
    }
}

/// Listen to combat-related events
fn listen_combat_events(
    mut mob_defeated_events: MessageReader<MobDefeated>,
    mut toast_writer: MessageWriter<ShowToast>,
) {
    for event in mob_defeated_events.read() {
        toast_writer.write(ShowToast::success(format!(
            "Defeated {}!",
            event.mob_id.spec().name
        )));
    }
}

/// Listen to economy-related events
fn listen_economy_events(
    mut gold_earned_events: MessageReader<GoldEarned>,
    mut gold_spent_events: MessageReader<GoldSpent>,
    mut loot_collected_events: MessageReader<LootCollected>,
    mut transaction_completed_events: MessageReader<TransactionCompleted>,
    thresholds: Res<ToastThresholds>,
    mut toast_writer: MessageWriter<ShowToast>,
) {
    // Gold earned
    for event in gold_earned_events.read() {
        if event.amount >= thresholds.gold_earned {
            toast_writer.write(ShowToast::success(format!("Earned {} gold", event.amount)));
        }
    }

    // Gold spent
    for event in gold_spent_events.read() {
        if event.amount >= thresholds.gold_spent {
            toast_writer.write(ShowToast::info(format!("Spent {} gold", event.amount)));
        }
    }

    // Loot collected
    for event in loot_collected_events.read() {
        if event.total_items >= thresholds.loot_items {
            toast_writer.write(ShowToast::success(format!(
                "Collected {} items",
                event.total_items
            )));
        }
    }

    // Transaction completed
    for event in transaction_completed_events.read() {
        if event.price >= thresholds.transaction {
            let action = if event.is_purchase { "Purchased" } else { "Sold" };
            toast_writer.write(ShowToast::info(format!(
                "{} {} for {} gold",
                action, event.item.name, event.price
            )));
        }
    }
}

/// Listen to brewing-related events
fn listen_brewing_events(
    mut brewing_events: MessageReader<BrewingResult>,
    mut toast_writer: MessageWriter<ShowToast>,
) {
    for event in brewing_events.read() {
        match event {
            BrewingResult::Success { item_name } => {
                toast_writer.write(ShowToast::success(format!("Crafted {}!", item_name)));
            }
            BrewingResult::InsufficientIngredients { recipe_name } => {
                toast_writer.write(ShowToast::warning(format!(
                    "Missing ingredients for {}",
                    recipe_name
                )));
            }
            BrewingResult::InventoryFull { item_name } => {
                toast_writer.write(ShowToast::error(format!(
                    "Inventory full - could not add {}",
                    item_name
                )));
            }
            BrewingResult::CraftingFailed { recipe_name } => {
                toast_writer.write(ShowToast::error(format!(
                    "Failed to craft {}",
                    recipe_name
                )));
            }
        }
    }
}

fn listen_skill_events(
    mut skill_events: MessageReader<SkillLeveledUp>,
    mut toast_writer: MessageWriter<ShowToast>,
) {
    for event in skill_events.read() {
        toast_writer.write(ShowToast::success(format!(
            "{} Level Up! Now level {}",
            event.skill.display_name(),
            event.new_level
        )));
    }
}

fn listen_mining_events(
    mut events: MessageReader<MiningResult>,
    mut toast_writer: MessageWriter<ShowToast>,
) {
    for event in events.read() {
        let title = match &event.mineable_type {
            MineableEntityType::Chest => "Chest Opened!".to_string(),
            MineableEntityType::Rock { rock_type } => format!("{} Mined!", rock_type.display_name()),
        };

        if event.loot_drops.is_empty() {
            toast_writer.write(ShowToast::info(title));
        } else {
            for drop in &event.loot_drops {
                if drop.quantity > 1 {
                    toast_writer.write(ShowToast::success(
                        format!("{}: {} x{}", title, drop.item.name, drop.quantity),
                    ));
                } else {
                    toast_writer.write(ShowToast::success(
                        format!("{}: {}", title, drop.item.name),
                    ));
                }
            }
        }
    }
}

fn listen_action_combat_events(
    mut gold_events: MessageReader<GoldGained>,
    mut xp_events: MessageReader<XpGained>,
    mut loot_events: MessageReader<LootDropped>,
    mut toast_writer: MessageWriter<ShowToast>,
) {
    for event in gold_events.read() {
        toast_writer.write(ShowToast::success(format!(
            "{} defeated! +{}g",
            event.source, event.amount
        )));
    }

    for event in xp_events.read() {
        toast_writer.write(ShowToast::info(format!("+{} xp", event.amount)));
    }

    for event in loot_events.read() {
        toast_writer.write(ShowToast::success(format!("Found: {}", event.item_name)));
    }
}
