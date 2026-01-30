use bevy::prelude::Color;
use rand::Rng;

use crate::skills::blacksmith_quality_bonus;
use crate::stats::StatSheet;

// ItemId is now defined in definitions.rs via macro

// ─────────────────────────────────────────────────────────────────────────────
// Item Type Hierarchy
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemType {
    Equipment(EquipmentType),
    Material(MaterialType),
    Consumable(ConsumableType),
    QuestItem,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EquipmentType {
    Weapon,
    Shield,
    Ring,
    Tool(ToolKind),
    Armor(crate::inventory::EquipmentSlot),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MaterialType {
    Ore,
    Fuel,
    Gem,
    CraftingMaterial,
    UpgradeStone,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ConsumableType {
    Potion,
    Food,
    Scroll,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ToolKind {
    Pickaxe,
}

// ─────────────────────────────────────────────────────────────────────────────
// ItemType Helper Methods
// ─────────────────────────────────────────────────────────────────────────────

impl ItemType {
    pub fn is_equipment(&self) -> bool {
        matches!(self, ItemType::Equipment(_))
    }

    pub fn is_stackable(&self) -> bool {
        !self.is_equipment() && !self.is_quest_item()
    }

    pub fn is_tool(&self) -> bool {
        matches!(self, ItemType::Equipment(EquipmentType::Tool(_)))
    }

    pub fn is_material(&self) -> bool {
        matches!(self, ItemType::Material(_))
    }

    pub fn is_consumable(&self) -> bool {
        matches!(self, ItemType::Consumable(_))
    }

    pub fn is_quest_item(&self) -> bool {
        matches!(self, ItemType::QuestItem)
    }

    pub fn equipment_slot(&self) -> Option<crate::inventory::EquipmentSlot> {
        match self {
            ItemType::Equipment(eq) => Some(eq.slot()),
            _ => None,
        }
    }
}

impl EquipmentType {
    pub fn slot(&self) -> crate::inventory::EquipmentSlot {
        use crate::inventory::EquipmentSlot;
        match self {
            EquipmentType::Weapon => EquipmentSlot::Weapon,
            EquipmentType::Shield => EquipmentSlot::OffHand,
            EquipmentType::Ring => EquipmentSlot::Ring,
            EquipmentType::Tool(_) => EquipmentSlot::Tool,
            EquipmentType::Armor(slot) => *slot,
        }
    }
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Equipment(eq) => write!(f, "Equipment ({})", eq),
            ItemType::Material(mat) => write!(f, "Material ({})", mat),
            ItemType::Consumable(con) => write!(f, "Consumable ({})", con),
            ItemType::QuestItem => write!(f, "Quest Item"),
        }
    }
}

impl std::fmt::Display for EquipmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EquipmentType::Weapon => write!(f, "Weapon"),
            EquipmentType::Shield => write!(f, "Shield"),
            EquipmentType::Ring => write!(f, "Ring"),
            EquipmentType::Tool(kind) => write!(f, "{}", kind),
            EquipmentType::Armor(slot) => write!(f, "{:?}", slot),
        }
    }
}

impl std::fmt::Display for MaterialType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaterialType::Ore => write!(f, "Ore"),
            MaterialType::Fuel => write!(f, "Fuel"),
            MaterialType::Gem => write!(f, "Gem"),
            MaterialType::CraftingMaterial => write!(f, "Crafting Material"),
            MaterialType::UpgradeStone => write!(f, "Upgrade Stone"),
        }
    }
}

impl std::fmt::Display for ConsumableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsumableType::Potion => write!(f, "Potion"),
            ConsumableType::Food => write!(f, "Food"),
            ConsumableType::Scroll => write!(f, "Scroll"),
        }
    }
}

impl std::fmt::Display for ToolKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolKind::Pickaxe => write!(f, "Pickaxe"),
        }
    }
}

#[derive(Debug)]
pub enum ItemError {
    MaxUpgradesReached,
    NotEquipment,
    MaxQualityReached,
    NotAConsumable,
}

