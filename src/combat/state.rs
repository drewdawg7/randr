use crate::{
    combat::{AttackResult, CombatEntityInfo, CombatRounds, get_combat_entity_info},
    mob::Mob,
    loot::LootDrop,
};

#[derive(Debug)]
pub struct ActiveCombat {
    pub mob: Mob,
    pub rounds: CombatRounds,
    pub last_player_attack: Option<AttackResult>,
    pub last_enemy_attack: Option<AttackResult>,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub loot_drops: Vec<LootDrop>,
}

impl ActiveCombat {
    pub fn new(mob: Mob) -> Self {
        Self {
            mob,
            rounds: CombatRounds::new(),
            last_player_attack: None,
            last_enemy_attack: None,
            gold_gained: 0,
            xp_gained: 0,
            loot_drops: Vec::new(),
        }
    }

    /// Get summary info about the enemy using the CombatEntity trait.
    pub fn enemy_info(&self) -> CombatEntityInfo {
        get_combat_entity_info(&self.mob)
    }
}
