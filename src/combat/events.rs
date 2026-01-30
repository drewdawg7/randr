use bevy::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct PlayerAttackMob {
    pub target: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct DealDamage {
    pub target: Entity,
    pub amount: i32,
    pub source_name: String,
}

#[derive(Event, Debug, Clone)]
pub struct EntityDied {
    pub entity: Entity,
    pub is_player: bool,
}
