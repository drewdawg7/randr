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

mod alchemy;
mod blacksmith;
mod combat;
mod dungeon;
mod inventory;
mod mining;
mod storage;
mod store;

use crate::inventory::EquipmentSlot;
use crate::item::recipe::RecipeId;
use crate::ui::Id;
use uuid::Uuid;


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

    // === Storage ===
    /// Deposit an item from player inventory to storage.
    DepositItem { item_uuid: Uuid },
    /// Withdraw an item from storage to player inventory.
    WithdrawItem { item_uuid: Uuid },

    // === Blacksmith ===
    /// Upgrade an item's stats at the blacksmith.
    UpgradeItem { item_uuid: Uuid },
    /// Upgrade an item's quality at the blacksmith.
    UpgradeQuality { item_uuid: Uuid },
    /// Add fuel to the blacksmith forge.
    AddFuel,
    /// Smelt a recipe at the blacksmith forge.
    SmeltRecipe { recipe_id: RecipeId },
    /// Forge an item from a recipe.
    ForgeRecipe { recipe_id: RecipeId },

    // === Alchemy ===
    /// Brew a recipe at the alchemist.
    BrewRecipe { recipe_id: RecipeId },
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

        // Storage
        GameCommand::DepositItem { item_uuid } => storage::deposit_item(item_uuid),
        GameCommand::WithdrawItem { item_uuid } => storage::withdraw_item(item_uuid),

        // Blacksmith
        GameCommand::UpgradeItem { item_uuid } => blacksmith::upgrade_item(item_uuid),
        GameCommand::UpgradeQuality { item_uuid } => blacksmith::upgrade_quality(item_uuid),
        GameCommand::AddFuel => blacksmith::add_fuel(),
        GameCommand::SmeltRecipe { recipe_id } => blacksmith::smelt_recipe(recipe_id),
        GameCommand::ForgeRecipe { recipe_id } => blacksmith::forge_recipe(recipe_id),

        // Alchemy
        GameCommand::BrewRecipe { recipe_id } => alchemy::brew_recipe(recipe_id),
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
            CommandMessage::Success(text) => gs.ui.toasts.success(text),
            CommandMessage::Info(text) => gs.ui.toasts.info(text),
            CommandMessage::Error(text) => gs.ui.toasts.error(text),
        }
    }

    // Change screen
    if let Some(screen) = result.screen_change {
        gs.ui.current_screen = screen;
    }
}
