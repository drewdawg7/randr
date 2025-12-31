use crate::combat::Combatant;
use crate::entities::progression::HasProgression;
use crate::utilities::text_bar_with_label;
use crate::{entities::Progression, inventory::Inventory, stats::StatSheet};
use crate::stats::HasStats;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: &'static str,
    pub gold: i32,
    pub prog: Progression,
    pub inventory: Inventory,
    pub stats: StatSheet
}


impl Player {
   
    pub fn get_attack(&self) -> i32 {
        self.attack()
    }
    pub fn get_defense(&self) -> i32 {
        self.def()
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
    pub fn increase_defense(&mut self, amount: i32) {
        self.inc_def(amount);
    }
    pub fn increase_health(&mut self, amount: i32) {
        self.inc_hp(amount);
    }

    pub fn decrease_health(&mut self, amount: i32) {
        self.dec_hp(amount);
    }
    pub fn increase_max_health(&mut self, amount: i32) {
        self.inc_max_hp(amount);
    }
    pub fn pretty_print(&self) -> String {


        let hp = text_bar_with_label("HP", self.get_health(), self.get_max_health(), 10);
        let gold = format!("{} gold", self.gold);
        let attack = format!("Attack: {} ({})", self.effective_attack(), self.get_attack());
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
