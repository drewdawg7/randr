pub mod blacksmith;
pub mod crafting;
pub mod crafting_complete;
pub mod items;
pub mod merchant;
pub mod mining;
pub mod npc_interactions;
pub mod player;
pub mod storage;
pub mod store_transactions;
pub mod toast;

// Re-export combat types from the combat module
pub use crate::combat::CombatPlugin;
pub use items::{ItemDropped, ItemEquipped, ItemPickedUp, ItemPlugin, ItemUnequipped, ItemUsed};
pub use crate::player::{PlayerGold, PlayerName};
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
pub use merchant::{
    BuyItemEvent, MerchantPlugin, MerchantTransactionResult, SellItemEvent,
};
pub use crafting_complete::{
    AnvilCraftingCompleteEvent, CraftingCompletePlugin, ForgeCraftingCompleteEvent,
};
pub use mining::MiningPlugin;
pub use npc_interactions::NpcInteractionsPlugin;
