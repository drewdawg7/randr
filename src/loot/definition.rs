use rand::Rng;

use crate::{item::ItemId, loot::enums::LootError};

pub struct LootItem {
    item_kind: ItemId,
    numerator: i32,
    denominator: i32,
}

#[derive(Default, Debug, Clone)]
pub struct LootTable {
    loot: Vec<LootItem>,
}

impl LootTable {
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

    /// Roll each item independently, return all items that dropped.
    pub fn roll_drops(&self) -> Vec<ItemId> {
        let mut rng = rand::thread_rng();
        self.loot
            .iter()
            .filter(|item| {
                let roll = rng.gen_range(1..=item.denominator);
                roll <= item.numerator
            })
            .map(|item| item.item_kind)
            .collect()
    }
}

impl LootItem {
    pub fn new(item: ItemId, numerator: i32, denominator: i32) -> Result<Self, LootError> {
        if denominator == 0 || denominator < numerator {
            return Err(LootError::InvalidDivision);
        };

        Ok(Self {
            item_kind: item,
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
