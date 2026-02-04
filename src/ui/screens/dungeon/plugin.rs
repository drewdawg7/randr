use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use tracing::instrument;

use crate::combat::ActiveCombat;
use crate::crafting_station::{AnvilActiveTimer, CraftingStationType, ForgeActiveTimer};
use crate::dungeon::{
    ChestEntity, ChestMined, CraftingStationEntity, CraftingStationInteraction, DepthSorting,
    DungeonEntityMarker, DungeonRegistry, DungeonState, FloorReady, GameLayer, MerchantInteraction,
    MineableEntityType, MiningResult, MovementConfig, MoveResult, NpcEntity, OverlappingCraftingStation,
    PlayerMoveIntent, RockEntity, RockMined, SpawnFloor, TileWorldSize, TilemapInfo,
};
use crate::input::GameAction;
use crate::location::LocationId;
use crate::mob::MobId;
use crate::states::{AppState, StateTransitionRequest};
use crate::ui::screens::anvil_modal::ActiveAnvilEntity;
use crate::ui::screens::fight_modal::state::FightModalMob;
use crate::ui::screens::forge_modal::ActiveForgeEntity;
use crate::ui::screens::modal::{ActiveModal, ModalType, OpenModal};
use crate::ui::screens::results_modal::ResultsModalData;
use crate::ui::PlayerSpriteSheet;

use super::components::{DungeonPlayer, FloorRoot, PendingPlayerSpawn};
use super::spawn::{add_entity_visuals, position_camera, spawn_floor_ui, spawn_player, DungeonCamera};
use super::systems::cleanup_dungeon;

pub struct DungeonScreenPlugin;

impl Plugin for DungeonScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(add_entity_visuals)
            .add_observer(on_map_created_queue_player_spawn)
            .add_systems(OnEnter(AppState::Dungeon), enter_dungeon)
            .add_systems(OnExit(AppState::Dungeon), cleanup_dungeon)
            .add_systems(
                Update,
                (
                    handle_floor_ready.run_if(on_message::<FloorReady>),
                    spawn_player_when_ready.run_if(resource_exists::<PendingPlayerSpawn>),
                    handle_dungeon_movement
                        .run_if(on_message::<GameAction>)
                        .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()),
                    handle_move_result.run_if(on_message::<MoveResult>),
                    update_player_sprite_direction,
                    handle_interact_action
                        .run_if(on_message::<GameAction>)
                        .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()),
                    handle_crafting_station_interaction.run_if(on_message::<CraftingStationInteraction>),
                    handle_mining_result.run_if(on_message::<MiningResult>),
                    handle_back_action,
                )
                    .chain()
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}

fn enter_dungeon(
    mut commands: Commands,
    registry: Res<DungeonRegistry>,
    mut state: ResMut<DungeonState>,
    mut spawn_floor: MessageWriter<SpawnFloor>,
) {
    if !state.is_in_dungeon() {
        state.enter_dungeon(LocationId::Home, &registry);
    }

    let Some(spawn_config) = state.get_spawn_config() else {
        return;
    };
    commands.insert_resource(spawn_config);

    let layout_id = state
        .current_floor()
        .map(|f| f.layout_id())
        .unwrap_or(crate::dungeon::LayoutId::CaveFloor);

    spawn_floor.write(SpawnFloor { layout_id });
}

#[instrument(level = "debug", skip_all)]
fn handle_floor_ready(
    mut commands: Commands,
    mut events: MessageReader<FloorReady>,
    asset_server: Res<AssetServer>,
    camera_query: Single<Entity, With<Camera2d>>,
    floor_root_query: Query<Entity, With<FloorRoot>>,
) {
    for event in events.read() {
        if let Ok(floor_root) = floor_root_query.single() {
            commands.entity(floor_root).despawn();
        }

        spawn_floor_ui(
            &mut commands,
            &asset_server,
            event.layout_id,
            *camera_query,
        );
    }
}

fn on_map_created_queue_player_spawn(
    _trigger: On<TiledEvent<MapCreated>>,
    mut commands: Commands,
    existing_player: Query<Entity, With<DungeonPlayer>>,
) {
    for entity in &existing_player {
        commands.entity(entity).despawn();
    }

    commands.insert_resource(PendingPlayerSpawn);
}

#[instrument(level = "debug", skip_all)]
fn spawn_player_when_ready(
    mut commands: Commands,
    tilemap_info: Option<Res<TilemapInfo>>,
    depth_sorting: Option<Res<DepthSorting>>,
    player_sheet: Res<PlayerSpriteSheet>,
    camera_query: Query<Entity, With<DungeonCamera>>,
) {
    let Some(info) = tilemap_info else {
        return;
    };

    let depth = depth_sorting.map(|d| *d).unwrap_or_default();

    spawn_player(&mut commands, info.center, &player_sheet, &depth);

    if let Ok(camera_entity) = camera_query.single() {
        position_camera(&mut commands, camera_entity, info.center, &depth);
    }

    commands.remove_resource::<PendingPlayerSpawn>();
}

fn handle_dungeon_movement(
    mut action_reader: MessageReader<GameAction>,
    mut move_events: MessageWriter<PlayerMoveIntent>,
) {
    for action in action_reader.read() {
        if let GameAction::Navigate(direction) = action {
            move_events.write(PlayerMoveIntent { direction: *direction });
        }
    }
}

