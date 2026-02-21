use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::assets::SpriteSheetKey;
use crate::stats::StatSheet;

pub use super::enums::{
    ConsumableType, EquipmentType, ItemQuality, ItemType, MaterialType, ToolKind,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ItemId {
    Sword,
    Dagger,
    GoldSword,
    IronSword,
    CopperSword,
    BonkStick,
    BasicShield,
    IronHelmet,
    IronChestplate,
    IronGauntlets,
    IronGreaves,
    IronLeggings,
    GoldHelmet,
    GoldChestplate,
    GoldGauntlets,
    GoldGreaves,
    GoldLeggings,
    CopperHelmet,
    CopperChestplate,
    CopperGauntlets,
    CopperGreaves,
    CopperLeggings,
    CopperPickaxe,
    GoldRing,
    ImbaRing,
    Coal,
    CopperOre,
    IronOre,
    GoldOre,
    GoldIngot,
    IronIngot,
    CopperIngot,
    BlueCrystal,
    RedCrystal,
    GreenCrystal,
    WhiteCrystal,
    OrangeCrystal,
    YellowCrystal,
    Cowhide,
    SlimeGel,
    BasicHPPotion,
    QualityUpgradeStone,
}

impl ItemId {
    pub const ALL: &'static [ItemId] = &[
        ItemId::Sword,
        ItemId::Dagger,
        ItemId::GoldSword,
        ItemId::IronSword,
        ItemId::CopperSword,
        ItemId::BonkStick,
        ItemId::BasicShield,
        ItemId::IronHelmet,
        ItemId::IronChestplate,
        ItemId::IronGauntlets,
        ItemId::IronGreaves,
        ItemId::IronLeggings,
        ItemId::GoldHelmet,
        ItemId::GoldChestplate,
        ItemId::GoldGauntlets,
        ItemId::GoldGreaves,
        ItemId::GoldLeggings,
        ItemId::CopperHelmet,
        ItemId::CopperChestplate,
        ItemId::CopperGauntlets,
        ItemId::CopperGreaves,
        ItemId::CopperLeggings,
        ItemId::CopperPickaxe,
        ItemId::GoldRing,
        ItemId::ImbaRing,
        ItemId::Coal,
        ItemId::CopperOre,
        ItemId::IronOre,
        ItemId::GoldOre,
        ItemId::GoldIngot,
        ItemId::IronIngot,
        ItemId::CopperIngot,
        ItemId::BlueCrystal,
        ItemId::RedCrystal,
        ItemId::GreenCrystal,
        ItemId::WhiteCrystal,
        ItemId::OrangeCrystal,
        ItemId::YellowCrystal,
        ItemId::Cowhide,
        ItemId::SlimeGel,
        ItemId::BasicHPPotion,
        ItemId::QualityUpgradeStone,
    ];
}

#[derive(Debug, Clone, Deserialize, Asset, TypePath)]
pub struct ItemSpec {
    pub id: ItemId,
    pub name: String,
    pub item_type: ItemType,
    pub quality: Option<ItemQuality>,
    pub max_upgrades: i32,
    pub max_stack_quantity: u32,
    pub stats: StatSheet,
    pub gold_value: i32,
    pub sprite_name: String,
    #[serde(default)]
    pub sprite_sheet: Option<SpriteSheetKey>,
}

use uuid::Uuid;
use super::definition::Item;
use super::sprite_info::SpriteInfo;

impl ItemSpec {
    pub(super) fn to_item(&self) -> Item {
        let quality = self.quality.unwrap_or_else(ItemQuality::roll);
        let base_stats = self.stats.clone();
        let stats = quality.multiply_stats(&base_stats);

        Item {
            item_uuid: Uuid::new_v4(),
            item_id: self.id,
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
            sprite: SpriteInfo {
                name: self.sprite_name.clone(),
                sheet_key: self.sprite_sheet.unwrap_or(SpriteSheetKey::IconItems),
            },
        }
    }

}
