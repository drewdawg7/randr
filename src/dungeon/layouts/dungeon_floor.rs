use crate::dungeon::{DungeonLayout, LayoutBuilder};
use crate::ui::DUNGEON_SCALE;

pub fn create_with_stairs() -> DungeonLayout {
    const ORIGINAL_W: usize = 40;
    const ORIGINAL_H: usize = 21;

    let w = (ORIGINAL_W as f32 / DUNGEON_SCALE) as usize;
    let h = (ORIGINAL_H as f32 / DUNGEON_SCALE) as usize;

    LayoutBuilder::new(w, h)
        .spawn_point(w / 2, h / 2)
        .torches(2..=4)
        .build()
}

pub fn create_final() -> DungeonLayout {
    const ORIGINAL_W: usize = 40;
    const ORIGINAL_H: usize = 21;

    let w = (ORIGINAL_W as f32 / DUNGEON_SCALE) as usize;
    let h = (ORIGINAL_H as f32 / DUNGEON_SCALE) as usize;

    LayoutBuilder::new(w, h)
        .spawn_point(w / 2, h / 2)
        .torches(2..=4)
        .build()
}
