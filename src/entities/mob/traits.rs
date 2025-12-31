use crate::{
    combat::{Combatant, DropsGold, IsKillable, MobDeathResult, Named},
    entities::progression::GivesXP,
    loot::{HasLoot, LootTable}, stats::{HasStats, StatSheet},
};

use super::Mob;

impl Named for Mob {
    fn name(&self) -> &str {
        self.name
    }
}

impl DropsGold for Mob {
    fn drop_gold(&self) -> i32 { self.gold }
}

impl GivesXP for Mob {
    fn give_xp(&self) -> i32 {
        self.dropped_xp
    }
}

impl Combatant for Mob {
    fn effective_attack(&self) -> i32 {
        self.get_attack()
    }

    fn effective_health(&self) -> i32 {
        self.get_health()
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

impl HasStats for Mob {
    fn stats(&self) -> &StatSheet {
        &self.stats
    }

    fn stats_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}

impl IsKillable for Mob {
    type DeathResult = MobDeathResult;

    fn on_death(&mut self) -> MobDeathResult {
        MobDeathResult {
            gold_dropped: self.drop_gold(),
            xp_dropped: self.give_xp(),
            loot_drops: self.loot().roll_drops(),
        }
    }
}
