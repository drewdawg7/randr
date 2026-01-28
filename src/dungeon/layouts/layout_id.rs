use crate::dungeon::DungeonLayout;

use super::{cave_floor, home_floor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutId {
    CaveFloor,
    HomeFloor,
}

impl LayoutId {
    pub fn layout(&self) -> DungeonLayout {
        match self {
            LayoutId::CaveFloor => cave_floor::create(),
            LayoutId::HomeFloor => home_floor::create(),
        }
    }

    pub const ALL: &'static [LayoutId] = &[
        LayoutId::CaveFloor,
        LayoutId::HomeFloor,
    ];
}
