use std::ops::RangeInclusive;

use rand::Rng;

use crate::{item::{Item, ItemId}, loot::enums::LootError};

/// Represents a single loot drop with a spawned item instance and quantity
#[derive(Debug, Clone)]
pub struct LootDrop {
    pub item: Item,
    pub quantity: i32,
}

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

    /// Roll drops with Magic Find bonus and custom spawn function.
    ///
    /// This is the core implementation that allows dependency injection for testing.
    ///
    /// Magic Find grants bonus roll attempts:
    /// - 20 MF = 20% chance for 1 bonus roll
    /// - 120 MF = 100% for 1 bonus roll + 20% for 2nd
    ///
    /// Each loot item is rolled (1 + bonus_rolls) times.
    /// If ANY roll succeeds, the item drops.
    /// For equipment: keeps highest quality roll.
    /// For other items: keeps highest quantity roll.
    pub fn roll_drops_with_spawner<F>(&self, magic_find: i32, spawn_item: F) -> Vec<LootDrop>
    where
        F: Fn(ItemId) -> Option<Item>,
    {
        let mut rng = rand::thread_rng();
        let mut drops = Vec::new();

        // Calculate bonus rolls from magic find
        let bonus_rolls = Self::calculate_bonus_rolls(&mut rng, magic_find);
        let total_rolls = 1 + bonus_rolls;

        for loot_item in &self.loot {
            let mut best_drop: Option<LootDrop> = None;

            for _ in 0..total_rolls {
                let roll = rng.gen_range(1..=loot_item.denominator);
                if roll <= loot_item.numerator {
                    if let Some(item) = spawn_item(loot_item.item_kind) {
                        let quantity = rng.gen_range(loot_item.quantity.clone());
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

    /// Calculate number of bonus rolls from magic find value.
    /// Each 100 MF = 1 guaranteed bonus roll.
    /// Remainder gives percentage chance for one additional roll.
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

    /// Pick the better of two drops.
    /// For equipment: higher quality wins.
    /// For other items: higher quantity wins.
    fn pick_better_drop(a: LootDrop, b: LootDrop) -> LootDrop {
        if a.item.item_type.is_equipment() {
            // Equipment: compare quality
            if b.item.quality > a.item.quality {
                b
            } else {
                a
            }
        } else {
            // Non-equipment: compare quantity
            if b.quantity > a.quantity {
                b
            } else {
                a
            }
        }
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
