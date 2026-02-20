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

    pub fn spec(&self) -> &'static ItemSpec {
        super::data::get_spec(*self)
    }

    pub fn sprite_name(&self) -> &str {
        self.spec().sprite_name.as_str()
    }

    pub fn sprite_sheet_key(&self) -> SpriteSheetKey {
        self.spec().sprite_sheet.unwrap_or(SpriteSheetKey::IconItems)
    }
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

impl ItemSpec {
    fn spawn(&self, id: ItemId) -> Item {
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
            id: self.id,
            name: self.name.clone(),
            item_type: self.item_type,
            quality: self.quality,
            max_upgrades: self.max_upgrades,
            max_stack_quantity: self.max_stack_quantity,
            stats: scaled_stats,
            gold_value: (self.gold_value as f32 * multiplier).round() as i32,
            sprite_name: self.sprite_name.clone(),
            sprite_sheet: self.sprite_sheet,
        }
    }

    pub fn with_name(&self, name: impl Into<String>) -> ItemSpec {
        ItemSpec {
            id: self.id,
            name: name.into(),
            item_type: self.item_type,
            quality: self.quality,
            max_upgrades: self.max_upgrades,
            max_stack_quantity: self.max_stack_quantity,
            stats: self.stats.clone(),
            gold_value: self.gold_value,
            sprite_name: self.sprite_name.clone(),
            sprite_sheet: self.sprite_sheet,
        }
    }

    pub fn with_quality(&self, quality: ItemQuality) -> ItemSpec {
        ItemSpec {
            id: self.id,
            name: self.name.clone(),
            item_type: self.item_type,
            quality: Some(quality),
            max_upgrades: self.max_upgrades,
            max_stack_quantity: self.max_stack_quantity,
            stats: self.stats.clone(),
            gold_value: self.gold_value,
            sprite_name: self.sprite_name.clone(),
            sprite_sheet: self.sprite_sheet,
        }
    }
}

pub struct ItemSpawner {
    id: ItemId,
    spec: ItemSpec,
}

impl ItemSpawner {
    pub fn spawn(self) -> Item {
        self.spec.spawn(self.id)
    }

    pub fn with_multiplier(mut self, multiplier: f32) -> Self {
        self.spec = self.spec.with_multiplier(multiplier);
        self
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.spec = self.spec.with_name(name);
        self
    }

    pub fn with_quality(mut self, quality: ItemQuality) -> Self {
        self.spec = self.spec.with_quality(quality);
        self
    }
}

impl ItemId {
    pub fn spawn(&self) -> Item {
        self.spec().spawn(*self)
    }

    pub fn with_multiplier(&self, multiplier: f32) -> ItemSpawner {
        ItemSpawner {
            id: *self,
            spec: self.spec().with_multiplier(multiplier),
        }
    }

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

    pub fn spawn_with_quality_bonus(&self, blacksmith_level: u32) -> Item {
        let quality = ItemQuality::roll_with_bonus(blacksmith_level);
        self.with_quality(quality).spawn()
    }
}