fn handle_move_result(
    mut commands: Commands,
    mut events: MessageReader<MoveResult>,
    fight_mob: Option<Res<FightModalMob>>,
) {
    for event in events.read() {
        if let MoveResult::TriggeredCombat { mob_id, entity, pos } = event {
            if fight_mob.is_some() {
                continue;
            }
            commands.insert_resource(FightModalMob {
                mob_id: *mob_id,
                pos: *pos,
                entity: *entity,
            });
            commands.insert_resource(ActiveCombat { mob_entity: *entity });
            commands.trigger(OpenModal(ModalType::FightModal));
        }
    }
}

#[instrument(level = "debug", skip_all)]
fn handle_interact_action(
    mut commands: Commands,
    mut action_reader: MessageReader<GameAction>,
    mut crafting_events: MessageWriter<CraftingStationInteraction>,
    overlapping_station: Res<OverlappingCraftingStation>,
    tile_size: Option<Res<TileWorldSize>>,
    spatial_query: SpatialQuery,
    marker_query: Query<&DungeonEntityMarker>,
    npc_query: Query<&NpcEntity>,
    crafting_query: Query<&CraftingStationEntity>,
    chest_query: Query<(), With<ChestEntity>>,
    rock_query: Query<&RockEntity>,
    player_query: Query<&Position, With<DungeonPlayer>>,
) {
    let is_interact = action_reader.read().any(|a| *a == GameAction::Mine);
    if !is_interact {
        return;
    }

    if let Some(entity) = overlapping_station.0 {
        if let Ok(crafting) = crafting_query.get(entity) {
            crafting_events.write(CraftingStationInteraction {
                entity,
                station_type: crafting.station_type,
            });
            return;
        }
    }

    let Ok(&Position(Vec2 { x: px, y: py })) = player_query.single() else {
        return;
    };

    let step = tile_size.map(|t| t.0).unwrap_or(crate::dungeon::constants::DEFAULT_TILE_SIZE);
    let adjacent_positions: [Vec2; 4] = [
        Vec2::new(px, py - step),
        Vec2::new(px, py + step),
        Vec2::new(px - step, py),
        Vec2::new(px + step, py),
    ];

    let filter = SpatialQueryFilter::from_mask([GameLayer::StaticEntity, GameLayer::Mob]);

    for pos in adjacent_positions {
        let intersections = spatial_query.point_intersections(pos, &filter);

        for entity in intersections {
            let Ok(marker) = marker_query.get(entity) else {
                continue;
            };

            if let Ok(npc) = npc_query.get(entity) {
                if npc.mob_id == MobId::Merchant {
                    commands.trigger(MerchantInteraction { entity });
                }
                return;
            }

            if chest_query.get(entity).is_ok() {
                commands.trigger(ChestMined {
                    entity,
                    pos: marker.pos,
                });
                return;
            }

            if let Ok(rock) = rock_query.get(entity) {
                commands.trigger(RockMined {
                    entity,
                    pos: marker.pos,
                    rock_type: rock.rock_type,
                });
                return;
            }
        }
    }
}

fn handle_crafting_station_interaction(
    mut commands: Commands,
    mut events: MessageReader<CraftingStationInteraction>,
    forge_query: Query<&ForgeActiveTimer>,
    anvil_query: Query<&AnvilActiveTimer>,
) {
    for event in events.read() {
        match event.station_type {
            CraftingStationType::Forge => {
                if forge_query.get(event.entity).is_err() {
                    commands.insert_resource(ActiveForgeEntity(event.entity));
                    commands.trigger(OpenModal(ModalType::ForgeModal));
                }
            }
            CraftingStationType::Anvil => {
                if anvil_query.get(event.entity).is_err() {
                    commands.insert_resource(ActiveAnvilEntity(event.entity));
                    commands.trigger(OpenModal(ModalType::AnvilModal));
                }
            }
        }
    }
}

fn handle_mining_result(mut commands: Commands, mut events: MessageReader<MiningResult>) {
    for event in events.read() {
        let title = match &event.mineable_type {
            MineableEntityType::Chest => "Chest Opened!".to_string(),
            MineableEntityType::Rock { rock_type } => {
                format!("{} Mined!", rock_type.display_name())
            }
        };

        commands.insert_resource(ResultsModalData {
            title,
            subtitle: None,
            sprite: None,
            gold_gained: None,
            xp_gained: None,
            loot_drops: event.loot_drops.clone(),
        });
        commands.trigger(OpenModal(ModalType::ResultsModal));
    }
}

fn handle_back_action(
    mut action_events: MessageReader<GameAction>,
    mut state_requests: MessageWriter<StateTransitionRequest>,
) {
    for action in action_events.read() {
        if matches!(action, GameAction::Back) {
            state_requests.write(StateTransitionRequest::Menu);
        }
    }
}

fn update_player_sprite_direction(
    mut query: Query<(&LinearVelocity, &mut Sprite), With<DungeonPlayer>>,
    movement: Res<MovementConfig>,
    tile_size: Res<TileWorldSize>,
) {
    let threshold = movement.flip_threshold(tile_size.0);

    for (velocity, mut sprite) in &mut query {
        if velocity.x < -threshold {
            sprite.flip_x = true;
        } else if velocity.x > threshold {
            sprite.flip_x = false;
        }
    }
}
