use crate::assets::SpriteSheetKey;
use crate::dungeon::layouts::LayoutId;
use crate::dungeon::spawn::SpawnTable;
use crate::mob::MobId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloorType {
    BasicDungeonFloor,
    CaveFloor,
    /// TMX-based cave floor loaded from Tiled map file.
    TmxCaveFloor,
}

impl FloorType {
    /// Returns the tileset key for rendering this floor type
    pub fn tileset_key(&self) -> SpriteSheetKey {
        match self {
            FloorType::BasicDungeonFloor => SpriteSheetKey::DungeonTileset,
            FloorType::CaveFloor | FloorType::TmxCaveFloor => SpriteSheetKey::CaveTileset,
        }
    }

    /// Returns the tile size multiplier for this floor type.
    /// Cave tiles are 32x32 (2x dungeon's 16x16), so they need 2x scaling.
    pub fn tile_scale(&self) -> f32 {
        match self {
            FloorType::BasicDungeonFloor => 1.0,
            FloorType::CaveFloor | FloorType::TmxCaveFloor => 2.0,
        }
    }

    pub fn spawn_table(&self, is_final: bool) -> SpawnTable {
        match self {
            FloorType::BasicDungeonFloor | FloorType::CaveFloor | FloorType::TmxCaveFloor => {
                let base = SpawnTable::new()
                    .mob(MobId::Goblin, 5)
                    .mob(MobId::Slime, 3)
                    .mob_count(3..=4)
                    .guaranteed_mob(MobId::DwarfDefender, 1)
                    .guaranteed_mob(MobId::DwarfWarrior, 1)
                    .guaranteed_mob(MobId::DwarfMiner, 1)
                    .guaranteed_mob(MobId::DwarfKing, 1)
                    .rock(0..=4)
                    .forge_chance(0.33)
                    .anvil_chance(0.33)
                    .npc_chance(MobId::Merchant, 0.33);

                if is_final {
                    base
                } else {
                    base.stairs(1..=1)
                }
            }
        }
    }

    pub fn layout_id(&self, is_final: bool) -> LayoutId {
        match self {
            FloorType::BasicDungeonFloor => {
                if is_final {
                    LayoutId::DungeonFloorFinal
                } else {
                    LayoutId::DungeonFloorWithStairs
                }
            }
            FloorType::CaveFloor => {
                if is_final {
                    LayoutId::CaveFloorFinal
                } else {
                    LayoutId::CaveFloorWithStairs
                }
            }
            // TMX floors use the same layout regardless of final status
            // (the TMX file itself determines the layout)
            FloorType::TmxCaveFloor => LayoutId::TmxCaveFloor,
        }
    }
}
