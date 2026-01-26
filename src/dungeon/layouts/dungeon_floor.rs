use crate::dungeon::{DungeonLayout, LayoutBuilder, SpawnTable};
use crate::mob::MobId;
use crate::ui::DUNGEON_SCALE;

/// Standard dungeon floor with configurable spawns.
/// Used by MainDungeon floors where spawn tables are defined per-floor.
pub fn create_with_stairs() -> DungeonLayout {
    const ORIGINAL_W: usize = 40;
    const ORIGINAL_H: usize = 21;

    let w = (ORIGINAL_W as f32 / DUNGEON_SCALE) as usize;
    let h = (ORIGINAL_H as f32 / DUNGEON_SCALE) as usize;

    LayoutBuilder::new(w, h)
        .spawn_point(w / 2, h / 2)
        .torches(2..=4)
        .spawn(
            SpawnTable::new()
                .mob(MobId::Goblin, 5)
                .mob(MobId::Slime, 3)
                .mob_count(3..=4)
                .stairs(1..=1),
        )
        .build()
}

/// Final dungeon floor without stairs (boss floor).
pub fn create_final() -> DungeonLayout {
    const ORIGINAL_W: usize = 40;
    const ORIGINAL_H: usize = 21;

    let w = (ORIGINAL_W as f32 / DUNGEON_SCALE) as usize;
    let h = (ORIGINAL_H as f32 / DUNGEON_SCALE) as usize;

    LayoutBuilder::new(w, h)
        .spawn_point(w / 2, h / 2)
        .torches(2..=4)
        .spawn(
            SpawnTable::new()
                .mob(MobId::Goblin, 5)
                .mob(MobId::Slime, 3)
                .mob_count(3..=4),
        )
        .build()
}
