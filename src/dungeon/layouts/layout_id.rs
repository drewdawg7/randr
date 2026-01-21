use crate::dungeon::DungeonLayout;

use super::starting_room;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutId {
    StartingRoom,
}

impl LayoutId {
    pub fn layout(&self) -> DungeonLayout {
        match self {
            LayoutId::StartingRoom => starting_room::create(),
        }
    }

    pub const ALL: &'static [LayoutId] = &[LayoutId::StartingRoom];
}
