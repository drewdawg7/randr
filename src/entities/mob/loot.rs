use crate::{item::ItemKind, loot::{LootItem, LootTable}};

use super::MobKind;

/// Returns the loot table for a given mob kind.
pub fn loot_table_for(kind: MobKind) -> LootTable {
    match kind {
        MobKind::Slime => LootTable::default(),
        MobKind::Goblin => {
            let mut table = LootTable::default();
            // 1 in 10 chance (10%) to drop a dagger
            if let Ok(item) = LootItem::new(ItemKind::Dagger, 1, 10) {
                let _ = table.add_loot_item(item);
            }
            table
        }
    }
}
