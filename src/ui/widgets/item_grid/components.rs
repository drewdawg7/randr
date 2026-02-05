use bevy::prelude::*;

use crate::input::NavigationDirection;
use crate::inventory::{Inventory, InventoryItem, ManagesItems};
use crate::ui::focus::FocusPanel;

#[derive(Clone)]
pub struct ItemGridEntry {
    pub sprite_sheet_key: crate::assets::SpriteSheetKey,
    pub sprite_name: String,
    pub quantity: u32,
}

impl ItemGridEntry {
    pub fn from_inventory_item(inv_item: &InventoryItem) -> Self {
        Self {
            sprite_sheet_key: inv_item.item.item_id.sprite_sheet_key(),
            sprite_name: inv_item.item.item_id.sprite_name().to_string(),
            quantity: inv_item.quantity,
        }
    }

    pub fn from_inventory(inventory: &Inventory) -> Vec<Self> {
        inventory
            .get_inventory_items()
            .iter()
            .map(Self::from_inventory_item)
            .collect()
    }
}

#[derive(Component)]
pub struct ItemGrid {
    pub items: Vec<ItemGridEntry>,
    pub selected_index: usize,
    pub grid_size: usize,
}

impl Default for ItemGrid {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            selected_index: 0,
            grid_size: 4,
        }
    }
}

impl ItemGrid {
    pub fn clamp_selection(&mut self) {
        if self.items.is_empty() {
            self.selected_index = 0;
        } else {
            self.selected_index = self.selected_index.min(self.items.len() - 1);
        }
    }

    pub fn navigate(&mut self, direction: NavigationDirection) {
        let gs = self.grid_size;
        let total_slots = gs * gs;

        let current = self.selected_index;
        let row = current / gs;
        let col = current % gs;

        let new_index = match direction {
            NavigationDirection::Left if col > 0 => current - 1,
            NavigationDirection::Right if col < gs - 1 => current + 1,
            NavigationDirection::Up if row > 0 => current - gs,
            NavigationDirection::Down if row < gs - 1 => current + gs,
            _ => current,
        };

        if new_index < total_slots {
            self.selected_index = new_index;
        }
    }
}

#[derive(Component)]
pub struct ItemGridFocusPanel(pub FocusPanel);

#[derive(Component)]
pub(super) struct GridItemSprite;

#[derive(Component)]
pub(super) struct GridItemQuantityText;
