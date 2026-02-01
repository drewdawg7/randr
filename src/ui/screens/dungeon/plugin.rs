use bevy::prelude::*;
use tracing::instrument;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::combat::ActiveCombat;
use crate::crafting_station::{AnvilActiveTimer, CraftingStationType, ForgeActiveTimer};
use crate::dungeon::{
    CraftingStationInteraction, DungeonEntity, DungeonEntityMarker, DungeonRegistry, DungeonState,
    FloorReady, GridOccupancy, MineEntity, MiningResult, MoveResult, NpcInteraction,
    PlayerMoveIntent, SpawnFloor,
};
use crate::input::{GameAction, HeldDirection, NavigationDirection};
use crate::game::{AnvilCraftingCompleteEvent, ForgeCraftingCompleteEvent};
use crate::location::LocationId;
use crate::mob::MobId;
use crate::states::AppState;
use crate::ui::screens::anvil_modal::ActiveAnvilEntity;
use crate::ui::screens::fight_modal::state::FightModalMob;
use crate::ui::screens::forge_modal::ActiveForgeEntity;
use crate::ui::screens::merchant_modal::MerchantStock;
use crate::ui::screens::modal::{ActiveModal, ModalType, OpenModal};
use crate::ui::screens::results_modal::ResultsModalData;
use crate::ui::MobSpriteSheets;
use crate::ui::{PlayerSpriteSheet, PlayerWalkTimer, SpriteAnimation};

use super::components::{DungeonPlayer, DungeonRoot, Interpolating, TargetPosition, TileSizes};
use super::spawn::spawn_floor_ui;
use super::systems::cleanup_dungeon;

pub struct DungeonScreenPlugin;

impl Plugin for DungeonScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Dungeon), enter_dungeon)
            .add_systems(OnExit(AppState::Dungeon), cleanup_dungeon)
            .add_systems(
                Update,
                (
                    handle_floor_ready.run_if(on_message::<FloorReady>),
                    handle_dungeon_movement
                        .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()),
                    handle_move_result.run_if(on_message::<MoveResult>),
                    interpolate_player_position.run_if(any_with_component::<Interpolating>),
                    handle_interact_action
                        .run_if(on_message::<GameAction>)
                        .run_if(|modal: Res<ActiveModal>| modal.modal.is_none()),
                    handle_npc_interaction.run_if(on_message::<NpcInteraction>),
                    handle_crafting_station_interaction.run_if(on_message::<CraftingStationInteraction>),
                    handle_mining_result.run_if(on_message::<MiningResult>),
                    handle_back_action,
                    revert_forge_idle.run_if(any_with_component::<ForgeActiveTimer>),
                    revert_anvil_idle.run_if(any_with_component::<AnvilActiveTimer>),
                )
                    .chain()
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}

fn enter_dungeon(
    registry: Res<DungeonRegistry>,
    mut state: ResMut<DungeonState>,
    mut spawn_floor: MessageWriter<SpawnFloor>,
) {
    if !state.is_in_dungeon() {
        state.enter_dungeon(LocationId::Home, &registry);
    }

    state.load_floor_layout();

    let Some(layout) = state.layout.clone() else {
        return;
    };

    let floor_type = state
        .current_floor()
        .map(|f| f.floor_type())
        .unwrap_or(crate::dungeon::FloorType::CaveFloor);

    spawn_floor.write(SpawnFloor {
        layout,
        player_pos: state.player_pos,
        player_size: state.player_size,
        floor_type,
    });
}

#[instrument(level = "debug", skip_all)]
fn handle_floor_ready(
    mut commands: Commands,
    mut events: MessageReader<FloorReady>,
    asset_server: Res<AssetServer>,
    game_sprites: Res<GameSprites>,
    mob_sheets: Res<MobSpriteSheets>,
    player_sheet: Res<PlayerSpriteSheet>,
    window: Single<&Window>,
    camera_query: Single<Entity, With<Camera2d>>,
    root_query: Query<Entity, With<DungeonRoot>>,
) {
    for event in events.read() {
        for entity in &root_query {
            commands.entity(entity).despawn();
        }
        commands.remove_resource::<TileSizes>();

        spawn_floor_ui(
            &mut commands,
            &asset_server,
            &event.layout,
            event.player_pos,
            event.floor_type,
            &game_sprites,
            &mob_sheets,
            &player_sheet,
            &window,
            *camera_query,
        );
    }
}

