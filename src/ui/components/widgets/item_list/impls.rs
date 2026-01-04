use std::borrow::Cow;

use ratatui::{style::Style, text::Span};

use crate::{
    economy::WorthGold,
    inventory::{EquipmentSlot, InventoryItem},
    item::{enums::ItemQuality, recipe::{ForgeMaterial, RecipeId}, Item},
    location::store::StoreItem,
    system::game_state,
    ui::theme as colors,
};

use super::traits::ListItem;

// =============================================================================
// InventoryItem implementations
// =============================================================================

/// Basic ListItem implementation for InventoryItem.
/// Used for simple inventory displays without additional context.
impl ListItem for InventoryItem {
    fn item(&self) -> Option<&Item> {
        Some(&self.item)
    }

    fn display_name(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.item.name)
    }

    fn quantity(&self) -> Option<u32> {
        if self.item.item_type.is_equipment() {
            None
        } else {
            Some(self.quantity)
        }
    }
}

// =============================================================================
// Inventory Modal wrapper
// =============================================================================

/// Wrapper for inventory items in the inventory modal.
/// Includes equipment slot information for equipped items.
#[derive(Clone)]
pub struct InventoryListItem {
    pub inv_item: InventoryItem,
    pub slot: Option<EquipmentSlot>,
}

impl ListItem for InventoryListItem {
    fn item(&self) -> Option<&Item> {
        Some(&self.inv_item.item)
    }

    fn display_name(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.inv_item.item.name)
    }

    fn quantity(&self) -> Option<u32> {
        if self.inv_item.item.item_type.is_equipment() {
            None
        } else {
            Some(self.inv_item.quantity)
        }
    }
}

// =============================================================================
// Store wrappers
// =============================================================================

/// Wrapper for store items in buy mode.
/// Shows item name, quantity in stock, and purchase price.
#[derive(Clone)]
pub struct StoreBuyItem {
    pub store_item: StoreItem,
    pub item_name: &'static str,
}

impl ListItem for StoreBuyItem {
    fn item(&self) -> Option<&Item> {
        self.store_item.display_item()
    }

    fn display_name(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.item_name)
    }

    fn quantity(&self) -> Option<u32> {
        Some(self.store_item.quantity() as u32)
    }

    fn suffix_spans(&self) -> Vec<Span<'static>> {
        if let Some(item) = self.store_item.display_item() {
            vec![
                Span::styled(format!("{:>6}g", item.purchase_price()), Style::default().fg(colors::YELLOW)),
            ]
        } else {
            vec![
                Span::styled("   ---", Style::default().fg(colors::GREY)),
            ]
        }
    }

    fn is_selectable(&self) -> bool {
        self.store_item.is_in_stock()
    }

    fn show_lock(&self) -> bool {
        false
    }
}

/// Wrapper for inventory items in sell mode.
/// Shows item name and sell price.
#[derive(Clone)]
pub struct SellableItem {
    pub inv_item: InventoryItem,
}

impl ListItem for SellableItem {
    fn item(&self) -> Option<&Item> {
        Some(&self.inv_item.item)
    }

    fn display_name(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.inv_item.item.name)
    }

    fn quantity(&self) -> Option<u32> {
        Some(self.inv_item.quantity)
    }

    fn suffix_spans(&self) -> Vec<Span<'static>> {
        vec![
            Span::styled(" - ", Style::default().fg(colors::WHITE)),
            Span::styled(format!("{}g", self.inv_item.item.sell_price()), Style::default().fg(colors::YELLOW)),
        ]
    }
}

// =============================================================================
// Blacksmith wrappers
// =============================================================================

/// Wrapper for equipment items in blacksmith upgrade mode.
/// Shows item name and upgrade cost (or MAX if at max upgrades).
#[derive(Clone)]
pub struct UpgradeableItem {
    pub inv_item: InventoryItem,
    pub upgrade_cost: i32,
    pub at_max: bool,
    pub can_afford: bool,
}

impl ListItem for UpgradeableItem {
    fn item(&self) -> Option<&Item> {
        Some(&self.inv_item.item)
    }

    fn display_name(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.inv_item.item.name)
    }

    fn quantity(&self) -> Option<u32> {
        None // Equipment doesn't show quantity
    }

    fn suffix_spans(&self) -> Vec<Span<'static>> {
        if self.at_max {
            vec![
                Span::styled(" - MAX", Style::default().fg(colors::DARK_GRAY)),
            ]
        } else {
            let cost_style = if self.can_afford {
                Style::default().fg(colors::YELLOW)
            } else {
                Style::default().fg(colors::RED)
            };
            vec![
                Span::styled(" - ", Style::default().fg(colors::WHITE)),
                Span::styled(format!("{} gold", self.upgrade_cost), cost_style),
            ]
        }
    }
}

/// Wrapper for equipment items in blacksmith quality upgrade mode.
/// Shows item name, current quality, and next quality (or MAX).
#[derive(Clone)]
pub struct QualityItem {
    pub inv_item: InventoryItem,
    pub next_quality: Option<ItemQuality>,
    pub can_afford: bool,
}

