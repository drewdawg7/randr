use bevy::prelude::*;

use crate::combat::{ActiveCombat, AttackResult, CombatPhase};
use crate::loot::LootDrop;

/// Combat state tracking for the Bevy ECS.
/// Represents different phases within the Fight screen.
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CombatState {
    /// Before combat starts - initializing combat
    #[default]
    PreCombat,
    /// Active combat in progress
    InCombat,
    /// After combat ends - processing rewards/cleanup
    PostCombat,
}

/// Bevy Resource that wraps the existing ActiveCombat struct.
/// This provides access to combat state within the Bevy ECS.
#[derive(Resource, Debug)]
pub struct ActiveCombatResource(pub Option<ActiveCombat>);

impl Default for ActiveCombatResource {
    fn default() -> Self {
        Self(None)
    }
}

impl ActiveCombatResource {
    /// Create a new combat resource with the given ActiveCombat
    pub fn new(combat: ActiveCombat) -> Self {
        Self(Some(combat))
    }

    /// Get a reference to the active combat, if any
    pub fn get(&self) -> Option<&ActiveCombat> {
        self.0.as_ref()
    }

    /// Get a mutable reference to the active combat, if any
    pub fn get_mut(&mut self) -> Option<&mut ActiveCombat> {
        self.0.as_mut()
    }

    /// Clear the active combat
    pub fn clear(&mut self) {
        self.0 = None;
    }

    /// Check if combat is active
    pub fn is_active(&self) -> bool {
        self.0.is_some()
    }
}

/// Event fired when combat starts
#[derive(Event, Debug, Clone)]
pub struct CombatStarted {
    pub enemy_name: String,
    pub enemy_health: i32,
    pub enemy_attack: i32,
    pub enemy_defense: i32,
}

/// Event fired when an attack is performed (player or enemy)
#[derive(Event, Debug, Clone)]
pub struct AttackPerformed {
    pub result: AttackResult,
    pub is_player_attacking: bool,
}

/// Event fired when combat ends (either victory or defeat)
#[derive(Event, Debug, Clone)]
pub struct CombatEnded {
    pub player_won: bool,
}

/// Event fired when the player wins combat
#[derive(Event, Debug, Clone)]
pub struct PlayerVictory {
    pub enemy_name: String,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub loot_drops: Vec<LootDrop>,
}

/// Event fired when the player is defeated
#[derive(Event, Debug, Clone)]
pub struct PlayerDefeat {
    pub enemy_name: String,
}

/// Tracks where combat was initiated from.
/// This determines where the player returns after combat ends.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CombatSourceResource {
    /// Combat from the field/town
    #[default]
    Field,
    /// Combat from dungeon (regular mob)
    Dungeon,
    /// Combat from dungeon boss
    DungeonBoss,
}

impl CombatSourceResource {
    /// Set the source to Field
    pub fn set_field(&mut self) {
        *self = Self::Field;
    }

    /// Set the source to Dungeon
    pub fn set_dungeon(&mut self) {
        *self = Self::Dungeon;
    }

    /// Set the source to DungeonBoss
    pub fn set_dungeon_boss(&mut self) {
        *self = Self::DungeonBoss;
    }

    /// Check if combat is from dungeon (regular or boss)
    pub fn is_dungeon(&self) -> bool {
        matches!(self, Self::Dungeon | Self::DungeonBoss)
    }

    /// Check if combat is from dungeon boss
    pub fn is_dungeon_boss(&self) -> bool {
        matches!(self, Self::DungeonBoss)
    }
}

/// Plugin that registers combat states, resources, and events.
/// This provides the Bevy integration layer for the combat system.
pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CombatState>()
            .init_resource::<ActiveCombatResource>()
            .init_resource::<CombatSourceResource>()
            .add_event::<CombatStarted>()
            .add_event::<AttackPerformed>()
            .add_event::<CombatEnded>()
            .add_event::<PlayerVictory>()
            .add_event::<PlayerDefeat>()
            .add_systems(Update, track_combat_phase_transitions);
    }
}

/// System that tracks combat phase changes and updates CombatState accordingly.
/// This ensures the Bevy state stays synchronized with the combat phase.
fn track_combat_phase_transitions(
    combat_res: Res<ActiveCombatResource>,
    current_state: Res<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    if let Some(combat) = combat_res.get() {
        let desired_state = match combat.phase {
            CombatPhase::PlayerTurn | CombatPhase::PlayerAttacking | CombatPhase::EnemyAttacking => {
                CombatState::InCombat
            }
            CombatPhase::Victory | CombatPhase::Defeat => CombatState::PostCombat,
        };

        // Only transition if state differs
        if **current_state != desired_state {
            next_state.set(desired_state);
        }
    } else {
        // No active combat, ensure we're in PreCombat state
        if **current_state != CombatState::PreCombat {
            next_state.set(CombatState::PreCombat);
        }
    }
}
