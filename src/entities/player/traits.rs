use std::{collections::HashMap, fmt::Display};

use crate::{combat::{Combatant, HasGold, Named}, entities::{progression::HasProgression, Player, Progression}, inventory::{EquipmentSlot, HasInventory, Inventory}, stats::{HasStats, StatInstance, StatSheet, StatType}};


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
                sheet.insert(StatType::Attack.instance(12));
                sheet.insert(StatType::Defense.instance(3));
                sheet.insert(StatType::Health.instance(100));
                sheet
            },
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}/{})", self.name, self.get_health(), self.get_max_health())
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



impl Combatant for Player {
    fn effective_attack(&self) -> i32 {
        let weapon = self.get_equipped_item(EquipmentSlot::Weapon);
        let weapon_attack = match weapon {
            Some(w) => w.attack(),
            None    => 0
        };
        self.get_attack() + weapon_attack
    }
    fn increase_health(&mut self, amount: i32) {
        self.increase_health(amount);
    }
    fn decrease_health(&mut self, amount: i32) {
        self.decrease_health(amount);
    }
    fn effective_defense(&self) -> i32 {
        let offhand = self.get_equipped_item(EquipmentSlot::OffHand);
        let offhand_defense = match offhand {
            Some(off) => off.def(),
            None     => 0
        };
        self.get_defense() + offhand_defense
    }
} 

impl HasProgression for Player {
    fn progression(&self) -> &Progression { &self.prog }
    fn progression_mut(&mut self) -> &mut Progression {
        &mut self.prog
    }
    fn on_level_up(&mut self) {
        if self.level() % 10 == 0 {
            self.increase_defense(1);
        }
        self.increase_health(5);
        self.increase_max_health(5);
        self.increase_attack(1);
    }
}