impl ListItem for QualityItem {
    fn item(&self) -> Option<&Item> {
        Some(&self.inv_item.item)
    }

    fn display_name(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.inv_item.item.name)
    }

    fn quantity(&self) -> Option<u32> {
        None
    }

    fn suffix_spans(&self) -> Vec<Span<'static>> {
        let current_quality = self.inv_item.item.quality;
        let current_color = colors::quality_color(current_quality);

        match self.next_quality {
            Some(next) => {
                let next_color = colors::quality_color(next);
                let arrow_style = if self.can_afford {
                    Style::default().fg(colors::WHITE)
                } else {
                    Style::default().fg(colors::RED)
                };
                vec![
                    Span::raw(" "),
                    Span::styled(format!("{:?}", current_quality), Style::default().fg(current_color)),
                    Span::styled(" -> ", arrow_style),
                    Span::styled(format!("{:?}", next), Style::default().fg(next_color)),
                ]
            }
            None => {
                vec![
                    Span::raw(" "),
                    Span::styled(format!("{:?}", current_quality), Style::default().fg(current_color)),
                    Span::styled(" MAX", Style::default().fg(colors::DARK_GRAY)),
                ]
            }
        }
    }
}

// =============================================================================
// Recipe wrapper for Forge and Brew
// =============================================================================

/// Wrapper for recipe items in forge/brew screens.
/// Shows recipe name with ingredient requirements (have/need).
#[derive(Clone)]
pub struct RecipeItem {
    pub recipe_id: RecipeId,
    pub name: &'static str,
    pub ingredients_display: String,
    pub can_craft: bool,
}

impl RecipeItem {
    /// Create a new RecipeItem with calculated ingredient display.
    pub fn new(recipe_id: RecipeId, name: &'static str) -> Self {
        use crate::inventory::HasInventory;
        use crate::item::recipe::Recipe;

        let gs = game_state();
        let recipe = Recipe::new(recipe_id).expect("Recipe should exist");

        let (ingredients_display, can_craft) = {
            let mut can_craft = true;
            let display = recipe
                .ingredients()
                .iter()
                .map(|(&item_id, &needed)| {
                    let have = gs.player.find_item_by_id(item_id)
                        .map(|inv| inv.quantity)
                        .unwrap_or(0);
                    if have < needed {
                        can_craft = false;
                    }
                    let item_name = item_id.spec().name;
                    format!("{}: {}/{}", item_name, have, needed)
                })
                .collect::<Vec<_>>()
                .join(", ");
            (display, can_craft)
        };

        Self {
            recipe_id,
            name,
            ingredients_display,
            can_craft,
        }
    }
}

impl ListItem for RecipeItem {
    fn item(&self) -> Option<&Item> {
        None // Recipes don't have an underlying Item
    }

    fn display_name(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.name)
    }

    fn quantity(&self) -> Option<u32> {
        None
    }

    fn suffix_spans(&self) -> Vec<Span<'static>> {
        let style = if self.can_craft {
            Style::default().fg(colors::WHITE)
        } else {
            Style::default().fg(colors::GREY)
        };
        vec![
            Span::styled(format!(" ({})", self.ingredients_display), style),
        ]
    }

    fn show_lock(&self) -> bool {
        false
    }

    fn forge_material(&self) -> Option<ForgeMaterial> {
        Some(self.recipe_id.material())
    }
}

// =============================================================================
// Storage wrappers
// =============================================================================

/// Wrapper for items in player inventory when viewing storage.
/// Items that are locked or equipped cannot be deposited.
#[derive(Clone)]
pub struct DepositableItem {
    pub inv_item: InventoryItem,
}

impl ListItem for DepositableItem {
    fn item(&self) -> Option<&Item> {
        Some(&self.inv_item.item)
    }

    fn display_name(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.inv_item.item.name)
    }

    fn quantity(&self) -> Option<u32> {
        if self.inv_item.item.item_type.is_equipment() {
            None
        } else {
            Some(self.inv_item.quantity)
        }
    }

    fn is_selectable(&self) -> bool {
        // Can't deposit locked or equipped items
        !self.inv_item.item.is_locked && !self.inv_item.item.is_equipped
    }
}

/// Wrapper for items in storage inventory.
#[derive(Clone)]
pub struct StoredItem {
    pub inv_item: InventoryItem,
}

impl ListItem for StoredItem {
    fn item(&self) -> Option<&Item> {
        Some(&self.inv_item.item)
    }

    fn display_name(&self) -> Cow<'static, str> {
        Cow::Borrowed(self.inv_item.item.name)
    }

    fn quantity(&self) -> Option<u32> {
        if self.inv_item.item.item_type.is_equipment() {
            None
        } else {
            Some(self.inv_item.quantity)
        }
    }

    fn show_lock(&self) -> bool {
        false // Storage items are never locked
    }
}
