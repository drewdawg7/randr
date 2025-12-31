use rand::Rng;

use crate::stats::StatSheet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemId {
    Sword,
    Dagger,
    BasicShield,
    QualityUpgradeStone,
    GoldRing,
    BronzePickaxe,
    Coal,
    CopperOre,
    TinOre,
}

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
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MaterialType {
    Ore,
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
        }
    }
}

pub enum ItemError {
    MaxUpgradesReached,
    NotEquipment,
    MaxQualityReached,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..100);

        match roll {
            0..=9   => ItemQuality::Poor,         // 10%
            10..=69 => ItemQuality::Normal,       // 60%
            70..=84 => ItemQuality::Improved,     // 15%
            85..=94 => ItemQuality::WellForged,   // 10%
            95..=97 => ItemQuality::Masterworked, // 3%
            _       => ItemQuality::Mythic,       // 2%
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
    pub fn multiply_stats(&self, mut sheet: StatSheet) -> StatSheet {
        let multiplier = self.multiplier();
        for stat in sheet.stats_mut().values_mut() {
            stat.current_value = ((stat.current_value as f64) * multiplier).round() as i32;
            stat.max_value     = ((stat.max_value     as f64) * multiplier).round() as i32;
        }
        sheet
    } 
}
