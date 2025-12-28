use rand::Rng;

use crate::{
    combat::{Combatant, DropsGold, Named},
    entities::progression::GivesXP,
    loot::{HasLoot, LootTable},
};

use super::Mob;

impl Named for Mob {
    fn name(&self) -> &str {
        self.name
    }
}

impl DropsGold for Mob {
    fn drop_gold(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=5)
    }
}

impl GivesXP for Mob {
    fn give_xp(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(15..=20)
    }
}

impl Combatant for Mob {
    fn effective_attack(&self) -> i32 {
        self.get_attack()
    }

    fn effective_health(&self) -> i32 {
        self.get_health()
    }

    fn increase_health(&mut self, amount: i32) {
        self.increase_health(amount);
    }
    fn decrease_health(&mut self, amount: i32) {
        self.decrease_health(amount);
    }
}

impl HasLoot for Mob {
    fn loot(&self) -> &LootTable {
        &self.loot_table
    }

    fn loot_mut(&mut self) -> &mut LootTable {
        &mut self.loot_table
    }
}
