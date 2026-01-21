use rand::seq::SliceRandom;
use rand::Rng;

use crate::dungeon::{DungeonEntity, DungeonLayout, Tile, TileType};
use crate::mob::MobId;

pub fn create() -> DungeonLayout {
    const W: usize = 8;
    const H: usize = 6;

    let mut layout = DungeonLayout::new(W, H);

    use TileType::*;

    #[rustfmt::skip]
    let tiles: [[TileType; W]; H] = [
        [Wall, Wall,  Wall,  Wall,        Wall,  Wall,  Wall,  Wall],
        [Wall, Floor, Floor, Floor,       Floor, Floor, Floor, Wall],
        [Wall, Floor, Floor, Floor,       Floor, Floor, Floor, Wall],
        [Wall, Floor, Floor, Floor,       Floor, Floor, Floor, Wall],
        [Wall, Floor, Floor, PlayerSpawn, Floor, Floor, Floor, Wall],
        [Wall, Wall,  Wall,  Exit,        Wall,  Wall,  Wall,  Wall],
    ];

    for (y, row) in tiles.iter().enumerate() {
        for (x, &tile_type) in row.iter().enumerate() {
            let variant = if matches!(tile_type, Floor | PlayerSpawn) {
                ((x + y) % 3) as u8
            } else {
                0
            };
            layout.set_tile(x, y, Tile::new(tile_type).with_variant(variant));
        }
    }

    layout.entrance = (3, 4);
    layout.exit = Some((3, 5));

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
