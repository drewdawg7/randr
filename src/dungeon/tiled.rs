use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use super::layouts::LayoutId;

pub fn map_path(layout_id: LayoutId) -> &'static str {
    match layout_id {
        LayoutId::CaveFloor => "maps/cave_floor.tmx",
        LayoutId::HomeFloor => "maps/home_floor.tmx",
    }
}

pub fn load_floor_map(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layout_id: LayoutId,
) -> Entity {
    let path = map_path(layout_id);
    let map_handle: Handle<TiledMapAsset> = asset_server.load(path);
    commands.spawn(TiledMap(map_handle)).id()
}
