use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::assets::SpriteSheetKey;

use super::definitions::{ItemId, ItemSpec};
use super::enums::{ItemQuality, ItemType};
use crate::stats::StatSheet;

const ITEMS_DIR: &str = "assets/data/items";

#[derive(Deserialize)]
struct ItemEntry {
    id: ItemId,
    name: String,
    item_type: ItemType,
    quality: Option<ItemQuality>,
    stats: StatSheet,
    max_upgrades: i32,
    max_stack_quantity: u32,
    gold_value: i32,
    sprite_name: String,
    #[serde(default)]
    sprite_sheet: Option<SpriteSheetKey>,
}

impl ItemEntry {
    fn into_parts(self) -> (ItemId, ItemSpec) {
        (
            self.id,
            ItemSpec {
                id: self.id,
                name: self.name,
                item_type: self.item_type,
                quality: self.quality,
                stats: self.stats,
                max_upgrades: self.max_upgrades,
                max_stack_quantity: self.max_stack_quantity,
                gold_value: self.gold_value,
                sprite_name: self.sprite_name,
                sprite_sheet: self.sprite_sheet,
            },
        )
    }
}

static ITEM_SPECS: OnceLock<HashMap<ItemId, ItemSpec>> = OnceLock::new();

fn load_item_specs() -> HashMap<ItemId, ItemSpec> {
    let specs: HashMap<ItemId, ItemSpec> = std::fs::read_dir(ITEMS_DIR)
        .unwrap_or_else(|e| panic!("Failed to read {ITEMS_DIR}: {e}"))
        .filter_map(|entry| {
            let path = entry.expect("bad dir entry").path();
            (path.extension()?.to_str()? == "ron").then(|| {
                let contents = std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
                let item: ItemEntry = ron::from_str(&contents)
                    .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", path.display()));
                item.into_parts()
            })
        })
        .collect();

    for id in ItemId::ALL {
        assert!(specs.contains_key(id), "Missing RON file for {id:?}");
    }

    specs
}

pub fn init() {
    ITEM_SPECS.get_or_init(load_item_specs);
}

pub fn get_spec(id: ItemId) -> &'static ItemSpec {
    ITEM_SPECS
        .get_or_init(load_item_specs)
        .get(&id)
        .unwrap_or_else(|| panic!("No spec for {id:?}"))
}
