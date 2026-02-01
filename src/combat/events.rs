use bevy::prelude::*;

use crate::loot::LootDrop;
use crate::mob::MobId;

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
pub struct VictoryAchieved {
    pub mob_id: MobId,
    pub mob_name: String,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub loot_drops: Vec<LootDrop>,
}
