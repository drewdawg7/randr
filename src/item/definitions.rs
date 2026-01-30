//! Item definitions using the entity_macros system
//!
//! This file consolidates:
//! - ItemSpec struct definition
//! - ItemId enum
//! - All item spec constants
//! - The spec() method on ItemId

use crate::assets::SpriteSheetKey;
use crate::inventory::EquipmentSlot;
use crate::stats::{StatSheet, StatType};

pub use super::enums::{
    ConsumableType, EquipmentType, ItemQuality, ItemType, MaterialType, ToolKind,
};

entity_macros::define_entity! {
    spec ItemSpec {
        pub name: String,
        pub item_type: ItemType,
        pub quality: Option<ItemQuality>,
        pub max_upgrades: i32,
        pub max_stack_quantity: u32,
        pub stats: StatSheet,
        pub gold_value: i32,
    }

    id ItemId;

    sprites(default_sheet: SpriteSheetKey::IconItems);

    variants {
        // ─────────────────────────────────────────────────────────────────────
        // Weapons
        // ─────────────────────────────────────────────────────────────────────
        Sword {
            name: String::from("Sword"),
            item_type: ItemType::Equipment(EquipmentType::Weapon),
            quality: None,
            stats: StatSheet::new().with(StatType::Attack, 10),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 15,
            @sprite: "Slice_155",
        }
        Dagger {
            name: String::from("Dagger"),
            item_type: ItemType::Equipment(EquipmentType::Weapon),
            quality: None,
            stats: StatSheet::new().with(StatType::Attack, 6),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 10,
            @sprite: "Slice_156",
        }
        GoldSword {
            name: String::from("Gold Sword"),
            item_type: ItemType::Equipment(EquipmentType::Weapon),
            quality: None,
            stats: StatSheet::new().with(StatType::Attack, 12),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 15,
            @sprite: "gold_sword" in SpriteSheetKey::GoldSword,
        }
        IronSword {
            name: String::from("Iron Sword"),
            item_type: ItemType::Equipment(EquipmentType::Weapon),
            quality: None,
            stats: StatSheet::new().with(StatType::Attack, 12),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 15,
            @sprite: "iron_sword" in SpriteSheetKey::IronSword,
        }
        CopperSword {
            name: String::from("Copper Sword"),
            item_type: ItemType::Equipment(EquipmentType::Weapon),
            quality: None,
            stats: StatSheet::new().with(StatType::Attack, 16),
            max_upgrades: 7,
            max_stack_quantity: 1,
            gold_value: 25,
            @sprite: "copper_sword" in SpriteSheetKey::CopperSword,
        }
        BonkStick {
            name: String::from("BONK STICK"),
            item_type: ItemType::Equipment(EquipmentType::Weapon),
            quality: Some(ItemQuality::Mythic),
            stats: StatSheet::new().with(StatType::Attack, 100),
            max_upgrades: 99,
            max_stack_quantity: 1,
            gold_value: 25000,
            @sprite: "Slice_607",
        }

        // ─────────────────────────────────────────────────────────────────────
        // Shields
        // ─────────────────────────────────────────────────────────────────────
        BasicShield {
            name: String::from("Basic Shield"),
            item_type: ItemType::Equipment(EquipmentType::Shield),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 4),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 15,
            @sprite: "Slice_100",
        }

        IronHelmet {
            name: String::from("Iron Helmet"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Head)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 36),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 180,
            @sprite: "iron_helmet" in SpriteSheetKey::Headgear,
        }
        IronChestplate {
            name: String::from("Iron Chestplate"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Chest)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 60),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 300,
            @sprite: "iron_chestplate" in SpriteSheetKey::Chestplates,
        }
        IronGauntlets {
            name: String::from("Iron Gauntlets"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Hands)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 24),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 120,
            @sprite: "Slice_558",
        }
        IronGreaves {
            name: String::from("Iron Greaves"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Feet)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 30),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 150,
            @sprite: "iron_greaves" in SpriteSheetKey::Greaves,
        }
        IronLeggings {
            name: String::from("Iron Leggings"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Legs)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 54),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 270,
            @sprite: "iron_leggings" in SpriteSheetKey::Leggings,
        }

        GoldHelmet {
            name: String::from("Gold Helmet"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Head)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 36),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 180,
            @sprite: "gold_helmet" in SpriteSheetKey::Headgear,
        }
        GoldChestplate {
            name: String::from("Gold Chestplate"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Chest)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 60),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 300,
            @sprite: "gold_chestplate" in SpriteSheetKey::Chestplates,
        }
        GoldGauntlets {
            name: String::from("Gold Gauntlets"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Hands)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 24),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 120,
            @sprite: "Slice_558",
        }
        GoldGreaves {
            name: String::from("Gold Greaves"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Feet)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 30),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 150,
            @sprite: "gold_greaves" in SpriteSheetKey::Greaves,
        }
        GoldLeggings {
            name: String::from("Gold Leggings"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Legs)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 54),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 270,
            @sprite: "gold_leggings" in SpriteSheetKey::Leggings,
        }

        CopperHelmet {
            name: String::from("Copper Helmet"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Head)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 48),
            max_upgrades: 7,
            max_stack_quantity: 1,
            gold_value: 270,
            @sprite: "copper_helmet" in SpriteSheetKey::Headgear,
        }
        CopperChestplate {
            name: String::from("Copper Chestplate"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Chest)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 80),
            max_upgrades: 7,
            max_stack_quantity: 1,
            gold_value: 450,
            @sprite: "copper_chestplate" in SpriteSheetKey::Chestplates,
        }
        CopperGauntlets {
            name: String::from("Copper Gauntlets"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Hands)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 32),
            max_upgrades: 7,
            max_stack_quantity: 1,
            gold_value: 180,
            @sprite: "Slice_558",
        }
        CopperGreaves {
            name: String::from("Copper Greaves"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Feet)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 40),
            max_upgrades: 7,
            max_stack_quantity: 1,
            gold_value: 225,
            @sprite: "copper_greaves" in SpriteSheetKey::Greaves,
        }
        CopperLeggings {
            name: String::from("Copper Leggings"),
            item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Legs)),
            quality: None,
            stats: StatSheet::new().with(StatType::Defense, 72),
            max_upgrades: 7,
            max_stack_quantity: 1,
            gold_value: 405,
            @sprite: "copper_leggings" in SpriteSheetKey::Leggings,
        }

        CopperPickaxe {
            name: String::from("Copper Pickaxe"),
            item_type: ItemType::Equipment(EquipmentType::Tool(ToolKind::Pickaxe)),
            quality: None,
            stats: StatSheet::new().with(StatType::Attack, 10).with(StatType::Mining, 10),
            max_upgrades: 5,
            max_stack_quantity: 1,
            gold_value: 50,
            @sprite: "Slice_826",
        }

        GoldRing {
            name: String::from("Midas' Touch"),
            item_type: ItemType::Equipment(EquipmentType::Ring),
            quality: None,
            stats: StatSheet::new().with(StatType::GoldFind, 10),
            max_upgrades: 7,
            max_stack_quantity: 1,
            gold_value: 50,
            @sprite: "gold_ring" in SpriteSheetKey::GoldRing,
        }
        ImbaRing {
            name: String::from("IMBA RING"),
            item_type: ItemType::Equipment(EquipmentType::Ring),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new().with(StatType::GoldFind, 200).with(StatType::MagicFind, 200),
            max_upgrades: 99,
            max_stack_quantity: 1,
            gold_value: 25000,
            @sprite: "Slice_1009",
        }

        Coal {
            name: String::from("Coal"),
            item_type: ItemType::Material(MaterialType::Fuel),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 4,
            @sprite: "Slice_693",
        }
        CopperOre {
            name: String::from("Copper Ore"),
            item_type: ItemType::Material(MaterialType::Ore),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 5,
            @sprite: "copper_ore" in SpriteSheetKey::CraftingMaterials,
        }
        IronOre {
            name: String::from("Iron Ore"),
            item_type: ItemType::Material(MaterialType::Ore),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 5,
            @sprite: "iron_ore" in SpriteSheetKey::CraftingMaterials,
        }
        GoldOre {
            name: String::from("Gold Ore"),
            item_type: ItemType::Material(MaterialType::Ore),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 5,
            @sprite: "gold_ore" in SpriteSheetKey::CraftingMaterials,
        }

        GoldIngot {
            name: String::from("Gold Ingot"),
            item_type: ItemType::Material(MaterialType::CraftingMaterial),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 10,
            @sprite: "gold_ingot" in SpriteSheetKey::CraftingMaterials,
        }
        IronIngot {
            name: String::from("Iron Ingot"),
            item_type: ItemType::Material(MaterialType::CraftingMaterial),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 10,
            @sprite: "iron_ingot" in SpriteSheetKey::CraftingMaterials,
        }
        CopperIngot {
            name: String::from("Copper Ingot"),
            item_type: ItemType::Material(MaterialType::CraftingMaterial),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 15,
            @sprite: "copper_ingot" in SpriteSheetKey::CraftingMaterials,
        }

        Cowhide {
            name: String::from("Cowhide"),
            item_type: ItemType::Material(MaterialType::CraftingMaterial),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 4,
            @sprite: "leather" in SpriteSheetKey::CraftingMaterials,
        }
        SlimeGel {
            name: String::from("Slime Gel"),
            item_type: ItemType::Material(MaterialType::CraftingMaterial),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 4,
            @sprite: "Slice_952",
        }

        BasicHPPotion {
            name: String::from("Basic HP Potion"),
            item_type: ItemType::Consumable(ConsumableType::Potion),
            quality: Some(ItemQuality::Normal),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 25,
            @sprite: "Slice_337",
        }

        QualityUpgradeStone {
            name: String::from("Magic Rock"),
            item_type: ItemType::Material(MaterialType::UpgradeStone),
            quality: Some(ItemQuality::Mythic),
            stats: StatSheet::new(),
            max_upgrades: 0,
            max_stack_quantity: 99,
            gold_value: 500,
            @sprite: "Slice_57",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Spawn Implementation
// ─────────────────────────────────────────────────────────────────────────────

use uuid::Uuid;
use crate::registry::{RegistryDefaults, SpawnFromSpec};
use super::definition::Item;

impl SpawnFromSpec<ItemId> for ItemSpec {
    type Output = Item;

    fn spawn_from_spec(item_id: ItemId, spec: &Self) -> Self::Output {
        spec.spawn(item_id)
    }
}

impl ItemSpec {
    /// Spawn an Item from this spec with the given ItemId. Internal use only.
    fn spawn(&self, id: ItemId) -> Item {
        // Use fixed quality from spec, or roll if None
        let quality = self.quality.unwrap_or_else(ItemQuality::roll);
        let base_stats = self.stats.clone();
        let stats = quality.multiply_stats(&base_stats);

        Item {
            item_uuid: Uuid::new_v4(),
            item_id: id,
            item_type: self.item_type,
            name: self.name.clone(),
            is_equipped: false,
            is_locked: false,
            num_upgrades: 0,
            max_upgrades: self.max_upgrades,
            max_stack_quantity: self.max_stack_quantity,
            gold_value: self.gold_value,
            base_stats,
            stats,
            quality,
        }
    }

    /// Create a scaled copy of this spec with stats multiplied by the given factor.
    /// Useful for dungeon scaling, elite variants, etc.
    pub fn with_multiplier(&self, multiplier: f32) -> ItemSpec {
        let mut scaled_stats = crate::stats::StatSheet::new();
        for stat_type in crate::stats::StatType::all() {
            let value = self.stats.value(*stat_type);
            if value > 0 {
                let new_value = (value as f32 * multiplier).round() as i32;
                scaled_stats.insert(stat_type.instance(new_value));
            }
        }

        ItemSpec {
            name: self.name.clone(),
            item_type: self.item_type,
            quality: self.quality,
            max_upgrades: self.max_upgrades,
            max_stack_quantity: self.max_stack_quantity,
            stats: scaled_stats,
            gold_value: (self.gold_value as f32 * multiplier).round() as i32,
        }
    }

    /// Create a copy with a new name (e.g., for "Enchanted Sword").
    pub fn with_name(&self, name: impl Into<String>) -> ItemSpec {
        ItemSpec {
            name: name.into(),
            item_type: self.item_type,
            quality: self.quality,
            max_upgrades: self.max_upgrades,
            max_stack_quantity: self.max_stack_quantity,
            stats: self.stats.clone(),
            gold_value: self.gold_value,
        }
    }

    /// Create a copy with a fixed quality.
    pub fn with_quality(&self, quality: ItemQuality) -> ItemSpec {
        ItemSpec {
            name: self.name.clone(),
            item_type: self.item_type,
            quality: Some(quality),
            max_upgrades: self.max_upgrades,
            max_stack_quantity: self.max_stack_quantity,
            stats: self.stats.clone(),
            gold_value: self.gold_value,
        }
    }
}

impl RegistryDefaults<ItemId> for ItemSpec {
    fn defaults() -> impl IntoIterator<Item = (ItemId, Self)> {
        ItemId::ALL.iter().map(|id| (*id, id.spec().clone()))
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Spawning
// ─────────────────────────────────────────────────────────────────────────────

/// Builder for spawning modified items.
pub struct ItemSpawner {
    id: ItemId,
    spec: ItemSpec,
}

impl ItemSpawner {
    /// Spawn the item with all modifications applied.
    pub fn spawn(self) -> Item {
        self.spec.spawn(self.id)
    }

    /// Scale all stat values and gold by a multiplier.
    pub fn with_multiplier(mut self, multiplier: f32) -> Self {
        self.spec = self.spec.with_multiplier(multiplier);
        self
    }

    /// Change the item's display name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.spec = self.spec.with_name(name);
        self
    }

    /// Set the item's quality.
    pub fn with_quality(mut self, quality: ItemQuality) -> Self {
        self.spec = self.spec.with_quality(quality);
        self
    }
}

impl ItemId {
    /// Spawn an Item instance from this ItemId.
    pub fn spawn(&self) -> Item {
        self.spec().spawn(*self)
    }

    /// Scale all stat values and gold by a multiplier.
    pub fn with_multiplier(&self, multiplier: f32) -> ItemSpawner {
        ItemSpawner {
            id: *self,
            spec: self.spec().with_multiplier(multiplier),
        }
    }

    /// Change the item's display name.
    pub fn with_name(&self, name: impl Into<String>) -> ItemSpawner {
        ItemSpawner {
            id: *self,
            spec: self.spec().with_name(name),
        }
    }

    pub fn with_quality(&self, quality: ItemQuality) -> ItemSpawner {
        ItemSpawner {
            id: *self,
            spec: self.spec().with_quality(quality),
        }
    }
}
