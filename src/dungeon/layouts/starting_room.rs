use crate::dungeon::{DungeonLayout, Tile, TileType};

pub fn create() -> DungeonLayout {
    const W: usize = 8;
    const H: usize = 6;

    let mut layout = DungeonLayout::new(W, H);

    use TileType::*;

    #[rustfmt::skip]
    let tiles: [[TileType; W]; H] = [
        [Wall, Wall,  Wall,  Wall, Wall,  Wall,  Wall,  Wall],
        [Wall, Floor, Floor, Floor, Floor, Floor, Floor, Wall],
        [Wall, Floor, Floor, Floor, Floor, Floor, Floor, Wall],
        [Wall, Floor, Floor, Floor, Floor, Floor, Floor, Wall],
        [Wall, Floor, Floor, Floor, Floor, Floor, Floor, Wall],
        [Wall, Wall,  Wall,  Exit, Wall,  Wall,  Wall,  Wall],
    ];

    for (y, row) in tiles.iter().enumerate() {
        for (x, &tile_type) in row.iter().enumerate() {
            let variant = if tile_type == Floor {
                ((x + y) % 3) as u8
            } else {
                0
            };
            layout.set_tile(x, y, Tile::new(tile_type).with_variant(variant));
        }
    }

    layout.entrance = (4, 3);
    layout.exit = Some((3, 5));

    layout
}
