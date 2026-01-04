//! Combat-related trait implementations for Player

use crate::{
    combat::{Combatant, DealsDamage, HasGold, IsKillable, Named, PlayerDeathResult},
    inventory::HasEquipment,
    stats::{HasStats, StatType},
};

use super::Player;

impl Named for Player {
    fn name(&self) -> &str {
        self.name
    }
}

impl HasGold for Player {
    fn gold(&self) -> i32 {
        self.gold
    }
    fn gold_mut(&mut self) -> &mut i32 {
        &mut self.gold
    }
}

impl IsKillable for Player {
    type DeathResult = PlayerDeathResult;

    fn on_death(&mut self, _magic_find: i32) -> PlayerDeathResult {
        let gold_lost = ((self.gold() as f64) * 0.05).round() as i32;
        self.dec_gold(gold_lost);
        // Restore health to full
        self.inc(StatType::Health, self.max_hp());
        PlayerDeathResult { gold_lost }
    }
}

impl DealsDamage for Player {
    fn equipment_attack_bonus(&self) -> i32 {
        self.equipment_attack() + self.tome_attack_bonus()
    }
}

impl Combatant for Player {
    fn effective_defense(&self) -> i32 {
        self.defense() + self.equipment_defense() + self.tome_defense_bonus()
    }
}
