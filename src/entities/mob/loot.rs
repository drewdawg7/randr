
use crate::{item::ItemId, loot::{LootItem, LootTable}};

use super::MobKind;

/// Returns the loot table for a given mob kind.
pub fn loot_table_for(kind: MobKind) -> LootTable {
    match kind {
        MobKind::Slime => {

            let mut table = LootTable::default();
            if let Ok(item) = LootItem::new(ItemId::Dagger, 1, 10, 1..=1) {
                let _ = table.add_loot_item(item);
            }
            if let Ok(item) = LootItem::new(ItemId::GoldRing, 1, 100, 1..=1) {
                let _ = table.add_loot_item(item);
            }
            table
        }
        MobKind::Goblin => {
            let mut table = LootTable::default();
            if let Ok(item) = LootItem::new(ItemId::Sword, 1, 15, 1..=1) {
                let _ = table.add_loot_item(item);
            }
            if let Ok(item) = LootItem::new(ItemId::BasicShield, 1, 15, 1..=1) {
                let _ = table.add_loot_item(item);
            }
            if let Ok(item) = LootItem::new(ItemId::GoldRing, 1, 100, 1..=1) {
                let _ = table.add_loot_item(item);
            }
            table
        }
        MobKind::Dragon => {
            let mut table = LootTable::default();
            if let Ok(item) = LootItem::new(ItemId::GoldRing, 1, 100, 1..=1) {
                let _ = table.add_loot_item(item);
            }
            if let Ok(item) = LootItem::new(ItemId::QualityUpgradeStone, 1, 1, 1..=1) {
                let _ = table.add_loot_item(item);
            }
            table
        }
    }
}
