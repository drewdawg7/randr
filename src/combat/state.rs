use crate::{
    combat::{AttackResult, CombatRounds},
    mob::Mob,
    loot::LootDrop,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatPhase {
    PlayerTurn,       // Waiting for Attack/Run input
    PlayerAttacking,  // Brief pause showing player attack result
    EnemyAttacking,   // Brief pause showing enemy attack result
    Victory,          // Combat ended, player won
    Defeat,           // Combat ended, player lost
}

pub struct ActiveCombat {
    pub mob: Mob,
    pub phase: CombatPhase,
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
            phase: CombatPhase::PlayerTurn,
            rounds: CombatRounds::new(),
            last_player_attack: None,
            last_enemy_attack: None,
            gold_gained: 0,
            xp_gained: 0,
            loot_drops: Vec::new(),
        }
    }

    pub fn is_combat_over(&self) -> bool {
        matches!(self.phase, CombatPhase::Victory | CombatPhase::Defeat)
    }
}
