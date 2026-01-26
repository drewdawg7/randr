//! A floor layout using clustered variant strategy for organic tile appearance.

use crate::dungeon::{
    ClusteredVariant, DungeonLayout, LayoutBuilder, SpawnTable, VariantStrategyKind,
};
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
        .variant_strategy(VariantStrategyKind::Clustered(ClusteredVariant::new(
            3,      // 3x3 tile clusters
            54321,  // seed
            5,      // 5 variants (0-4)
        )))
        .entrance(w / 2, 1)
        .door(w / 2, 0)
        .torches(2..=4)
        .spawn(
            SpawnTable::new()
                .mob(MobId::Goblin, 1)
                .mob(MobId::Slime, 1)
                .mob_count(3..=4)
                .chest(1..=2)
                .stairs(1..=1),
        )
        .build()
}
