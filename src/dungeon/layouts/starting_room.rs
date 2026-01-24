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
        .entrance(w / 2, 1) // Player spawn in front of door
        .door(w / 2, 0) // Decorative door on back wall
        .torches(2..=4)
        .spawn(
            SpawnTable::new()
                .mob(MobId::Goblin, 1)
                .mob(MobId::Slime, 1)
                .mob_count(2..=2)
                .guaranteed_mob(MobId::BlackDragon, 1)
                .npc(MobId::Merchant, 1..=1)
                .chest(1..=1)
                .stairs(1..=1)
                .rock(2..=4),
        )
        .build()
}