/// Result of an item upgrade, containing the new level and stat increases
#[derive(Debug, Clone)]
pub struct UpgradeResult {
    /// The new upgrade level after the upgrade
    pub new_level: i32,
    /// The delta of stats that were increased
    pub stat_increases: StatSheet,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum ItemQuality {
    Poor,
    Normal,
    Improved,
    WellForged,
    Masterworked,
    Mythic
}
impl ItemQuality {
    /// Returns the human-readable display name for this quality level
    pub fn display_name(&self) -> &'static str {
        match self {
            ItemQuality::Poor => "Poor",
            ItemQuality::Normal => "Normal",
            ItemQuality::Improved => "Improved",
            ItemQuality::WellForged => "Well-Forged",
            ItemQuality::Masterworked => "Masterworked",
            ItemQuality::Mythic => "Mythic",
        }
    }

    /// Returns the display color for this quality level
    pub fn color(&self) -> Color {
        match self {
            ItemQuality::Poor => Color::srgb(0.6, 0.6, 0.6),
            ItemQuality::Normal => Color::srgb(1.0, 1.0, 1.0),
            ItemQuality::Improved => Color::srgb(0.3, 1.0, 0.3),
            ItemQuality::WellForged => Color::srgb(0.3, 0.5, 1.0),
            ItemQuality::Masterworked => Color::srgb(0.8, 0.3, 1.0),
            ItemQuality::Mythic => Color::srgb(1.0, 0.5, 0.0),
        }
    }

    pub fn next_quality(&self) -> Option<ItemQuality>{
            match self {
                ItemQuality::Poor         => Some(ItemQuality::Normal),
                ItemQuality::Normal       => Some(ItemQuality::Improved),
                ItemQuality::Improved     => Some(ItemQuality::WellForged),
                ItemQuality::WellForged   => Some(ItemQuality::Masterworked),
                ItemQuality::Masterworked => Some(ItemQuality::Mythic),
                ItemQuality::Mythic       => None
            }
    }
    pub fn roll() -> Self {
        Self::roll_with_bonus(0)
    }

    pub fn roll_with_bonus(blacksmith_level: u32) -> Self {
        let mut rng = rand::thread_rng();
        let bonus = blacksmith_quality_bonus(blacksmith_level);
        let roll = rng.gen_range(0..100) + bonus;

        match roll {
            ..=9    => ItemQuality::Poor,
            10..=69 => ItemQuality::Normal,
            70..=84 => ItemQuality::Improved,
            85..=94 => ItemQuality::WellForged,
            95..=97 => ItemQuality::Masterworked,
            _       => ItemQuality::Mythic,
        }
    }

    pub fn multiplier(&self) -> f64 {
        match self {
            ItemQuality::Poor         => 0.80,
            ItemQuality::Normal       => 1.0,
            ItemQuality::Improved     => 1.2,
            ItemQuality::WellForged   => 1.4,
            ItemQuality::Masterworked => 1.6,
            ItemQuality::Mythic       => 1.8,
        }
    }

    pub fn value_multiplier(&self) -> f64 {
        match self {
            ItemQuality::Poor         => 0.90,
            ItemQuality::Normal       => 1.0,
            ItemQuality::Improved     => 1.1,
            ItemQuality::WellForged   => 1.2,
            ItemQuality::Masterworked => 1.3,
            ItemQuality::Mythic       => 1.4,
        }
    }
    pub fn upgrade_cost_multiplier(&self) -> f64 {
        match self {
            ItemQuality::Poor         => 0.90,
            ItemQuality::Normal       => 1.0,
            ItemQuality::Improved     => 1.1,
            ItemQuality::WellForged   => 1.2,
            ItemQuality::Masterworked => 1.3,
            ItemQuality::Mythic       => 1.4,
        }
    }
    pub fn multiply_stats(&self, sheet: &StatSheet) -> StatSheet {
        let multiplier = self.multiplier();
        let mut result = sheet.clone();
        for stat in result.stats_mut().values_mut() {
            stat.current_value = ((stat.current_value as f64) * multiplier).round() as i32;
            stat.max_value     = ((stat.max_value     as f64) * multiplier).round() as i32;
        }
        result
    }
}
