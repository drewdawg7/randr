use std::borrow::Cow;

use ratatui::text::Span;

use crate::item::{recipe::ForgeMaterial, Item, ItemType};

/// Core trait for items that can be displayed in an ItemList.
/// Implement this for any type that should appear in an item list.
#[allow(dead_code)]
pub trait ListItem {
    /// Reference to the underlying Item (for quality color, lock status, etc.)
    /// Return None for non-item entries like recipe items.
    fn item(&self) -> Option<&Item>;

    /// Primary display text for the item
    fn display_name(&self) -> Cow<'static, str>;

    /// Quantity to display. None for equipment (shows upgrade count instead).
    fn quantity(&self) -> Option<u32>;

    /// Additional suffix spans after the item name (price, cost, etc.)
    fn suffix_spans(&self) -> Vec<Span<'static>> {
        vec![]
    }

    /// Whether this item can be selected/interacted with
    fn is_selectable(&self) -> bool {
        true
    }

    /// Whether to show the lock prefix icon
    fn show_lock(&self) -> bool {
        self.item().map(|i| i.is_locked).unwrap_or(false)
    }

    /// Material type for forge filtering (only relevant for recipes)
    fn forge_material(&self) -> Option<ForgeMaterial> {
        None
    }
}

/// Trait for filtering items in an ItemList.
/// Implement this to create custom filters.
pub trait ItemFilter<T>: Clone + Default {
    /// Label to display on the filter button
    fn label(&self) -> &'static str;

    /// Returns true if the item passes the filter
    fn matches(&self, item: &T) -> bool;

    /// Returns the next filter in the cycle
    fn next(&self) -> Self;
}

/// No-op filter for lists that don't need filtering.
/// Always matches all items.
#[derive(Clone, Copy, Default)]
pub struct NoFilter;

impl<T> ItemFilter<T> for NoFilter {
    fn label(&self) -> &'static str {
        "All"
    }

    fn matches(&self, _item: &T) -> bool {
        true
    }

    fn next(&self) -> Self {
        NoFilter
    }
}

/// Standard inventory filter for filtering by item type.
/// Used by the inventory modal.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum InventoryFilter {
    #[default]
    All,
    Equipment,
    Materials,
    Consumables,
}

impl InventoryFilter {
    /// Check if an item type matches this filter
    pub fn matches_item_type(&self, item_type: &ItemType) -> bool {
        match self {
            InventoryFilter::All => true,
            InventoryFilter::Equipment => item_type.is_equipment(),
            InventoryFilter::Materials => item_type.is_material(),
            InventoryFilter::Consumables => item_type.is_consumable(),
        }
    }
}

impl<T: ListItem> ItemFilter<T> for InventoryFilter {
    fn label(&self) -> &'static str {
        match self {
            InventoryFilter::All => "All",
            InventoryFilter::Equipment => "Equipment",
            InventoryFilter::Materials => "Materials",
            InventoryFilter::Consumables => "Consumables",
        }
    }

    fn matches(&self, item: &T) -> bool {
        match item.item() {
            Some(i) => self.matches_item_type(&i.item_type),
            None => true, // Non-item entries always pass
        }
    }

    fn next(&self) -> Self {
        match self {
            InventoryFilter::All => InventoryFilter::Equipment,
            InventoryFilter::Equipment => InventoryFilter::Materials,
            InventoryFilter::Materials => InventoryFilter::Consumables,
            InventoryFilter::Consumables => InventoryFilter::All,
        }
    }
}

/// Filter for forge recipes by material type.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ForgeFilter {
    #[default]
    All,
    Copper,
    Tin,
    Bronze,
    Other,
}

impl<T: ListItem> ItemFilter<T> for ForgeFilter {
    fn label(&self) -> &'static str {
        match self {
            ForgeFilter::All => "All",
            ForgeFilter::Copper => "Copper",
            ForgeFilter::Tin => "Tin",
            ForgeFilter::Bronze => "Bronze",
            ForgeFilter::Other => "Other",
        }
    }

    fn matches(&self, item: &T) -> bool {
        match self {
            ForgeFilter::All => true,
            ForgeFilter::Copper => item.forge_material() == Some(ForgeMaterial::Copper),
            ForgeFilter::Tin => item.forge_material() == Some(ForgeMaterial::Tin),
            ForgeFilter::Bronze => item.forge_material() == Some(ForgeMaterial::Bronze),
            ForgeFilter::Other => item.forge_material() == Some(ForgeMaterial::Other),
        }
    }

    fn next(&self) -> Self {
        match self {
            ForgeFilter::All => ForgeFilter::Copper,
            ForgeFilter::Copper => ForgeFilter::Tin,
            ForgeFilter::Tin => ForgeFilter::Bronze,
            ForgeFilter::Bronze => ForgeFilter::Other,
            ForgeFilter::Other => ForgeFilter::All,
        }
    }
}
