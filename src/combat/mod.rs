mod attack;
pub mod orchestration;
pub mod plugin;
mod result;
mod state;
mod system;
mod traits;
mod tests;

pub(crate) use attack::*;
pub use orchestration::{CombatLogState, PlayerCombatAction, PostCombatAction};
pub use plugin::{
    ActiveCombatResource, AttackPerformed, CombatEnded, CombatPhaseState, CombatPlugin,
    CombatSourceResource, CombatStarted, PlayerDefeat, PlayerVictory,
};
pub(crate) use result::*;
pub(crate) use state::*;
pub(crate) use system::*;
pub(crate) use traits::*;
