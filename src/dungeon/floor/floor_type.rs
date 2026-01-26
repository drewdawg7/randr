use crate::dungeon::layouts::LayoutId;
use crate::dungeon::spawn::SpawnTable;
use crate::mob::MobId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloorType {
    BasicDungeonFloor,
}

impl FloorType {
    pub fn spawn_table(&self, is_final: bool) -> SpawnTable {
        match self {
            FloorType::BasicDungeonFloor => {
                let base = SpawnTable::new()
                    .mob(MobId::Goblin, 5)
                    .mob(MobId::Slime, 3)
                    .mob_count(3..=4)
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
        }
    }
}
