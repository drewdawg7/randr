use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use tracing::instrument;

use crate::assets::GameSprites;
use crate::crafting_station::{AnvilCraftingState, CraftingStationType, ForgeCraftingState};
use crate::dungeon::{
    map_path, DungeonEntity, DungeonEntityMarker, DungeonLayout, EntityRenderData, FloorType,
    GridPosition, LayoutId,
};
use crate::mob::MobCombatBundle;
use crate::ui::animation::SpriteAnimation;
use crate::ui::{MobSpriteSheets, PlayerSpriteSheet, PlayerWalkTimer};
use crate::ui::widgets::PlayerStats;

use super::components::{DungeonPlayer, DungeonRoot, Interpolating, TargetPosition, TileSizes};
use super::constants::{BASE_TILE, ENTITY_VISUAL_SCALE};

#[derive(Component)]
pub struct DungeonCamera;

pub fn spawn_floor_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layout: &DungeonLayout,
    player_pos: GridPosition,
    floor_type: FloorType,
    game_sprites: &GameSprites,
    mob_sheets: &MobSpriteSheets,
    player_sheet: &PlayerSpriteSheet,
    window: &Window,
    camera_entity: Entity,
) {
    let available_width = window.width() - 20.0;
    let available_height = window.height() - 50.0;

    let tile_scale = floor_type.tile_scale();
    let max_tile_from_width = available_width / (layout.width() as f32 * tile_scale);
    let max_tile_from_height = available_height / (layout.height() as f32 * tile_scale);

    let base_tile_size = max_tile_from_width.min(max_tile_from_height).max(BASE_TILE);
    let tile_size = base_tile_size * tile_scale;
    let map_height = layout.height();

    commands.insert_resource(TileSizes {
        tile_size,
        base_tile_size,
        map_height,
    });

    let layout_id = floor_type.layout_id(false);
    spawn_tilemap(commands, asset_server, layout_id, tile_size);

    let center_x = (layout.width() as f32 * tile_size) / 2.0;
    let center_y = (map_height as f32 * tile_size) / 2.0;
    commands.entity(camera_entity).insert((
        DungeonCamera,
        Transform::from_xyz(center_x, center_y, 999.0),
    ));

    commands
        .spawn((
            DungeonRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(PlayerStats);
        });

    let entity_sprite_size = ENTITY_VISUAL_SCALE * base_tile_size;

    spawn_entities(
        commands,
        layout,
        tile_size,
        entity_sprite_size,
        map_height,
        game_sprites,
        mob_sheets,
    );

    spawn_player(
        commands,
        player_pos,
        tile_size,
        entity_sprite_size,
        map_height,
        player_sheet,
    );
}

fn spawn_tilemap(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layout_id: LayoutId,
    tile_size: f32,
) {
    let path = map_path(layout_id);
    let map_handle: Handle<TiledMapAsset> = asset_server.load(path);
    let scale = tile_size / 32.0;

    commands.spawn((
        TiledMap(map_handle),
        Transform::from_scale(Vec3::splat(scale)),
    ));
}

fn grid_to_world(grid_x: usize, grid_y: usize, tile_size: f32, map_height: usize) -> Vec3 {
    let world_x = grid_x as f32 * tile_size + tile_size / 2.0;
    let world_y = (map_height - 1 - grid_y) as f32 * tile_size + tile_size / 2.0;
    let z = grid_y as f32 * 0.01;
    Vec3::new(world_x, world_y, z)
}

#[instrument(level = "debug", skip_all, fields(entity_count = layout.entities().len()))]
fn spawn_entities(
    commands: &mut Commands,
    layout: &DungeonLayout,
    tile_size: f32,
    entity_sprite_size: f32,
    map_height: usize,
    game_sprites: &GameSprites,
    mob_sheets: &MobSpriteSheets,
) {
    for (pos, entity) in layout.entities() {
        let world_pos = grid_to_world(pos.x, pos.y, tile_size, map_height);
        let scale = entity_sprite_size / 32.0;

        let marker = DungeonEntityMarker {
            pos: *pos,
            entity_type: *entity,
        };

        match entity.render_data() {
            EntityRenderData::SpriteSheet {
                sheet_key,
                sprite_name,
            } => {
                if let Some(sheet) = game_sprites.get(sheet_key) {
                    if let Some(sprite) = sheet.sprite(sprite_name) {
                        let mut entity_cmd = commands.spawn((
                            marker,
                            sprite,
                            Transform::from_translation(world_pos)
                                .with_scale(Vec3::splat(scale)),
                        ));
                        match entity {
                            DungeonEntity::CraftingStation {
                                station_type: CraftingStationType::Forge,
                                ..
                            } => {
                                entity_cmd.insert(ForgeCraftingState::default());
                            }
                            DungeonEntity::CraftingStation {
                                station_type: CraftingStationType::Anvil,
                                ..
                            } => {
                                entity_cmd.insert(AnvilCraftingState::default());
                            }
                            _ => {}
                        }
                    }
                }
            }
            EntityRenderData::AnimatedMob { mob_id } => {
                if let Some(sheet) = mob_sheets.get(mob_id) {
                    commands.spawn((
                        marker,
                        MobCombatBundle::from_mob_id(mob_id),
                        Sprite::from_atlas_image(
                            sheet.texture.clone(),
                            TextureAtlas {
                                layout: sheet.layout.clone(),
                                index: sheet.animation.first_frame,
                            },
                        ),
                        Transform::from_translation(world_pos)
                            .with_scale(Vec3::splat(scale)),
                        SpriteAnimation::new(&sheet.animation),
                    ));
                }
            }
        }
    }
}

fn spawn_player(
    commands: &mut Commands,
    player_pos: GridPosition,
    tile_size: f32,
    entity_sprite_size: f32,
    map_height: usize,
    player_sheet: &PlayerSpriteSheet,
) {
    let world_pos = grid_to_world(player_pos.x, player_pos.y, tile_size, map_height);
    let scale = entity_sprite_size / 32.0;

    let Some(texture) = player_sheet.texture.clone() else {
        return;
    };
    let Some(layout) = player_sheet.layout.clone() else {
        return;
    };

    commands.spawn((
        DungeonPlayer,
        TargetPosition(Vec2::new(world_pos.x, world_pos.y)),
        Interpolating,
        PlayerWalkTimer(Timer::from_seconds(0.3, TimerMode::Once)),
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: player_sheet.animation.first_frame,
            },
        ),
        Transform::from_translation(world_pos).with_scale(Vec3::splat(scale)),
        SpriteAnimation::new(&player_sheet.animation),
    ));
}
