use once_cell::sync::Lazy;

use crate::{
    item::enums::{EquipmentType, ItemQuality, ItemType, MaterialType, ToolKind},
    stats::{StatSheet, StatType},
};

use super::definition::ItemSpec;



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
