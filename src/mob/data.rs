use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::data::StatRange;
use crate::dungeon::EntitySize;
use crate::item::ItemId;
use crate::loot::LootTable;

use super::definitions::{MobId, MobQuality, MobSpec};

const MOBS_DIR: &str = "assets/data/mobs";

#[derive(Deserialize)]
struct LootEntryRon {
    item: ItemId,
    numerator: i32,
    denominator: i32,
    quantity: StatRange,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MobSpriteData {
    pub aseprite_path: String,
    pub idle_tag: String,
    pub hurt_tag: Option<String>,
    pub death_tag: Option<String>,
    pub frame_size: (u32, u32),
}

#[derive(Deserialize)]
struct MobEntry {
    id: MobId,
    name: String,
    quality: MobQuality,
    max_health: StatRange,
    attack: StatRange,
    defense: StatRange,
    dropped_gold: StatRange,
    dropped_xp: StatRange,
    #[serde(default)]
    loot: Vec<LootEntryRon>,
    #[serde(default)]
    entity_size: Option<(f32, f32)>,
    sprite: MobSpriteData,
}

impl MobEntry {
    fn into_parts(self) -> (MobId, MobSpec, MobSpriteData) {
        let loot = self
            .loot
            .into_iter()
            .fold(LootTable::new(), |builder, entry| {
                builder.with(entry.item, entry.numerator, entry.denominator, entry.quantity.into())
            })
            .build();

        let entity_size = match self.entity_size {
            Some((w, h)) => EntitySize::new(w, h),
            None => EntitySize::default(),
        };

        (
            self.id,
            MobSpec {
                name: self.name,
                max_health: self.max_health.into(),
                attack: self.attack.into(),
                defense: self.defense.into(),
                dropped_gold: self.dropped_gold.into(),
                dropped_xp: self.dropped_xp.into(),
                quality: self.quality,
                loot,
                entity_size,
            },
            self.sprite,
        )
    }
}

struct MobData {
    specs: HashMap<MobId, MobSpec>,
    sprites: HashMap<MobId, MobSpriteData>,
}

static MOB_DATA: OnceLock<MobData> = OnceLock::new();

fn load_mob_data() -> MobData {
    let mut specs = HashMap::new();
    let mut sprites = HashMap::new();

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
        let entry: MobEntry = ron::from_str(&contents)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", path.display()));
        let (id, spec, sprite) = entry.into_parts();
        specs.insert(id, spec);
        sprites.insert(id, sprite);
    }

    for id in MobId::ALL {
        assert!(specs.contains_key(id), "Missing RON file for {id:?}");
    }

    MobData { specs, sprites }
}

pub fn init() {
    MOB_DATA.get_or_init(load_mob_data);
}

pub fn get_spec(id: MobId) -> &'static MobSpec {
    MOB_DATA
        .get_or_init(load_mob_data)
        .specs
        .get(&id)
        .unwrap_or_else(|| panic!("No spec for {id:?}"))
}

pub fn get_sprite(id: MobId) -> &'static MobSpriteData {
    MOB_DATA
        .get_or_init(load_mob_data)
        .sprites
        .get(&id)
        .unwrap_or_else(|| panic!("No sprite data for {id:?}"))
}

pub fn specs_loaded() -> bool {
    MOB_DATA.get().is_some()
}
