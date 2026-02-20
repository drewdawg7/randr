use std::collections::HashMap;
use std::sync::OnceLock;

use super::definitions::{MobId, MobSpec, MobSpriteData};

const MOBS_DIR: &str = "assets/data/mobs";

static MOB_SPECS: OnceLock<HashMap<MobId, MobSpec>> = OnceLock::new();

fn load_from_filesystem() -> HashMap<MobId, MobSpec> {
    let mut specs = HashMap::new();
    for dir_entry in std::fs::read_dir(MOBS_DIR)
        .unwrap_or_else(|e| panic!("Failed to read {MOBS_DIR}: {e}"))
    {
        let path = dir_entry.expect("bad dir entry").path();
        let Some(ext) = path.extension() else { continue };
        if ext != "ron" {
            continue;
        }

        let contents = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
        let spec: MobSpec = ron::from_str(&contents)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", path.display()));
        specs.insert(spec.id, spec);
    }

    for id in MobId::ALL {
        assert!(specs.contains_key(id), "Missing RON file for {id:?}");
    }

    specs
}

pub fn populate(specs: HashMap<MobId, MobSpec>) {
    MOB_SPECS.set(specs).ok();
}

pub fn init() {
    MOB_SPECS.get_or_init(load_from_filesystem);
}

pub fn get_spec(id: MobId) -> &'static MobSpec {
    MOB_SPECS
        .get_or_init(load_from_filesystem)
        .get(&id)
        .unwrap_or_else(|| panic!("No spec for {id:?}"))
}

pub fn get_sprite(id: MobId) -> &'static MobSpriteData {
    &get_spec(id).sprite
}

pub fn specs_loaded() -> bool {
    MOB_SPECS.get().is_some()
}
