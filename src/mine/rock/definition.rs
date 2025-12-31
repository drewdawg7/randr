use std::collections::HashMap;

use crate::{
    combat::IsKillable,
    entities::Player,
    game_state,
    item::Item,
    loot::{LootItem, LootTable},
    stats::{StatSheet, StatType},
    HasInventory,
    ItemKind,
};

use super::enums::RockId;

pub struct Rock {
    pub rock_id: RockId,
    pub stats: StatSheet,
    pub loot: LootTable,
}

impl Rock {
    pub fn copper_rock() -> Self {
        let mut table = LootTable::default();
        if let Ok(item) = LootItem::new(ItemKind::CopperOre, 1, 1) {
            let _ = table.add_loot_item(item);
        }
        let mut stats = HashMap::new();
        stats.insert(StatType::Health, StatType::instance(StatType::Health, 50));

        Self {
            rock_id: RockId::Copper,
            stats: StatSheet { stats },
            loot: table,
        }
    }

    pub fn coal_rock() -> Self {
        let mut table = LootTable::default();
        if let Ok(item) = LootItem::new(ItemKind::Coal, 1, 1) {
            let _ = table.add_loot_item(item);
        }
        let mut stats = HashMap::new();
        stats.insert(StatType::Health, StatType::instance(StatType::Health, 50));
        Self {
            rock_id: RockId::Coal,
            stats: StatSheet { stats },
            loot: table,
        }
    }

    pub fn tin_rock() -> Self {
        let mut table = LootTable::default();
        if let Ok(item) = LootItem::new(ItemKind::TinOre, 1, 1) {
            let _ = table.add_loot_item(item);
        }
        let mut stats = HashMap::new();
        stats.insert(StatType::Health, StatType::instance(StatType::Health, 50));
        Self {
            rock_id: RockId::Tin,
            stats: StatSheet { stats },
            loot: table,
        }
    }

    pub fn roll_drops(&self) -> Vec<Item> {
        let drops = self.loot.roll_drops();
        drops
            .iter()
            .map(|item_kind| game_state().spawn_item(*item_kind))
            .collect()
    }

    pub fn mine(&mut self, player: &mut Player) {
        let damage = player.get_effective_mining();
        self.take_damage(damage);
        if !self.is_alive() {
            let result = self.on_death();
            for drop in result.drops {
                let _ = player.add_to_inv(drop);
            }
        }
    }
}
