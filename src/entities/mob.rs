use crate::combat::{Combatant, Named, DropsGold};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Mob {
    pub name: String,
    pub health: i32,
    pub attack: i32,
}

impl Mob {
    pub fn new(name: &str) -> Mob {
        Mob {
            name: name.into(),
            health: 22,
            attack: 4

        }
    }
    pub fn spawn_mobs(amount: i32) -> Vec<Mob> {
        (0..amount)
            .map(|_| Mob::new("Slime"))
            .collect()
    }
}

impl Named for Mob {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl DropsGold for Mob {
    fn drop_gold(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=5)
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



