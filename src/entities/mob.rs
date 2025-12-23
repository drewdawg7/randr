
use std::collections::HashMap;

use crate::{combat::{Combatant, DropsGold, Named}, entities::progression::GivesXP, registry::{Registry, RegistryDefaults, SpawnFromSpec}, stats::{HasStats, StatInstance, StatSheet, StatType}};
use rand::Rng;

pub type MobSpecId = usize;

#[derive(Debug, Clone)]
pub struct Mob {
    pub spec: MobKind,
    pub name: &'static str,
    pub stats: StatSheet,
}


impl Mob {

    pub fn get_health(&self) -> i32 {
        self.get_stat_sheet().get_stat_value(StatType::Health)
    }

    pub fn get_max_health(&self) -> i32 {
        self.get_stat_sheet().get_max_stat_value(StatType::Health)
    }

    pub fn increase_health(&mut self, amount: i32) {
        self.get_stat_sheet_mut().increase_stat(StatType::Health, amount);
    }

    pub fn decrease_health(&mut self, amount: i32) {
        self.get_stat_sheet_mut().decrease_stat(StatType::Health, amount);
    }
}

pub struct MobSpec {
    pub name: &'static str,
    pub max_health: i32,
    pub attack: i32,
}


#[derive(Debug, Copy, Clone,Eq, PartialEq, Hash)]
pub enum MobKind {
    Slime,
    Goblin
}

impl HasStats for Mob {

    fn get_stat_sheet(&self) -> &StatSheet {
        &self.stats
    }

    fn get_stat_sheet_mut(&mut self) -> &mut StatSheet {
        &mut self.stats
    }
}

pub type MobRegistry = Registry<MobKind, MobSpec>; 

impl SpawnFromSpec<MobKind> for MobSpec {
    type Output = Mob;
    fn spawn_from_spec(kind: MobKind, spec: &Self) -> Self::Output {
        Mob {
            spec: kind,
            name: spec.name,
            stats: {
                let mut stats: HashMap<StatType, StatInstance> = HashMap::new();
                stats.insert(
                    StatType::Attack, 
                    StatInstance {
                        stat_type: StatType::Attack,
                        current_value: spec.attack,
                        max_value: spec.attack 
                    }

                );

                stats.insert(
                    StatType::Health, 
                    StatInstance {
                        stat_type: StatType::Health,
                        current_value: spec.max_health,
                        max_value: spec.max_health
                    }

                );
                StatSheet { stats }
            }
        }
    }
}

impl RegistryDefaults<MobKind> for MobSpec {
    fn defaults() -> impl IntoIterator<Item = (MobKind, MobSpec)> {
        [
            (
                MobKind::Slime,
                MobSpec {
                    name: "Slime",
                    max_health: 10,
                    attack: 2

                }
            ),
            (
                MobKind::Goblin,
                MobSpec {
                    name: "Goblin",
                    max_health: 45,
                    attack: 4
                }
            )
        ]
    }
}



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
    fn attack_power(&self) -> i32 {
        match self.get_stat_sheet().get_stat(StatType::Attack) {
            Some(stat) => stat.current_value,
            None       => 0,
        }
    }
    
    fn health(&self) -> i32 {
        self.get_health()
    }



    fn increase_health(&mut self, amount: i32) {
        self.increase_health(amount);
    }  
    fn decrease_health(&mut self, amount: i32) {
        self.decrease_health(amount);
    }

}



