//! Game command system.
//!
//! This module provides a command layer that decouples game logic from UI.
//! Instead of UI components directly mutating game state, they dispatch commands
//! which are executed by handlers that encapsulate the game logic.
//!
//! # Usage
//!
//! ```rust,ignore
//! // In a UI component's event handler:
//! match selection {
//!     FightSelection::Attack => {
//!         let result = execute(GameCommand::PlayerAttack);
//!         // Handle result (show toast, change screen, etc.)
//!     }
//! }
//! ```

mod combat;
mod dungeon;
mod inventory;
mod mining;
mod store;

use crate::inventory::EquipmentSlot;
use crate::ui::Id;
use uuid::Uuid;

pub use combat::*;
pub use dungeon::*;
pub use inventory::*;
pub use mining::*;
pub use store::*;

/// Result of executing a game command.
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Whether the command succeeded.
    pub success: bool,
    /// Optional message to display (toast).
    pub message: Option<CommandMessage>,
    /// Optional screen to navigate to.
    pub screen_change: Option<Id>,
}

impl CommandResult {
    /// Create a successful result with no message.
    pub fn ok() -> Self {
        Self {
            success: true,
            message: None,
            screen_change: None,
        }
    }

    /// Create a successful result with a success message.
    pub fn success(msg: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(CommandMessage::Success(msg.into())),
            screen_change: None,
        }
    }

    /// Create a successful result with an info message.
    pub fn info(msg: impl Into<String>) -> Self {
        Self {
            success: true,
            message: Some(CommandMessage::Info(msg.into())),
            screen_change: None,
        }
    }

    /// Create a failed result with an error message.
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            message: Some(CommandMessage::Error(msg.into())),
            screen_change: None,
        }
    }

    /// Add a screen change to the result.
    pub fn with_screen(mut self, screen: Id) -> Self {
        self.screen_change = Some(screen);
        self
    }
}

/// Type of message to display after command execution.
#[derive(Debug, Clone)]
pub enum CommandMessage {
    Success(String),
    Info(String),
    Error(String),
}

/// All game commands that can be executed.
///
/// Commands represent player actions that modify game state.
/// UI components dispatch these instead of directly mutating state.
#[derive(Debug, Clone)]
pub enum GameCommand {
    // === Combat ===
    /// Player attacks the current enemy.
    PlayerAttack,
    /// Player runs from combat.
    PlayerRun,
    /// Return from combat to previous screen.
    ReturnFromCombat,
    /// Start a new fight (from field).
    StartNewFight,

    // === Mining ===
    /// Mine the current rock.
    MineRock,

    // === Store ===
    /// Purchase an item from the store.
    PurchaseItem { store_idx: usize },
    /// Sell an item from player inventory.
    SellItem { item_uuid: Uuid },

    // === Dungeon ===
    /// Enter/interact with the current room.
    EnterRoom,
    /// Move in a direction.
    MoveDungeon { direction: crate::dungeon::Direction },
    /// Leave the dungeon.
    LeaveDungeon,
    /// Rest at a rest room (heal).
    Rest,
    /// Attack the boss.
    AttackBoss,

    // === Inventory ===
    /// Equip an item.
    EquipItem {
        item_uuid: Uuid,
        slot: EquipmentSlot,
    },
    /// Unequip an item from a slot.
    UnequipItem { slot: EquipmentSlot },
    /// Toggle item lock status.
    ToggleLock { item_uuid: Uuid },
    /// Use a consumable item.
    UseConsumable { item_uuid: Uuid },
}

/// Execute a game command and return the result.
///
/// This is the main entry point for the command system.
/// UI components call this to dispatch actions to the game logic.
pub fn execute(cmd: GameCommand) -> CommandResult {
    match cmd {
        // Combat
        GameCommand::PlayerAttack => combat::player_attack(),
        GameCommand::PlayerRun => combat::player_run(),
        GameCommand::ReturnFromCombat => combat::return_from_combat(),
        GameCommand::StartNewFight => combat::start_new_fight(),

        // Mining
        GameCommand::MineRock => mining::mine_rock(),

        // Store
        GameCommand::PurchaseItem { store_idx } => store::purchase_item(store_idx),
        GameCommand::SellItem { item_uuid } => store::sell_item(item_uuid),

        // Dungeon
        GameCommand::EnterRoom => dungeon::enter_room(),
        GameCommand::MoveDungeon { direction } => dungeon::move_dungeon(direction),
        GameCommand::LeaveDungeon => dungeon::leave_dungeon(),
        GameCommand::Rest => dungeon::rest(),
        GameCommand::AttackBoss => dungeon::attack_boss(),

        // Inventory
        GameCommand::EquipItem { item_uuid, slot } => inventory::equip_item(item_uuid, slot),
        GameCommand::UnequipItem { slot } => inventory::unequip_item(slot),
        GameCommand::ToggleLock { item_uuid } => inventory::toggle_lock(item_uuid),
        GameCommand::UseConsumable { item_uuid } => inventory::consume_item(item_uuid),
    }
}

/// Apply command result to game state (show toast, change screen).
///
/// Call this after executing a command to apply side effects.
pub fn apply_result(result: &CommandResult) {
    let gs = crate::system::game_state();

    // Show toast message
    if let Some(msg) = &result.message {
        match msg {
            CommandMessage::Success(text) => gs.toasts.success(text),
            CommandMessage::Info(text) => gs.toasts.info(text),
            CommandMessage::Error(text) => gs.toasts.error(text),
        }
    }

    // Change screen
    if let Some(screen) = result.screen_change {
        gs.current_screen = screen;
    }
}
