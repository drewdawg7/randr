use crate::dungeon::DungeonLayout;

use super::{clustered_floor, dungeon_floor, home_layout, starting_room};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutId {
    StartingRoom,
    ClusteredFloor,
    HomeLayout,
    DungeonFloorWithStairs,
    DungeonFloorFinal,
}

impl LayoutId {
    pub fn layout(&self) -> DungeonLayout {
        match self {
            LayoutId::StartingRoom => starting_room::create(),
            LayoutId::ClusteredFloor => clustered_floor::create(),
            LayoutId::HomeLayout => home_layout::create(),
            LayoutId::DungeonFloorWithStairs => dungeon_floor::create_with_stairs(),
            LayoutId::DungeonFloorFinal => dungeon_floor::create_final(),
        }
    }

    pub const ALL: &'static [LayoutId] = &[
        LayoutId::StartingRoom,
        LayoutId::ClusteredFloor,
        LayoutId::HomeLayout,
        LayoutId::DungeonFloorWithStairs,
        LayoutId::DungeonFloorFinal,
    ];
}
