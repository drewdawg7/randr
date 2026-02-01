use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct TileIndex {
    pub(crate) solid: HashSet<(u32, u32)>,
    pub(crate) doors: HashSet<(u32, u32)>,
}

impl TileIndex {
    pub fn is_solid(&self, x: u32, y: u32) -> bool {
        self.solid.contains(&(x, y))
    }

    pub fn is_door(&self, x: u32, y: u32) -> bool {
        self.doors.contains(&(x, y))
    }

    pub fn is_walkable(&self, x: u32, y: u32) -> bool {
        !self.is_solid(x, y)
    }
}
