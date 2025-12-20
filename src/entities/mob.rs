
use crate::{combat::{Combatant, DropsGold, Named}, entities::progression::GivesXP, registry::{Registry, RegistryDefaults, SpawnFromSpec}};
use rand::Rng;

pub type MobSpecId = usize;

#[derive(Debug, Clone)]
pub struct Mob {
    pub spec: MobKind,
    pub name: &'static str,
    pub health: i32,
    pub attack: i32,
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



pub type MobRegistry = Registry<MobKind, MobSpec>; 

impl SpawnFromSpec<MobKind> for MobSpec {
    type Output = Mob;
    fn spawn_from_spec(kind: MobKind, spec: &Self) -> Self::Output {
        Mob {
            spec: kind,
            name: spec.name,
            health: spec.max_health,
            attack: spec.attack
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
                    max_health: 15,
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
        self.attack
    }

    fn health(&self) -> i32 {
        self.health
    }

    fn health_mut(&mut self) -> &mut i32 {
        &mut self.health
    }

}



