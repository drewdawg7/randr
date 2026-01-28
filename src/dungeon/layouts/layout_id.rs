use crate::dungeon::DungeonLayout;

use super::{cave_floor, clustered_floor, dungeon_floor, home_layout, starting_room, tmx_cave_floor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutId {
    StartingRoom,
    ClusteredFloor,
    HomeLayout,
    DungeonFloorWithStairs,
    DungeonFloorFinal,
    CaveFloorWithStairs,
    CaveFloorFinal,
    /// TMX-based cave floor loaded from Tiled map file.
    TmxCaveFloor,
}

impl LayoutId {
    pub fn layout(&self) -> DungeonLayout {
        match self {
            LayoutId::StartingRoom => starting_room::create(),
            LayoutId::ClusteredFloor => clustered_floor::create(),
            LayoutId::HomeLayout => home_layout::create(),
            LayoutId::DungeonFloorWithStairs => dungeon_floor::create_with_stairs(),
            LayoutId::DungeonFloorFinal => dungeon_floor::create_final(),
            LayoutId::CaveFloorWithStairs => cave_floor::create_with_stairs(),
            LayoutId::CaveFloorFinal => cave_floor::create_final(),
            LayoutId::TmxCaveFloor => tmx_cave_floor::create(),
        }
    }

    pub const ALL: &'static [LayoutId] = &[
        LayoutId::StartingRoom,
        LayoutId::ClusteredFloor,
        LayoutId::HomeLayout,
        LayoutId::DungeonFloorWithStairs,
        LayoutId::DungeonFloorFinal,
        LayoutId::CaveFloorWithStairs,
        LayoutId::CaveFloorFinal,
        LayoutId::TmxCaveFloor,
    ];
}
