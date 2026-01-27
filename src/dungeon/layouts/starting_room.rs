use crate::dungeon::{DungeonLayout, LayoutBuilder};
use crate::ui::DUNGEON_SCALE;

pub fn create() -> DungeonLayout {
    const ORIGINAL_W: usize = 40;
    const ORIGINAL_H: usize = 21;

    let w = (ORIGINAL_W as f32 / DUNGEON_SCALE) as usize;
    let h = (ORIGINAL_H as f32 / DUNGEON_SCALE) as usize;

    LayoutBuilder::new(w, h)
        .entrance(w / 2, 1)
        .door(w / 2, 0)
        .torches(2..=4)
        .build()
}
