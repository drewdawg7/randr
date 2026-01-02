use once_cell::sync::Lazy;

use crate::{
    inventory::EquipmentSlot,
    item::enums::{EquipmentType, ItemQuality, ItemType, MaterialType, ToolKind},
    stats::{StatSheet, StatType},
};

use super::definition::ItemSpec;


pub static BRONZE_SWORD: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Bronze Sword",
    item_type: ItemType::Equipment(EquipmentType::Weapon),
    quality: None,
    stats: StatSheet::new().with(StatType::Attack, 16),
    max_upgrades: 7,
    max_stack_quantity: 1,
    gold_value: 25,
});
pub static TIN_SWORD: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Tin Sword",
    item_type: ItemType::Equipment(EquipmentType::Weapon),
    quality: None,
    stats: StatSheet::new().with(StatType::Attack, 12),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 15,
});

pub static COPPER_SWORD: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Copper Sword",
    item_type: ItemType::Equipment(EquipmentType::Weapon),
    quality: None,
    stats: StatSheet::new().with(StatType::Attack, 12),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 15,
});
pub static BRONZE_INGOT: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Bronze Ingot",
    item_type: ItemType::Material(MaterialType::CraftingMaterial),
    quality: Some(ItemQuality::Normal),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 15,
});

pub static TIN_INGOT: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Tin Ingot",
    item_type: ItemType::Material(MaterialType::CraftingMaterial),
    quality: Some(ItemQuality::Normal),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 10,
});
pub static COPPER_INGOT: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Copper Ingot",
    item_type: ItemType::Material(MaterialType::CraftingMaterial),
    quality: Some(ItemQuality::Normal),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 10,
});
pub static BASIC_HP_POTION: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Basic HP Potion",
    item_type: ItemType::Consumable(crate::item::enums::ConsumableType::Potion),
    quality: Some(ItemQuality::Normal),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 25,
});

pub static SWORD: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Sword",
    item_type: ItemType::Equipment(EquipmentType::Weapon),
    quality: None,
    stats: StatSheet::new().with(StatType::Attack, 10),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 15,
});

pub static BRONZE_PICKAXE: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Bronze Pickaxe",
    item_type: ItemType::Equipment(EquipmentType::Tool(ToolKind::Pickaxe)),
    quality: None,
    stats: StatSheet::new()
        .with(StatType::Attack, 10)
        .with(StatType::Mining, 10),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 50,
});

pub static DAGGER: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Dagger",
    item_type: ItemType::Equipment(EquipmentType::Weapon),
    quality: None,
    stats: StatSheet::new().with(StatType::Attack, 6),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 10,
});

pub static BASIC_SHIELD: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Basic Shield",
    item_type: ItemType::Equipment(EquipmentType::Shield),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 4),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 15,
});

pub static GOLD_RING: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Midas' Touch",
    item_type: ItemType::Equipment(EquipmentType::Ring),
    quality: None,
    stats: StatSheet::new().with(StatType::GoldFind, 10),
    max_upgrades: 7,
    max_stack_quantity: 1,
    gold_value: 50,
});

pub static QUALITY_UPGRADE_STONE: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Magic Rock",
    item_type: ItemType::Material(MaterialType::UpgradeStone),
    quality: Some(ItemQuality::Mythic),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 500,
});

pub static COAL: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Coal",
    item_type: ItemType::Material(MaterialType::Ore),
    quality: Some(ItemQuality::Normal),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 4,
});


pub static SLIMEGEL: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Slime Gel",
    item_type: ItemType::Material(MaterialType::CraftingMaterial),
    quality: Some(ItemQuality::Normal),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 4,
});
pub static COWHIDE: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Cowhide",
    item_type: ItemType::Material(MaterialType::CraftingMaterial),
    quality: Some(ItemQuality::Normal),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 4,
});
pub static COPPER_ORE: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Copper Ore",
    item_type: ItemType::Material(MaterialType::Ore),
    quality: Some(ItemQuality::Normal),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 5,
});

pub static TIN_ORE: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Tin Ore",
    item_type: ItemType::Material(MaterialType::Ore),
    quality: Some(ItemQuality::Normal),
    stats: StatSheet::new(),
    max_upgrades: 0,
    max_stack_quantity: 99,
    gold_value: 5,
});

// ─────────────────────────────────────────────────────────────────────────────
// Copper Armor (same stats as Tin)
// ─────────────────────────────────────────────────────────────────────────────

pub static COPPER_HELMET: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Copper Helmet",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Head)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 36),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 180,
});

pub static COPPER_CHESTPLATE: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Copper Chestplate",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Chest)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 60),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 300,
});

pub static COPPER_GAUNTLETS: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Copper Gauntlets",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Hands)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 24),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 120,
});

pub static COPPER_GREAVES: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Copper Greaves",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Feet)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 30),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 150,
});

pub static COPPER_LEGGINGS: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Copper Leggings",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Legs)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 54),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 270,
});

// ─────────────────────────────────────────────────────────────────────────────
// Tin Armor (same stats as Copper)
// ─────────────────────────────────────────────────────────────────────────────

pub static TIN_HELMET: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Tin Helmet",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Head)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 36),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 180,
});

pub static TIN_CHESTPLATE: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Tin Chestplate",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Chest)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 60),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 300,
});

pub static TIN_GAUNTLETS: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Tin Gauntlets",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Hands)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 24),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 120,
});

pub static TIN_GREAVES: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Tin Greaves",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Feet)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 30),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 150,
});

pub static TIN_LEGGINGS: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Tin Leggings",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Legs)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 54),
    max_upgrades: 5,
    max_stack_quantity: 1,
    gold_value: 270,
});

// ─────────────────────────────────────────────────────────────────────────────
// Bronze Armor (better stats than Copper/Tin)
// ─────────────────────────────────────────────────────────────────────────────

pub static BRONZE_HELMET: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Bronze Helmet",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Head)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 48),
    max_upgrades: 7,
    max_stack_quantity: 1,
    gold_value: 270,
});

pub static BRONZE_CHESTPLATE: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Bronze Chestplate",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Chest)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 80),
    max_upgrades: 7,
    max_stack_quantity: 1,
    gold_value: 450,
});

pub static BRONZE_GAUNTLETS: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Bronze Gauntlets",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Hands)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 32),
    max_upgrades: 7,
    max_stack_quantity: 1,
    gold_value: 180,
});

pub static BRONZE_GREAVES: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Bronze Greaves",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Feet)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 40),
    max_upgrades: 7,
    max_stack_quantity: 1,
    gold_value: 225,
});

pub static BRONZE_LEGGINGS: Lazy<ItemSpec> = Lazy::new(|| ItemSpec {
    name: "Bronze Leggings",
    item_type: ItemType::Equipment(EquipmentType::Armor(EquipmentSlot::Legs)),
    quality: None,
    stats: StatSheet::new().with(StatType::Defense, 72),
    max_upgrades: 7,
    max_stack_quantity: 1,
    gold_value: 405,
});
