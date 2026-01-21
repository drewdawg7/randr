use rand::seq::SliceRandom;
use rand::Rng;

use crate::dungeon::{DungeonEntity, DungeonLayout, Tile, TileType};
use crate::mob::MobId;

pub fn create() -> DungeonLayout {
    const W: usize = 40;
    const H: usize = 21;

    let mut layout = DungeonLayout::new(W, H);

    use TileType::*;

    // Generate tiles procedurally: walls on perimeter, floor inside
    for y in 0..H {
        for x in 0..W {
            let tile_type = if y == 0 || y == H - 1 || x == 0 || x == W - 1 {
                // Perimeter walls, with exit at bottom center
                if y == H - 1 && x == W / 2 {
                    Exit
                } else {
                    Wall
                }
            } else if y == H - 2 && x == W / 2 {
                // Player spawn above exit
                PlayerSpawn
            } else {
                Floor
            };

            let variant = if matches!(tile_type, Floor | PlayerSpawn) {
                ((x + y) % 3) as u8
            } else {
                0
            };
            layout.set_tile(x, y, Tile::new(tile_type).with_variant(variant));
        }
    }

    layout.entrance = (W / 2, H - 2);
    layout.exit = Some((W / 2, H - 1));

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
