pub mod blacksmith;
pub mod crafting;
pub mod dungeon;
pub mod items;
pub mod magic;
pub mod player;
pub mod storage;
pub mod store_transactions;
pub mod toast;

// Re-export combat types from the combat module
pub use crate::combat::{
    ActiveCombatResource, AttackPerformed, CombatEnded, CombatPhaseState, CombatPlugin,
    CombatSourceResource, CombatStarted, PlayerDefeat, PlayerVictory,
};
pub use dungeon::{DungeonCompleted, DungeonPlugin, DungeonResource, RoomCleared, RoomEntered};
pub use items::{ItemDropped, ItemEquipped, ItemPickedUp, ItemPlugin, ItemUnequipped, ItemUsed};
pub use magic::MagicPlugin;
pub use crate::player::{Player, PlayerGold, PlayerName};
pub use crate::storage::Storage;
pub use player::{GoldChanged, PlayerDamaged, PlayerHealed, PlayerLeveledUp, PlayerPlugin};
pub use storage::{ItemDeposited, ItemWithdrawn, StoragePlugin};
pub use toast::{ShowToast, ToastPlugin, ToastQueue, ToastType};
pub use blacksmith::{
    calculate_upgrade_cost, BlacksmithPlugin, BlacksmithResult, ForgeRecipeEvent,
    SmeltRecipeEvent, UpgradeItemEvent, UpgradeQualityEvent,
};
pub use crafting::{BrewPotionEvent, BrewingResult, CraftingPlugin};
pub use store_transactions::{
    StorageDepositEvent, StorageWithdrawEvent, StorePurchaseEvent, StoreSellEvent,
    StoreTransactionResult, StoreTransactionsPlugin,
};
