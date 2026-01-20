use bevy::prelude::*;

use crate::loot::LootDrop;
use crate::states::AppState;

use super::orchestration::{
    cleanup_combat, execute_player_attack, handle_continue_action, handle_fight_again,
    handle_run_action, initialize_combat, CombatLogState, PlayerCombatAction, PostCombatAction,
};
use super::{ActiveCombat, AttackResult};

#[derive(SubStates, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[source(AppState = AppState::Fight)]
pub enum CombatPhaseState {
    #[default]
    Initializing,
    PlayerTurn,
    Victory,
    Defeat,
}

#[derive(Resource, Debug, Default)]
pub struct ActiveCombatResource(pub Option<ActiveCombat>);

impl ActiveCombatResource {
    pub fn new(combat: ActiveCombat) -> Self {
        Self(Some(combat))
    }

    pub fn get(&self) -> Option<&ActiveCombat> {
        self.0.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut ActiveCombat> {
        self.0.as_mut()
    }

    pub fn clear(&mut self) {
        self.0 = None;
    }

    pub fn is_active(&self) -> bool {
        self.0.is_some()
    }
}

#[derive(Event, Debug, Clone)]
pub struct CombatStarted {
    pub enemy_name: String,
    pub enemy_health: i32,
    pub enemy_attack: i32,
    pub enemy_defense: i32,
}

#[derive(Event, Debug, Clone)]
pub struct AttackPerformed {
    pub result: AttackResult,
    pub is_player_attacking: bool,
}

#[derive(Event, Debug, Clone)]
pub struct CombatEnded {
    pub player_won: bool,
}

#[derive(Event, Debug, Clone)]
pub struct PlayerVictory {
    pub enemy_name: String,
    pub gold_gained: i32,
    pub xp_gained: i32,
    pub loot_drops: Vec<LootDrop>,
}

#[derive(Event, Debug, Clone)]
pub struct PlayerDefeat {
    pub enemy_name: String,
}

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CombatSourceResource {
    #[default]
    Field,
}

impl CombatSourceResource {
    pub fn set_field(&mut self) {
        *self = Self::Field;
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<CombatPhaseState>()
            .init_resource::<ActiveCombatResource>()
            .init_resource::<CombatSourceResource>()
            .init_resource::<CombatLogState>()
            .add_event::<CombatStarted>()
            .add_event::<AttackPerformed>()
            .add_event::<CombatEnded>()
            .add_event::<PlayerVictory>()
            .add_event::<PlayerDefeat>()
            .add_event::<PlayerCombatAction>()
            .add_event::<PostCombatAction>()
            .add_systems(OnEnter(CombatPhaseState::Initializing), initialize_combat)
            .add_systems(
                Update,
                (execute_player_attack, handle_run_action)
                    .run_if(in_state(CombatPhaseState::PlayerTurn)),
            )
            .add_systems(
                Update,
                (handle_fight_again, handle_continue_action)
                    .run_if(in_state(CombatPhaseState::Victory).or(in_state(CombatPhaseState::Defeat))),
            )
            .add_systems(OnExit(AppState::Fight), cleanup_combat);
    }
}
