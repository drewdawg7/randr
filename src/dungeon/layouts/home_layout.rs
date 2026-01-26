use crate::dungeon::{DungeonLayout, LayoutBuilder, SpawnTable};
use crate::ui::DUNGEON_SCALE;

pub fn create() -> DungeonLayout {
    // Smaller room for home base
    const ORIGINAL_W: usize = 20;
    const ORIGINAL_H: usize = 15;

    let w = (ORIGINAL_W as f32 / DUNGEON_SCALE) as usize;
    let h = (ORIGINAL_H as f32 / DUNGEON_SCALE) as usize;

    LayoutBuilder::new(w, h)
        .entrance(w / 2, 1) // GateFloor in front of door, player spawns here
        .door(w / 2, 0) // Door tile on back wall (collision handled in movement system)
        .torches(1..=2)
        .spawn(SpawnTable::empty())
        .build()
}
