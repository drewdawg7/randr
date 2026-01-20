use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::dungeon::{LayoutId, TileRenderer};

use super::super::{ContentArea, TabContent, TownTab};

pub struct DungeonTabPlugin;

impl Plugin for DungeonTabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(TownTab::Dungeon), spawn_dungeon_content);
    }
}

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

    const TILE_SIZE: f32 = 48.0;

    let layout = LayoutId::StartingRoom.layout();

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
                content
                    .spawn(Node {
                        display: Display::Grid,
                        grid_template_columns: vec![GridTrack::px(TILE_SIZE); layout.width()],
                        grid_template_rows: vec![GridTrack::px(TILE_SIZE); layout.height()],
                        ..default()
                    })
                    .with_children(|grid| {
                        for y in 0..layout.height() {
                            for x in 0..layout.width() {
                                let mut cell = grid.spawn(Node {
                                    width: Val::Px(TILE_SIZE),
                                    height: Val::Px(TILE_SIZE),
                                    ..default()
                                });
                                if let Some((slice, flip_x)) = TileRenderer::resolve(&layout, x, y)
                                {
                                    if let Some(mut img) = sheet.image_node(slice.as_str()) {
                                        if flip_x {
                                            img.flip_x = true;
                                        }
                                        cell.insert(img);
                                    }
                                }
                            }
                        }
                    });
            });
    });
}
