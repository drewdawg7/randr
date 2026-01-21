use rand::seq::SliceRandom;
use rand::Rng;

use crate::dungeon::{DungeonEntity, DungeonLayout, LayoutBuilder};
use crate::mob::MobId;

pub fn create() -> DungeonLayout {
    const W: usize = 40;
    const H: usize = 21;

    let mut layout = LayoutBuilder::new(W, H)
        .entrance(W / 2, H - 2) // Player spawn above exit
        .exit(W / 2, H - 1) // Exit at bottom center
        .build();

    // Spawn entities on random floor tiles without overlap
    let mut spawn_points = layout.spawn_points();
    let mut rng = rand::thread_rng();

    // Shuffle spawn points to get random positions
    spawn_points.shuffle(&mut rng);
    let mut spawn_iter = spawn_points.into_iter();

    // Spawn 1 chest
    if let Some((x, y)) = spawn_iter.next() {
        let variant = rng.gen_range(0..4);
        layout.add_entity(x, y, DungeonEntity::Chest { variant });
    }

    // Spawn 1 goblin
    if let Some((x, y)) = spawn_iter.next() {
        layout.add_entity(x, y, DungeonEntity::Mob { mob_id: MobId::Goblin });
    }

    // Spawn 1 slime
    if let Some((x, y)) = spawn_iter.next() {
        layout.add_entity(x, y, DungeonEntity::Mob { mob_id: MobId::Slime });
    }

    layout
}
