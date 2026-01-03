use std::{collections::HashMap, fmt::Display};

use crate::{
    combat::{Combatant, DealsDamage, HasGold, IsKillable, Named, PlayerDeathResult},
    entities::{progression::HasProgression, Player, Progression},
    inventory::{HasEquipment, HasInventory, Inventory},
    stats::{HasStats, StatInstance, StatSheet, StatType},
};


impl Default for Player {
    fn default() -> Self {
        Self {
            gold: 0,
            name: "Drew",
            prog: Progression::new(),
            inventory: Inventory::new(),
            stats: {
                let stats: HashMap<StatType, StatInstance> = HashMap::new();
                let mut sheet = StatSheet { stats };
                sheet.insert(StatType::Attack.instance(8));
                sheet.insert(StatType::Defense.instance(3));
                sheet.insert(StatType::GoldFind.instance(0));
                sheet.insert(StatType::Mining.instance(100));
                sheet.insert(StatType::Health.instance(100));
                sheet
            },
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}/{})", self.name, self.hp(), self.max_hp())
    }
}

impl HasStats for Player {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
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

impl HasInventory for Player {
    fn inventory(&self) -> &Inventory {
        &self.inventory
    }

    fn inventory_mut(&mut self) -> &mut Inventory {
        &mut self.inventory
    }
}

impl HasEquipment for Player {}

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

impl HasProgression for Player {
    fn progression(&self) -> &Progression { &self.prog }
    fn progression_mut(&mut self) -> &mut Progression {
        &mut self.prog
    }
    fn on_level_up(&mut self) {
        if self.level() % 10 == 0 {
            self.inc(StatType::Defense, 1);
        }
        self.inc(StatType::Health, 5);
        self.inc_max(StatType::Health, 5);
        self.inc(StatType::Attack, 1);
    }
}

