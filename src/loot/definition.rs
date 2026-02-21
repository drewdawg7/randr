use bon::Builder;
use rand::Rng;
use serde::Deserialize;

use crate::data::StatRange;
use crate::item::{Item, ItemId, ItemRegistry};
use crate::loot::enums::LootError;

#[derive(Debug, Clone)]
pub struct LootDrop {
    pub item: Item,
    pub quantity: i32,
}

#[derive(Clone, Deserialize)]
pub struct LootItem {
    #[serde(rename = "item")]
    item_kind: ItemId,
    numerator: i32,
    denominator: i32,
    quantity: StatRange,
}

#[derive(Default, Debug, Clone, Builder, Deserialize)]
#[serde(from = "Vec<LootItem>")]
pub struct LootTable {
    #[builder(field)]
    loot: Vec<LootItem>,
}

impl From<Vec<LootItem>> for LootTable {
    fn from(loot: Vec<LootItem>) -> Self {
        Self { loot }
    }
}

use loot_table_builder::State;

impl<S: State> LootTableBuilder<S> {
    pub fn with(mut self, item: ItemId, numerator: i32, denominator: i32, quantity: StatRange) -> Self {
        if let Ok(loot_item) = LootItem::new(item, numerator, denominator, quantity) {
            let already_exists = self.loot.iter().any(|i| i.item_kind == item);
            if !already_exists {
                self.loot.push(loot_item);
            }
        }
        self
    }
}

impl LootTable {
    pub fn new() -> LootTableBuilder {
        Self::builder()
    }

    pub fn add_loot_item(&mut self, item: LootItem) -> Result<usize, LootError> {
        if self.check_item_kind(&item.item_kind) {
            return Err(LootError::ItemAlreadyInTable);
        };
        self.loot.push(item);
        Ok(self.loot.len() - 1)
    }

    pub fn get_loot_item_from_kind(&self, kind: &ItemId) -> Option<&LootItem> {
        self.loot.iter().find(|i| i.item_kind == *kind)
    }

    pub fn check_item_kind(&self, kind: &ItemId) -> bool {
        self.get_loot_item_from_kind(kind).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &LootItem> {
        self.loot.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.loot.is_empty()
    }

    pub fn ore_proportions(&self) -> impl Iterator<Item = (ItemId, f32)> + '_ {
        self.loot.iter().map(|item| {
            let chance = item.numerator as f32 / item.denominator as f32;
            (item.item_kind, chance)
        })
    }

    pub fn roll_drops(&self, magic_find: i32, registry: &ItemRegistry) -> Vec<LootDrop> {
        self.roll_drops_with_spawner(magic_find, |id| Some(registry.spawn(id)))
    }

    pub fn roll_drops_with_spawner<F>(&self, magic_find: i32, spawn_item: F) -> Vec<LootDrop>
    where
        F: Fn(ItemId) -> Option<Item>,
    {
        let mut rng = rand::thread_rng();
        let mut drops = Vec::new();

        let bonus_rolls = Self::calculate_bonus_rolls(&mut rng, magic_find);
        let total_rolls = 1 + bonus_rolls;

        for loot_item in &self.loot {
            let mut best_drop: Option<LootDrop> = None;

            for _ in 0..total_rolls {
                let roll = rng.gen_range(1..=loot_item.denominator);
                if roll <= loot_item.numerator {
                    if let Some(item) = spawn_item(loot_item.item_kind) {
                        let quantity = rng.gen_range(loot_item.quantity.start()..=loot_item.quantity.end());
                        let drop = LootDrop { item, quantity };

                        best_drop = Some(match best_drop {
                            None => drop,
                            Some(existing) => Self::pick_better_drop(existing, drop),
                        });
                    }
                }
            }

            if let Some(drop) = best_drop {
                drops.push(drop);
            }
        }

        drops
    }

    fn calculate_bonus_rolls<R: Rng>(rng: &mut R, magic_find: i32) -> i32 {
        if magic_find <= 0 {
            return 0;
        }

        let guaranteed_rolls = magic_find / 100;
        let chance_for_extra = magic_find % 100;

        let extra_roll = if chance_for_extra > 0 {
            let roll = rng.gen_range(1..=100);
            if roll <= chance_for_extra { 1 } else { 0 }
        } else {
            0
        };

        guaranteed_rolls + extra_roll
    }

    fn pick_better_drop(a: LootDrop, b: LootDrop) -> LootDrop {
        if a.item.item_type.is_equipment() {
            if b.item.quality > a.item.quality {
                b
            } else {
                a
            }
        } else {
            if b.quantity > a.quantity {
                b
            } else {
                a
            }
        }
    }
}

impl LootItem {
    pub fn new(item: ItemId, numerator: i32, denominator: i32, quantity: StatRange) -> Result<Self, LootError> {
        if denominator == 0 || denominator < numerator {
            return Err(LootError::InvalidDivision);
        };

        Ok(Self {
            item_kind: item,
            quantity,
            numerator,
            denominator,
        })
    }

    pub fn item_id(&self) -> ItemId {
        self.item_kind
    }

    pub fn drop_chance_percent(&self) -> f32 {
        (self.numerator as f32 / self.denominator as f32) * 100.0
    }

    pub fn quantity_range(&self) -> StatRange {
        self.quantity
    }
}

impl std::fmt::Debug for LootItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LootItem")
            .field("item_kind", &self.item_kind)
            .field("chance", &format!("{}/{}", self.numerator, self.denominator))
            .finish()
    }
}
