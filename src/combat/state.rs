use crate::{
    combat::{CombatEntityInfo, CombatRounds, get_combat_entity_info},
    mob::Mob,
    loot::LootDrop,
};

#[derive(Debug)]
pub struct ActiveCombat {
    pub mob: Mob,
    pub rounds: CombatRounds,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub loot_drops: Vec<LootDrop>,
}

impl ActiveCombat {
    pub fn new(mob: Mob) -> Self {
        Self {
            mob,
            rounds: CombatRounds::new(),
            gold_gained: 0,
            xp_gained: 0,
            loot_drops: Vec::new(),
        }
    }

    /// Get summary info about the enemy using the CombatEntity trait.
    pub fn enemy_info(&self) -> CombatEntityInfo {
        get_combat_entity_info(&self.mob)
    }
}
