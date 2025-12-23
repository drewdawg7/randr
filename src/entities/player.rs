
use std::fmt::Display;

use crate::{combat::{Combatant, HasGold, Named}, entities::{progression::HasProgression, Progression}, inventory::{EquipmentSlot, HasInventory, Inventory}, stats::{HasStats, StatSheet, StatType}, utilities::{text_bar_with_label}};

#[derive(Debug, Clone)]
pub struct Player {
    pub name: &'static str,
    pub gold: i32,
    pub prog: Progression,
    pub inventory: Inventory,
    pub stats: StatSheet
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


impl Player {
   
    pub fn get_attack(&self) -> i32 {
        self.attack()
    }

    pub fn increase_attack(&mut self, amount: i32) {
        self.inc_attack(amount);
    }

    pub fn get_health(&self) -> i32 {
        self.hp()
    }

    pub fn get_max_health(&self) -> i32 {
        self.max_hp()
    }

    pub fn increase_health(&mut self, amount: i32) {
        self.inc_hp(amount);
    }

    pub fn decrease_health(&mut self, amount: i32) {
        self.dec_hp(amount);
    }

    pub fn pretty_print(&self) -> String {


        let hp = text_bar_with_label("HP", self.get_health(), self.get_max_health(), 10);
        let gold = format!("{} gold", self.gold);
        let attack = format!("Attack: {} ({})", self.attack_power(), self.get_attack());
        let xp = self.progression().pretty_print();
        let first_row = format!("{} | {} | {}", self.name, self.progression().level, gold);

        let s: String = format!(
            "\n{}\n{}\n{}\n{}\n",
            first_row,
            hp,
            xp,
            attack
        );
        s
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
    fn attack_power(&self) -> i32 {
        let weapon = self.get_equipped_item(EquipmentSlot::Weapon);
        let weapon_attack = match weapon {
            Some(w) => w.attack,
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


} 

impl HasProgression for Player {
    fn progression(&self) -> &Progression { &self.prog }
    fn progression_mut(&mut self) -> &mut Progression {
        &mut self.prog
    }
    fn on_level_up(&mut self) {
        self.increase_health(5);
        self.increase_attack(1);
    }
}
