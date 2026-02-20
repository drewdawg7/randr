use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;

use super::definitions::{ItemId, ItemSpec};

const ITEMS_DIR: &str = "assets/data/items";

#[derive(Deserialize)]
struct ItemEntry {
    id: ItemId,
    #[serde(flatten)]
    spec: ItemSpec,
}

static ITEM_SPECS: OnceLock<HashMap<ItemId, ItemSpec>> = OnceLock::new();

pub fn init() {
    let specs: HashMap<ItemId, ItemSpec> = std::fs::read_dir(ITEMS_DIR)
        .unwrap_or_else(|e| panic!("Failed to read {ITEMS_DIR}: {e}"))
        .filter_map(|entry| {
            let path = entry.expect("bad dir entry").path();
            (path.extension()?.to_str()? == "ron").then(|| {
                let contents = std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
                let item: ItemEntry = ron::from_str(&contents)
                    .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", path.display()));
                (item.id, item.spec)
            })
        })
        .collect();

    for id in ItemId::ALL {
        assert!(specs.contains_key(id), "Missing RON file for {id:?}");
    }

    ITEM_SPECS.set(specs).expect("Item specs already initialized");
}

pub fn get_spec(id: ItemId) -> &'static ItemSpec {
    ITEM_SPECS
        .get()
        .expect("Item specs not initialized")
        .get(&id)
        .unwrap_or_else(|| panic!("No spec for {id:?}"))
}
