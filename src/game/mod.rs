pub mod blacksmith;
pub mod crafting;
pub mod items;
pub mod player;
pub mod storage;
pub mod store_transactions;
pub mod toast;

// Re-export combat types from the combat module
pub use crate::combat::CombatPlugin;
pub use items::{ItemDropped, ItemEquipped, ItemPickedUp, ItemPlugin, ItemUnequipped, ItemUsed};
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
    StorageDepositEvent, StorageTransactionResult, StorageTransactionsPlugin, StorageWithdrawEvent,
};