#[derive(Resource)]
struct LastMoveDirection(NavigationDirection);

const MOVE_INTERVAL: f32 = 0.1;
const MOVE_SPEED: f32 = 8.0;

fn handle_dungeon_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut action_reader: MessageReader<GameAction>,
    mut move_events: MessageWriter<PlayerMoveIntent>,
    held_direction: Res<HeldDirection>,
    mut move_timer: Local<f32>,
) {
    let fresh_direction = action_reader.read().find_map(|a| match a {
        GameAction::Navigate(dir) => Some(*dir),
        _ => None,
    });

    if let Some(direction) = fresh_direction {
        commands.insert_resource(LastMoveDirection(direction));
        move_events.write(PlayerMoveIntent { direction });
        *move_timer = MOVE_INTERVAL;
        return;
    }

    if let Some(direction) = held_direction.0 {
        *move_timer -= time.delta_secs();
        if *move_timer <= 0.0 {
            commands.insert_resource(LastMoveDirection(direction));
            move_events.write(PlayerMoveIntent { direction });
            *move_timer = MOVE_INTERVAL;
        }
    } else {
        *move_timer = 0.0;
    }
}

fn handle_move_result(
    mut commands: Commands,
    mut events: MessageReader<MoveResult>,
    last_direction: Option<Res<LastMoveDirection>>,
    tile_sizes: Res<TileSizes>,
    sheet: Res<PlayerSpriteSheet>,
    fight_mob: Option<Res<FightModalMob>>,
    mut player_query: Query<
        (
            Entity,
            &mut TargetPosition,
            &mut Sprite,
            &mut SpriteAnimation,
            &mut PlayerWalkTimer,
        ),
        With<DungeonPlayer>,
    >,
) {
    for event in events.read() {
        match event {
            MoveResult::Moved { new_pos } => {
                let Ok((entity, mut target_pos, mut sprite, mut anim, mut walk_timer)) =
                    player_query.single_mut()
                else {
                    continue;
                };

                let tile_size = tile_sizes.tile_size;
                let map_height = tile_sizes.map_height;

                let world_x = new_pos.x as f32 * tile_size + tile_size / 2.0;
                let world_y = (map_height - 1 - new_pos.y) as f32 * tile_size + tile_size / 2.0;
                target_pos.0 = Vec2::new(world_x, world_y);
                commands.entity(entity).insert(Interpolating);

                if let Some(ref dir) = last_direction {
                    match dir.0 {
                        NavigationDirection::Left => sprite.flip_x = true,
                        NavigationDirection::Right => sprite.flip_x = false,
                        _ => {}
                    }
                }

                let already_walking = anim.first_frame == sheet.walk_animation.first_frame;
                if !already_walking {
                    anim.first_frame = sheet.walk_animation.first_frame;
                    anim.last_frame = sheet.walk_animation.last_frame;
                    anim.current_frame = sheet.walk_animation.first_frame;
                    anim.frame_duration = sheet.walk_animation.frame_duration;
                    anim.synchronized = false;
                    anim.timer =
                        Timer::from_seconds(sheet.walk_animation.frame_duration, TimerMode::Repeating);
                }
                walk_timer.0.reset();
            }
            MoveResult::TriggeredCombat { mob_id, entity, pos } => {
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
            MoveResult::Blocked | MoveResult::TriggeredStairs | MoveResult::TriggeredDoor => {}
        }
    }
}

#[instrument(level = "debug", skip_all, fields(player_pos = ?state.player_pos))]
fn handle_interact_action(
    mut action_reader: MessageReader<GameAction>,
    mut npc_events: MessageWriter<NpcInteraction>,
    mut crafting_events: MessageWriter<CraftingStationInteraction>,
    mut mine_events: MessageWriter<MineEntity>,
    state: Res<DungeonState>,
    occupancy: Res<GridOccupancy>,
    entity_query: Query<&DungeonEntityMarker>,
) {
    let is_interact = action_reader.read().any(|a| *a == GameAction::Mine);
    if !is_interact {
        return;
    }

    let px = state.player_pos.x;
    let py = state.player_pos.y;
    let adjacent_cells: [(i32, i32); 4] = [
        (px as i32, py as i32 - 1),
        (px as i32, py as i32 + 1),
        (px as i32 - 1, py as i32),
        (px as i32 + 1, py as i32),
    ];

    for (x, y) in adjacent_cells {
        if x < 0 || y < 0 {
            continue;
        }

        let Some(entity) = occupancy.entity_at(x as usize, y as usize) else {
            continue;
        };

        let Ok(marker) = entity_query.get(entity) else {
            continue;
        };

        match &marker.entity_type {
            DungeonEntity::Npc { mob_id, .. } => {
                npc_events.write(NpcInteraction { mob_id: *mob_id });
                return;
            }
            DungeonEntity::CraftingStation { station_type, .. } => {
                crafting_events.write(CraftingStationInteraction {
                    entity,
                    station_type: *station_type,
                });
                return;
            }
            DungeonEntity::Chest { .. } | DungeonEntity::Rock { .. } => {
                mine_events.write(MineEntity {
                    entity,
                    pos: marker.pos,
                    entity_type: marker.entity_type,
                });
                return;
            }
            _ => {}
        }
    }
}

fn handle_npc_interaction(mut commands: Commands, mut events: MessageReader<NpcInteraction>) {
    for event in events.read() {
        if event.mob_id == MobId::Merchant {
            commands.insert_resource(MerchantStock::generate());
            commands.trigger(OpenModal(ModalType::MerchantModal));
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
        let title = match &event.entity_type {
            DungeonEntity::Chest { .. } => "Chest Opened!".to_string(),
            DungeonEntity::Rock { rock_type, .. } => {
                format!("{} Mined!", rock_type.display_name())
            }
            _ => "Loot!".to_string(),
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

fn revert_forge_idle(
    mut commands: Commands,
    time: Res<Time>,
    game_sprites: Res<GameSprites>,
    mut crafting_events: MessageWriter<ForgeCraftingCompleteEvent>,
    mut query: Query<(Entity, &mut ForgeActiveTimer, &mut ImageNode)>,
) {
    for (entity, mut timer, mut image) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            crafting_events.write(ForgeCraftingCompleteEvent { entity });

            if let Some(sheet) = game_sprites.get(SpriteSheetKey::CraftingStations) {
                if let Some(idle_idx) = sheet.get("forge_1_idle") {
                    if let Some(ref mut atlas) = image.texture_atlas {
                        atlas.index = idle_idx;
                    }
                }
            }
            commands.entity(entity).remove::<ForgeActiveTimer>();
            commands.entity(entity).remove::<SpriteAnimation>();
        }
    }
}

fn revert_anvil_idle(
    mut commands: Commands,
    time: Res<Time>,
    game_sprites: Res<GameSprites>,
    mut crafting_events: MessageWriter<AnvilCraftingCompleteEvent>,
    mut query: Query<(Entity, &mut AnvilActiveTimer, &mut ImageNode)>,
) {
    for (entity, mut timer, mut image) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            crafting_events.write(AnvilCraftingCompleteEvent { entity });

            if let Some(sheet) = game_sprites.get(SpriteSheetKey::CraftingStations) {
                if let Some(idle_idx) = sheet.get("anvil_idle") {
                    if let Some(ref mut atlas) = image.texture_atlas {
                        atlas.index = idle_idx;
                    }
                }
            }
            commands.entity(entity).remove::<AnvilActiveTimer>();
            commands.entity(entity).remove::<SpriteAnimation>();
        }
    }
}

fn handle_back_action(
    mut action_events: MessageReader<GameAction>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for action in action_events.read() {
        if matches!(action, GameAction::Back) {
            next_state.set(AppState::Menu);
        }
    }
}

fn interpolate_player_position(
    mut commands: Commands,
    time: Res<Time>,
    tile_sizes: Option<Res<TileSizes>>,
    mut query: Query<(Entity, &TargetPosition, &mut Transform), With<Interpolating>>,
) {
    let Some(tile_sizes) = tile_sizes else { return };

    for (entity, target, mut transform) in &mut query {
        let current = transform.translation.truncate();
        let delta = target.0 - current;
        let distance = delta.length();

        if distance < 0.5 {
            transform.translation.x = target.0.x;
            transform.translation.y = target.0.y;
            commands.entity(entity).remove::<Interpolating>();
        } else {
            let speed = MOVE_SPEED * tile_sizes.tile_size;
            let step = speed * time.delta_secs();
            let new_pos = current + delta.normalize() * step.min(distance);
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;
        }
    }
}
