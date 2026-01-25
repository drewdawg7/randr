use bevy::prelude::*;
use rand::Rng;

use crate::item::ItemId;
use crate::location::store::StoreItem;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;

/// Component marker for the merchant modal UI.
#[derive(Component)]
pub struct MerchantModalRoot;

/// Marker for the merchant stock grid (left side).
#[derive(Component)]
pub struct MerchantStockGrid;

/// Marker for the player inventory grid (right side).
#[derive(Component)]
pub struct MerchantPlayerGrid;

/// Resource holding the merchant's current stock.
#[derive(Resource)]
pub struct MerchantStock {
    pub items: Vec<StoreItem>,
}

impl MerchantStock {
    /// Generate random merchant stock from a pool of items.
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();

        // Pool of items the merchant can sell
        let pool: Vec<(ItemId, i32)> = vec![
            // Consumables
            (ItemId::BasicHPPotion, rng.gen_range(3..=8)),
            // Basic weapons
            (ItemId::Sword, 1),
            (ItemId::Dagger, 1),
            (ItemId::TinSword, 1),
            (ItemId::CopperSword, 1),
            (ItemId::BronzeSword, 1),
            // Shields
            (ItemId::BasicShield, 1),
            // Copper armor
            (ItemId::CopperHelmet, 1),
            (ItemId::CopperChestplate, 1),
            (ItemId::CopperGauntlets, 1),
            (ItemId::CopperGreaves, 1),
            (ItemId::CopperLeggings, 1),
            // Tin armor
            (ItemId::TinHelmet, 1),
            (ItemId::TinChestplate, 1),
            (ItemId::TinGauntlets, 1),
            (ItemId::TinGreaves, 1),
            (ItemId::TinLeggings, 1),
            // Bronze armor
            (ItemId::BronzeHelmet, 1),
            (ItemId::BronzeChestplate, 1),
            (ItemId::BronzeGauntlets, 1),
            (ItemId::BronzeGreaves, 1),
            (ItemId::BronzeLeggings, 1),
            // Tools
            (ItemId::BronzePickaxe, 1),
            // Accessories
            (ItemId::GoldRing, 1),
            // Ores
            (ItemId::CopperOre, rng.gen_range(5..=15)),
            (ItemId::TinOre, rng.gen_range(5..=15)),
            (ItemId::Coal, rng.gen_range(5..=15)),
            // Ingots
            (ItemId::CopperIngot, rng.gen_range(2..=5)),
            (ItemId::TinIngot, rng.gen_range(2..=5)),
            (ItemId::BronzeIngot, rng.gen_range(1..=3)),
            // Materials
            (ItemId::Cowhide, rng.gen_range(3..=8)),
            (ItemId::SlimeGel, rng.gen_range(3..=8)),
        ];

        // Randomly select 8-12 items from the pool
        let num_items = rng.gen_range(8..=12).min(pool.len());
        let mut selected_indices: Vec<usize> = (0..pool.len()).collect();

        // Shuffle and take first num_items
        for i in 0..num_items {
            let swap_idx = rng.gen_range(i..pool.len());
            selected_indices.swap(i, swap_idx);
        }

        let items = selected_indices
            .into_iter()
            .take(num_items)
            .map(|idx| {
                let (item_id, quantity) = pool[idx];
                StoreItem::new(item_id, quantity)
            })
            .collect();

        Self { items }
    }
}

/// Marker resource to trigger spawning the merchant modal.
#[derive(Resource)]
pub struct SpawnMerchantModal;

/// Marker resource to force detail pane refresh after buy/sell transactions.
#[derive(Resource)]
pub struct MerchantDetailRefresh;

/// Type-safe handle for the merchant modal.
///
/// Used with `ModalCommands`:
/// ```ignore
/// commands.toggle_modal::<MerchantModal>();
/// commands.close_modal::<MerchantModal>();
/// ```
pub struct MerchantModal;

impl RegisteredModal for MerchantModal {
    type Root = MerchantModalRoot;
    const MODAL_TYPE: ModalType = ModalType::MerchantModal;

    fn spawn(world: &mut World) {
        world.insert_resource(SpawnMerchantModal);
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<MerchantStock>();
    }
}
