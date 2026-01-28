use crate::dungeon::DungeonLayout;

use super::{cave_floor, clustered_floor, dungeon_floor, starting_room, tmx_cave_floor, tmx_home_floor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutId {
    StartingRoom,
    ClusteredFloor,
    DungeonFloorWithStairs,
    DungeonFloorFinal,
    CaveFloorWithStairs,
    CaveFloorFinal,
    /// TMX-based cave floor loaded from Tiled map file.
    TmxCaveFloor,
    /// TMX-based home floor loaded from Tiled map file.
    TmxHomeFloor,
}

impl LayoutId {
    pub fn layout(&self) -> DungeonLayout {
        match self {
            LayoutId::StartingRoom => starting_room::create(),
            LayoutId::ClusteredFloor => clustered_floor::create(),
            LayoutId::DungeonFloorWithStairs => dungeon_floor::create_with_stairs(),
            LayoutId::DungeonFloorFinal => dungeon_floor::create_final(),
            LayoutId::CaveFloorWithStairs => cave_floor::create_with_stairs(),
            LayoutId::CaveFloorFinal => cave_floor::create_final(),
            LayoutId::TmxCaveFloor => tmx_cave_floor::create(),
            LayoutId::TmxHomeFloor => tmx_home_floor::create(),
        }
    }

    pub const ALL: &'static [LayoutId] = &[
        LayoutId::StartingRoom,
        LayoutId::ClusteredFloor,
        LayoutId::DungeonFloorWithStairs,
        LayoutId::DungeonFloorFinal,
        LayoutId::CaveFloorWithStairs,
        LayoutId::CaveFloorFinal,
        LayoutId::TmxCaveFloor,
        LayoutId::TmxHomeFloor,
    ];
}
