use bevy::prelude::*;
use rand::Rng;

use crate::item::ItemId;
use crate::location::store::StoreItem;
use crate::ui::focus::FocusPanel;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;
use crate::ui::widgets::{DetailPaneContext, ItemGrid};
use crate::ui::InfoPanelSource;

/// Component marker for the merchant modal UI.
#[derive(Component)]
pub struct MerchantModalRoot;

/// Marker for the merchant stock grid (left side).
#[derive(Component)]
pub struct MerchantStockGrid;

/// Marker for the player inventory grid (right side).
#[derive(Component)]
pub struct MerchantPlayerGrid;

pub struct MerchantDetailPane;

impl DetailPaneContext for MerchantDetailPane {
    type LeftGridMarker = MerchantStockGrid;
    type RightGridMarker = MerchantPlayerGrid;

    const LEFT_FOCUS: FocusPanel = FocusPanel::MerchantStock;
    const RIGHT_FOCUS: FocusPanel = FocusPanel::PlayerInventory;

    fn source_from_left_grid(grid: &ItemGrid) -> InfoPanelSource {
        InfoPanelSource::Store {
            selected_index: grid.selected_index,
        }
    }

    fn source_from_right_grid(grid: &ItemGrid) -> InfoPanelSource {
        InfoPanelSource::Inventory {
            selected_index: grid.selected_index,
        }
    }
}

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
            (ItemId::GoldSword, 1),
            (ItemId::IronSword, 1),
            (ItemId::CopperSword, 1),
            // Shields
            (ItemId::BasicShield, 1),
            // Copper armor
            (ItemId::IronHelmet, 1),
            (ItemId::IronChestplate, 1),
            (ItemId::IronGauntlets, 1),
            (ItemId::IronGreaves, 1),
            (ItemId::IronLeggings, 1),
            // Tin armor
            (ItemId::GoldHelmet, 1),
            (ItemId::GoldChestplate, 1),
            (ItemId::GoldGauntlets, 1),
            (ItemId::GoldGreaves, 1),
            (ItemId::GoldLeggings, 1),
            // Bronze armor
            (ItemId::CopperHelmet, 1),
            (ItemId::CopperChestplate, 1),
            (ItemId::CopperGauntlets, 1),
            (ItemId::CopperGreaves, 1),
            (ItemId::CopperLeggings, 1),
            // Tools
            (ItemId::CopperPickaxe, 1),
            // Accessories
            (ItemId::GoldRing, 1),
            // Ores
            (ItemId::IronOre, rng.gen_range(5..=15)),
            (ItemId::GoldOre, rng.gen_range(5..=15)),
            (ItemId::Coal, rng.gen_range(5..=15)),
            // Ingots
            (ItemId::IronIngot, rng.gen_range(2..=5)),
            (ItemId::GoldIngot, rng.gen_range(2..=5)),
            (ItemId::CopperIngot, rng.gen_range(1..=3)),
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
        world.run_system_cached(do_spawn_merchant_modal).ok();
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<MerchantStock>();
    }
}

fn do_spawn_merchant_modal(
    commands: Commands,
    stock: Res<MerchantStock>,
    player: Query<&crate::inventory::Inventory, With<crate::player::PlayerMarker>>,
) {
    let Ok(inventory) = player.single() else {
        return;
    };
    super::render::spawn_merchant_modal_impl(commands, &stock, inventory);
}
