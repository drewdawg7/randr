use crate::{
    combat::IsKillable,
    entities::Player,
    item::Item,
    loot::LootTable,
    stats::StatSheet,
    HasInventory,
};

use super::enums::RockId;

#[derive(Clone)]
pub struct Rock {
    pub rock_id: RockId,
    pub stats: StatSheet,
    pub loot: LootTable,
}

impl Rock {
    pub fn roll_drops(&self) -> Vec<Item> {
        let drops = self.loot.roll_drops();
        drops
            .iter()
            .flat_map(|loot_drop| {
                (0..loot_drop.quantity).map(move |_| loot_drop.item.clone())
            })
            .collect()
    }

    /// Mine this rock. Returns drops if rock was destroyed, None otherwise.
    pub fn mine(&mut self, player: &mut Player) -> Option<Vec<Item>> {
        let damage = player.effective_mining();
        self.take_damage(damage);
        if !self.is_alive() {
            let result = self.on_death();
            for drop in &result.drops {
                let _ = player.add_to_inv(drop.clone());
            }
            Some(result.drops)
        } else {
            None
        }
    }
}
