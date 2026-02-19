use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct PlayerAttackMob {
    pub target: Entity,
}

#[derive(Message, Debug, Clone)]
pub struct DealDamage {
    pub target: Entity,
    pub amount: i32,
    pub source_name: String,
}

#[derive(Message, Debug, Clone)]
pub struct EntityDied {
    pub entity: Entity,
    pub is_player: bool,
}

#[derive(Message, Debug, Clone)]
pub struct DamageEntity {
    pub target: Entity,
    pub amount: i32,
}

#[derive(Message, Debug, Clone)]
pub struct GoldGained {
    pub amount: i32,
    pub source: String,
}

#[derive(Message, Debug, Clone)]
pub struct XpGained {
    pub amount: i32,
    pub source: String,
}

#[derive(Message, Debug, Clone)]
pub struct LootDropped {
    pub item_name: String,
}
