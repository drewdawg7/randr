use crate::assets::SpriteSheetKey;
use crate::dungeon::layouts::LayoutId;
use crate::dungeon::spawn::SpawnTable;
use crate::mob::MobId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloorType {
    CaveFloor,
}

impl FloorType {
    pub fn tileset_key(&self) -> SpriteSheetKey {
        SpriteSheetKey::CaveTileset
    }

    pub fn tile_scale(&self) -> f32 {
        1.0
    }

    pub fn spawn_table(&self, is_final: bool) -> SpawnTable {
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
            base.build()
        } else {
            base.stairs(1..=1).build()
        }
    }

    pub fn layout_id(&self, _is_final: bool) -> LayoutId {
        LayoutId::CaveFloor
    }
}
