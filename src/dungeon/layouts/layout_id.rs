use crate::dungeon::DungeonLayout;

use super::{clustered_floor, starting_room};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutId {
    StartingRoom,
    ClusteredFloor,
}

impl LayoutId {
    pub fn layout(&self) -> DungeonLayout {
        match self {
            LayoutId::StartingRoom => starting_room::create(),
            LayoutId::ClusteredFloor => clustered_floor::create(),
        }
    }

    pub const ALL: &'static [LayoutId] = &[LayoutId::StartingRoom, LayoutId::ClusteredFloor];
}
