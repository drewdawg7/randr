use std::ops::RangeInclusive;

use rand::Rng;

use crate::{item::ItemId, loot::enums::LootError};

pub struct LootItem {
    item_kind: ItemId,
    numerator: i32,
    denominator: i32,
    quantity: RangeInclusive<i32>,
}

#[derive(Default, Debug, Clone)]
pub struct LootTable {
    loot: Vec<LootItem>,
}

impl LootTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder method to add a loot item. Silently ignores invalid items.
    pub fn with(mut self, item: ItemId, numerator: i32, denominator: i32, quantity: RangeInclusive<i32>) -> Self {
        if let Ok(loot_item) = LootItem::new(item, numerator, denominator, quantity) {
            let _ = self.add_loot_item(loot_item);
        }
        self
    }

    pub fn add_loot_item(&mut self, item: LootItem) -> Result<(), LootError> {
        if self.check_item_kind(&item.item_kind) {
            return Err(LootError::ItemAlreadyInTable);
        };
        self.loot.push(item);
        Ok(())
    }

    pub fn get_loot_item_from_kind(&self, kind: &ItemId) -> Option<&LootItem> {
        self.loot.iter().find(|i| i.item_kind == *kind)
    }

    pub fn check_item_kind(&self, kind: &ItemId) -> bool {
        self.get_loot_item_from_kind(kind).is_some()
    }

    /// Returns iterator over (ItemId, drop_chance as f32 0.0-1.0)
    pub fn ore_proportions(&self) -> impl Iterator<Item = (ItemId, f32)> + '_ {
        self.loot.iter().map(|item| {
            let chance = item.numerator as f32 / item.denominator as f32;
            (item.item_kind, chance)
        })
    }

    /// Roll each item independently, return all items that dropped with their quantity.
    pub fn roll_drops(&self) -> Vec<(ItemId, i32)> {
        let mut rng = rand::thread_rng();
        let mut drops = Vec::new();

        for item in &self.loot {
            let roll = rng.gen_range(1..=item.denominator);
            if roll <= item.numerator {
                let quantity = rng.gen_range(item.quantity.clone());
                drops.push((item.item_kind, quantity));
            }
        }

        drops
    }
}

impl LootItem {
    pub fn new(item: ItemId, numerator: i32, denominator: i32, quantity: RangeInclusive<i32>) -> Result<Self, LootError> {
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
}

impl Clone for LootItem {
    fn clone(&self) -> Self {
        Self {
            item_kind: self.item_kind,
            numerator: self.numerator,
            denominator: self.denominator,
            quantity: self.quantity.clone(),
        }
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
