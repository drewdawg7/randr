use std::collections::HashMap;

use bevy::asset::LoadedFolder;
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::item::definitions::{ItemId, ItemSpec};
use crate::mob::definitions::{MobId, MobSpec};
use crate::registry::Registry;
use crate::states::AppState;

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<MobSpec>::new(&["mob.ron"]),
            RonAssetPlugin::<ItemSpec>::new(&["item.ron"]),
        ))
        .add_systems(OnEnter(AppState::Loading), start_loading)
        .add_systems(
            Update,
            check_loading_complete.run_if(in_state(AppState::Loading)),
        );
    }
}

#[derive(Resource)]
struct PendingLoads {
    mob_folder: Handle<LoadedFolder>,
    item_folder: Handle<LoadedFolder>,
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(PendingLoads {
        mob_folder: asset_server.load_folder("data/mobs"),
        item_folder: asset_server.load_folder("data/items"),
    });
}

fn check_loading_complete(
    mut commands: Commands,
    pending: Res<PendingLoads>,
    folders: Res<Assets<LoadedFolder>>,
    mob_assets: Res<Assets<MobSpec>>,
    item_assets: Res<Assets<ItemSpec>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (Some(mob_folder), Some(item_folder)) = (
        folders.get(&pending.mob_folder),
        folders.get(&pending.item_folder),
    ) else {
        return;
    };

    let mob_specs: Vec<&MobSpec> = mob_folder
        .handles
        .iter()
        .filter_map(|h| mob_assets.get(h.id().typed::<MobSpec>()))
        .collect();

    if mob_specs.len() != mob_folder.handles.len() {
        return;
    }

    let item_specs: Vec<&ItemSpec> = item_folder
        .handles
        .iter()
        .filter_map(|h| item_assets.get(h.id().typed::<ItemSpec>()))
        .collect();

    if item_specs.len() != item_folder.handles.len() {
        return;
    }

    let mob_map: HashMap<MobId, MobSpec> = mob_specs
        .into_iter()
        .map(|spec| (spec.id, spec.clone()))
        .collect();
    let item_map: HashMap<ItemId, ItemSpec> = item_specs
        .into_iter()
        .map(|spec| (spec.id, spec.clone()))
        .collect();

    crate::mob::data::populate(mob_map.clone());
    crate::item::data::populate(item_map.clone());

    commands.insert_resource(Registry::new(mob_map));
    commands.insert_resource(Registry::new(item_map));

    commands.remove_resource::<PendingLoads>();
    next_state.set(AppState::Menu);
}
