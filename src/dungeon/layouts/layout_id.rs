use crate::dungeon::DungeonLayout;

use super::{tmx_cave_floor, tmx_home_floor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutId {
    TmxCaveFloor,
    TmxHomeFloor,
}

impl LayoutId {
    pub fn layout(&self) -> DungeonLayout {
        match self {
            LayoutId::TmxCaveFloor => tmx_cave_floor::create(),
            LayoutId::TmxHomeFloor => tmx_home_floor::create(),
        }
    }

    pub const ALL: &'static [LayoutId] = &[
        LayoutId::TmxCaveFloor,
        LayoutId::TmxHomeFloor,
    ];
}
