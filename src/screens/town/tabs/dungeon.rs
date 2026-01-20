use bevy::prelude::*;

use crate::assets::{DungeonTileSlice, GameSprites, SpriteSheetKey};

use super::super::{ContentArea, TabContent, TownTab};

/// Plugin for the Dungeon tab.
pub struct DungeonTabPlugin;

impl Plugin for DungeonTabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(TownTab::Dungeon), spawn_dungeon_content);
    }
}

/// Spawns dungeon UI content when entering the Dungeon tab.
fn spawn_dungeon_content(
    mut commands: Commands,
    content_query: Query<Entity, With<ContentArea>>,
    game_sprites: Res<GameSprites>,
) {
    let Ok(content_entity) = content_query.get_single() else {
        return;
    };

    let Some(sheet) = game_sprites.get(SpriteSheetKey::DungeonTileset) else {
        return;
    };

    // Tile size (original 16x16, scaled up 3x)
    const TILE_SIZE: f32 = 48.0;

    // Define a simple dungeon room layout (8 columns x 6 rows)
    // Each cell is (DungeonTileSlice, flip_x)
    let layout: [[(DungeonTileSlice, bool); 8]; 6] = [
        // Row 0: Top wall
        [
            (DungeonTileSlice::SideWall5, true),
            (DungeonTileSlice::TopWall1, false),
            (DungeonTileSlice::TopWall2, false),
            (DungeonTileSlice::TorchWall1, false),
            (DungeonTileSlice::TopWall3, false),
            (DungeonTileSlice::TopWall4, false),
            (DungeonTileSlice::TopWall2, false),
            (DungeonTileSlice::SideWall5, false),
        ],
        // Row 1: Upper middle
        [
            (DungeonTileSlice::SideWall6, true),
            (DungeonTileSlice::FloorTile2, false),
            (DungeonTileSlice::FloorTile3, false),
            (DungeonTileSlice::FloorTile4, false),
            (DungeonTileSlice::FloorTile2, false),
            (DungeonTileSlice::FloorTile3, false),
            (DungeonTileSlice::FloorTile4, false),
            (DungeonTileSlice::SideWall6, false),
        ],
        // Row 2: Middle
        [
            (DungeonTileSlice::SideWall7, true),
            (DungeonTileSlice::FloorTile3, false),
            (DungeonTileSlice::FloorTile4, false),
            (DungeonTileSlice::FloorTile2, false),
            (DungeonTileSlice::FloorTile3, false),
            (DungeonTileSlice::FloorTile4, false),
            (DungeonTileSlice::FloorTile2, false),
            (DungeonTileSlice::SideWall7, false),
        ],
        // Row 3: Middle
        [
            (DungeonTileSlice::SideWall8, true),
            (DungeonTileSlice::FloorTile4, false),
            (DungeonTileSlice::FloorTile2, false),
            (DungeonTileSlice::FloorTile3, false),
            (DungeonTileSlice::FloorTile4, false),
            (DungeonTileSlice::FloorTile2, false),
            (DungeonTileSlice::FloorTile3, false),
            (DungeonTileSlice::SideWall8, false),
        ],
        // Row 4: Lower middle
        [
            (DungeonTileSlice::SideWall6, true),
            (DungeonTileSlice::FloorTile2, false),
            (DungeonTileSlice::FloorTile3, false),
            (DungeonTileSlice::FloorTile4, false),
            (DungeonTileSlice::FloorTile2, false),
            (DungeonTileSlice::FloorTile3, false),
            (DungeonTileSlice::FloorTile4, false),
            (DungeonTileSlice::SideWall6, false),
        ],
        // Row 5: Bottom wall
        [
            (DungeonTileSlice::BottomRightWall, true),
            (DungeonTileSlice::BottomWall1, false),
            (DungeonTileSlice::BottomWall2, false),
            (DungeonTileSlice::Gate, false),
            (DungeonTileSlice::BottomWall3, false),
            (DungeonTileSlice::BottomWall4, false),
            (DungeonTileSlice::BottomWall2, false),
            (DungeonTileSlice::BottomRightWall, false),
        ],
    ];

    commands.entity(content_entity).with_children(|parent| {
        parent
            .spawn((
                TabContent,
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ))
            .with_children(|content| {
                // Spawn grid container
                content
                    .spawn(Node {
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::px(TILE_SIZE); 8],
                        grid_template_rows: vec![GridTrack::px(TILE_SIZE); 6],
                        ..default()
                    })
                    .with_children(|grid| {
                        // Spawn each tile
                        for row in &layout {
                            for (tile, flip_x) in row {
                                let mut cell = grid.spawn(Node {
                                    width: Val::Px(TILE_SIZE),
                                    height: Val::Px(TILE_SIZE),
                                    ..default()
                                });
                                if let Some(mut img) = sheet.image_node(tile.as_str()) {
                                    if *flip_x {
                                        img.flip_x = true;
                                    }
                                    cell.insert(img);
                                }
                            }
                        }
                    });
            });
    });
}
