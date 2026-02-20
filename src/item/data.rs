use std::collections::HashMap;
use std::sync::OnceLock;

use super::definitions::{ItemId, ItemSpec};

const ITEMS_DIR: &str = "assets/data/items";

static ITEM_SPECS: OnceLock<HashMap<ItemId, ItemSpec>> = OnceLock::new();

fn load_from_filesystem() -> HashMap<ItemId, ItemSpec> {
    let mut specs = HashMap::new();
    for dir_entry in std::fs::read_dir(ITEMS_DIR)
        .unwrap_or_else(|e| panic!("Failed to read {ITEMS_DIR}: {e}"))
    {
        let path = dir_entry.expect("bad dir entry").path();
        let Some(ext) = path.extension() else { continue };
        if ext != "ron" {
            continue;
        }

        let contents = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
        let spec: ItemSpec = ron::from_str(&contents)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", path.display()));
        specs.insert(spec.id, spec);
    }

    for id in ItemId::ALL {
        assert!(specs.contains_key(id), "Missing RON file for {id:?}");
    }

    specs
}

pub fn populate(specs: HashMap<ItemId, ItemSpec>) {
    ITEM_SPECS.set(specs).ok();
}

pub fn init() {
    ITEM_SPECS.get_or_init(load_from_filesystem);
}

pub fn get_spec(id: ItemId) -> &'static ItemSpec {
    ITEM_SPECS
        .get_or_init(load_from_filesystem)
        .get(&id)
        .unwrap_or_else(|| panic!("No spec for {id:?}"))
}

pub fn specs_loaded() -> bool {
    ITEM_SPECS.get().is_some()
}
