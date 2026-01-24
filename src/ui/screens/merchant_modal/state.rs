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
            // Potions (higher quantities)
            (ItemId::BasicHPPotion, rng.gen_range(3..=8)),
            // Basic weapons
            (ItemId::Sword, 1),
            (ItemId::Dagger, 1),
            (ItemId::BasicShield, 1),
            // Ores (random selection)
            (ItemId::CopperOre, rng.gen_range(5..=15)),
            (ItemId::TinOre, rng.gen_range(5..=15)),
            (ItemId::Coal, rng.gen_range(5..=15)),
            // Ingots
            (ItemId::CopperIngot, rng.gen_range(2..=5)),
            (ItemId::TinIngot, rng.gen_range(2..=5)),
            (ItemId::BronzeIngot, rng.gen_range(1..=3)),
        ];

        // Randomly select 6-10 items from the pool
        let num_items = rng.gen_range(6..=10).min(pool.len());
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
