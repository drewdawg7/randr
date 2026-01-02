use crate::{
    combat::IsKillable,
    entities::Player,
    game_state,
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
            .flat_map(|(item_kind, quantity)| {
                (0..*quantity).map(move |_| game_state().spawn_item(*item_kind))
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
