use crate::dungeon::{DungeonLayout, LayoutBuilder, SpawnTable};
use crate::mob::MobId;
use crate::ui::DUNGEON_SCALE;

pub fn create() -> DungeonLayout {
    // Original layout dimensions (at scale 1.0)
    const ORIGINAL_W: usize = 40;
    const ORIGINAL_H: usize = 21;

    // Scale dimensions based on DUNGEON_SCALE
    let w = (ORIGINAL_W as f32 / DUNGEON_SCALE) as usize;
    let h = (ORIGINAL_H as f32 / DUNGEON_SCALE) as usize;

    LayoutBuilder::new(w, h)
        .entrance(w / 2, h - 2) // Player spawn above exit
        .exit(w / 2, h - 1) // Exit at bottom center
        .spawn(
            SpawnTable::new()
                .mob(MobId::Goblin, 1)
                .mob(MobId::Slime, 1)
                .mob_count(2..=2)
                .chest(1..=1),
        )
        .build()
}
