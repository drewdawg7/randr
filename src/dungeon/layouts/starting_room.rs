use crate::dungeon::{DungeonLayout, LayoutBuilder, SpawnTable};
use crate::mob::MobId;

pub fn create() -> DungeonLayout {
    const W: usize = 40;
    const H: usize = 21;

    LayoutBuilder::new(W, H)
        .entrance(W / 2, H - 2) // Player spawn above exit
        .exit(W / 2, H - 1) // Exit at bottom center
        .spawn(
            SpawnTable::new()
                .mob(MobId::Goblin, 1)
                .mob(MobId::Slime, 1)
                .mob_count(2..=2)
                .chest(1..=1),
        )
        .build()
}
