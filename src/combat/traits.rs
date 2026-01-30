use crate::combat::Attack;
use crate::entities::progression::GivesXP;
use crate::loot::HasLoot;
use crate::stats::{HasStats, StatType};

pub trait IsKillable: HasStats {
    type DeathResult;

    fn health(&self) -> i32 {
        self.hp()
    }

    fn take_damage(&mut self, amount: i32) {
        self.dec(StatType::Health, amount);
    }

    fn is_alive(&self) -> bool {
        self.health() > 0
    }

    fn on_death(&mut self, magic_find: i32) -> Self::DeathResult;
}

pub trait DealsDamage: HasStats {
    const ATTACK_VARIANCE: f64 = 0.25;

    fn equipment_attack_bonus(&self) -> i32 {
        0
    }

    fn get_attack(&self) -> Attack {
        let total = self.attack() + self.equipment_attack_bonus();
        let variance = (total as f64 * Self::ATTACK_VARIANCE).round() as i32;
        Attack::new((total - variance).max(1), total + variance)
    }

    fn effective_attack(&self) -> i32 {
        self.get_attack().average()
    }
}

pub trait Combatant: Named + IsKillable + DealsDamage {
    fn effective_defense(&self) -> i32 {
        self.defense()
    }

    fn effective_health(&self) -> i32 {
        self.health()
    }
}

pub trait Named {
    fn name(&self) -> &str;
}

pub trait DropsGold {
    fn drop_gold(&self) -> i32;
}

pub trait HasGold {
    fn gold(&self) -> i32;
    fn gold_mut(&mut self) -> &mut i32;

    fn add_gold(&mut self, amount: i32) {
        *self.gold_mut() += amount;
    }

    fn dec_gold(&mut self, amount: i32) {
        *self.gold_mut() = (self.gold() - amount).max(0);
    }
}

pub trait CombatEntity: Combatant + DropsGold + GivesXP + HasLoot {}

impl<T> CombatEntity for T where T: Combatant + DropsGold + GivesXP + HasLoot {}
