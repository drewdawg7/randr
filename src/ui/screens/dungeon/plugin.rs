use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::crafting_station::{
    AnvilActiveTimer, AnvilCraftingState, CraftingStationType, ForgeActiveTimer,
    ForgeCraftingState,
};
use crate::dungeon::{
    CraftingStationInteraction, DungeonEntity, DungeonEntityMarker, DungeonRegistry, DungeonState,
    FloorReady, GridOccupancy, MineEntity, MiningResult, MoveResult, NpcInteraction,
    PlayerMoveIntent, SpawnFloor, TilesetGrid,
};
use crate::input::{GameAction, NavigationDirection};
use crate::inventory::{Inventory, ManagesItems};
use crate::location::LocationId;
use crate::mob::MobId;
use crate::states::AppState;
use crate::ui::screens::anvil_modal::ActiveAnvilEntity;
use crate::ui::screens::fight_modal::state::{FightModalMob, SpawnFightModal};
use crate::ui::screens::forge_modal::ActiveForgeEntity;
use crate::ui::screens::merchant_modal::MerchantStock;
use crate::ui::screens::modal::{ActiveModal, ModalType, OpenModal};
use crate::ui::screens::results_modal::{ResultsModalData, SpawnResultsModal};
use crate::ui::MobSpriteSheets;
use crate::ui::{PlayerSpriteSheet, PlayerWalkTimer, SpriteAnimation};

use super::components::{DungeonPlayer, DungeonRoot, SmoothPosition, TileSizes, UiScale};
use super::constants::ENTITY_VISUAL_SCALE;
use super::spawn::spawn_floor_ui;
use super::systems::{cleanup_dungeon, handle_window_resize, interpolate_positions};

pub struct DungeonScreenPlugin;

impl Plugin for DungeonScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Dungeon), enter_dungeon)
            .add_systems(OnExit(AppState::Dungeon), cleanup_dungeon)
            .add_systems(
                Update,
                (
                    handle_floor_ready.run_if(on_event::<FloorReady>),
                    handle_dungeon_movement,
                    handle_move_result.run_if(on_event::<MoveResult>),
                    interpolate_positions,
                    handle_interact_action,
                    handle_npc_interaction.run_if(on_event::<NpcInteraction>),
                    handle_crafting_station_interaction.run_if(on_event::<CraftingStationInteraction>),
                    handle_mining_result.run_if(on_event::<MiningResult>),
                    handle_back_action,
                    handle_window_resize,
                    revert_forge_idle,
                    revert_anvil_idle,
                )
                    .chain()
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}

fn enter_dungeon(
    registry: Res<DungeonRegistry>,
    mut state: ResMut<DungeonState>,
    mut spawn_floor: EventWriter<SpawnFloor>,
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

    spawn_floor.send(SpawnFloor {
        layout,
        player_pos: state.player_pos,
        player_size: state.player_size,
        floor_type,
    });
}

fn handle_floor_ready(
    mut commands: Commands,
    mut events: EventReader<FloorReady>,
    game_sprites: Res<GameSprites>,
    mob_sheets: Res<MobSpriteSheets>,
    tileset: Res<TilesetGrid>,
    window: Single<&Window>,
    root_query: Query<Entity, With<DungeonRoot>>,
) {
    for event in events.read() {
        for entity in &root_query {
            commands.entity(entity).despawn_recursive();
        }
        commands.remove_resource::<UiScale>();
        commands.remove_resource::<TileSizes>();

        spawn_floor_ui(
            &mut commands,
            &event.layout,
            event.player_pos,
            event.floor_type,
            &game_sprites,
            &mob_sheets,
            &tileset,
            &window,
        );
    }
}

#[derive(Resource)]
struct LastMoveDirection(NavigationDirection);

fn handle_dungeon_movement(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut move_events: EventWriter<PlayerMoveIntent>,
    keyboard: Res<ButtonInput<KeyCode>>,
    active_modal: Res<ActiveModal>,
    player_query: Query<&SmoothPosition, With<DungeonPlayer>>,
) {
    if active_modal.modal.is_some() {
        return;
    }

    let Ok(smooth_pos) = player_query.get_single() else {
        return;
    };

    if smooth_pos.moving {
        return;
    }

    let direction = action_reader
        .read()
        .find_map(|a| match a {
            GameAction::Navigate(dir) => Some(*dir),
            _ => None,
        })
        .or_else(|| held_direction(&keyboard));

    let Some(direction) = direction else {
        return;
    };

    commands.insert_resource(LastMoveDirection(direction));
    move_events.send(PlayerMoveIntent { direction });
}

fn handle_move_result(
    mut commands: Commands,
    mut events: EventReader<MoveResult>,
    last_direction: Option<Res<LastMoveDirection>>,
    tile_sizes: Res<TileSizes>,
    sheet: Res<PlayerSpriteSheet>,
    mut player_query: Query<
        (&mut SmoothPosition, &mut ImageNode, &mut SpriteAnimation, &mut PlayerWalkTimer),
        With<DungeonPlayer>,
    >,
) {
    for event in events.read() {
        match event {
            MoveResult::Moved { new_pos } => {
                let Ok((mut smooth_pos, mut player_image, mut anim, mut walk_timer)) =
                    player_query.get_single_mut()
                else {
                    continue;
                };

                let tile_size = tile_sizes.tile_size;
                let entity_sprite_size = ENTITY_VISUAL_SCALE * tile_sizes.base_tile_size;
                let entity_offset = -(entity_sprite_size - tile_size) / 2.0;
                smooth_pos.target = Vec2::new(
                    new_pos.x as f32 * tile_size + entity_offset,
                    new_pos.y as f32 * tile_size + entity_offset,
                );
                smooth_pos.moving = true;

                if let Some(ref dir) = last_direction {
                    match dir.0 {
                        NavigationDirection::Left => player_image.flip_x = true,
                        NavigationDirection::Right => player_image.flip_x = false,
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
                // Combat data is now stored on the entity's components (MobCombatBundle)
                commands.insert_resource(FightModalMob {
                    mob_id: *mob_id,
                    pos: *pos,
                    entity: *entity,
                });
                commands.insert_resource(SpawnFightModal);
            }
            MoveResult::Blocked | MoveResult::TriggeredStairs | MoveResult::TriggeredDoor => {}
        }
    }
}

fn held_direction(keyboard: &ButtonInput<KeyCode>) -> Option<NavigationDirection> {
    if keyboard.pressed(KeyCode::ArrowLeft) {
        Some(NavigationDirection::Left)
    } else if keyboard.pressed(KeyCode::ArrowRight) {
        Some(NavigationDirection::Right)
    } else if keyboard.pressed(KeyCode::ArrowUp) {
        Some(NavigationDirection::Up)
    } else if keyboard.pressed(KeyCode::ArrowDown) {
        Some(NavigationDirection::Down)
    } else {
        None
    }
}

fn handle_interact_action(
    mut action_reader: EventReader<GameAction>,
    mut npc_events: EventWriter<NpcInteraction>,
    mut crafting_events: EventWriter<CraftingStationInteraction>,
    mut mine_events: EventWriter<MineEntity>,
    state: Res<DungeonState>,
    occupancy: Res<GridOccupancy>,
    active_modal: Res<ActiveModal>,
    entity_query: Query<&DungeonEntityMarker>,
) {
    if active_modal.modal.is_some() {
        return;
    }

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
                npc_events.send(NpcInteraction { mob_id: *mob_id });
                return;
            }
            DungeonEntity::CraftingStation { station_type, .. } => {
                crafting_events.send(CraftingStationInteraction {
                    entity,
                    station_type: *station_type,
                });
                return;
            }
            DungeonEntity::Chest { .. } | DungeonEntity::Rock { .. } => {
                mine_events.send(MineEntity {
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

fn handle_npc_interaction(mut commands: Commands, mut events: EventReader<NpcInteraction>) {
    for event in events.read() {
        if event.mob_id == MobId::Merchant {
            commands.insert_resource(MerchantStock::generate());
            commands.trigger(OpenModal(ModalType::MerchantModal));
        }
    }
}

fn handle_crafting_station_interaction(
    mut commands: Commands,
    mut events: EventReader<CraftingStationInteraction>,
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

fn handle_mining_result(mut commands: Commands, mut events: EventReader<MiningResult>) {
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
        commands.insert_resource(SpawnResultsModal);
    }
}

fn revert_forge_idle(
    mut commands: Commands,
    time: Res<Time>,
    game_sprites: Res<GameSprites>,
    mut query: Query<(
        Entity,
        &mut ForgeActiveTimer,
        &mut ImageNode,
        Option<&mut ForgeCraftingState>,
    )>,
) {
    for (entity, mut timer, mut image, forge_state) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Some(mut state) = forge_state {
                state.complete_crafting();
            }

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
    mut inventory: ResMut<Inventory>,
    mut query: Query<(
        Entity,
        &mut AnvilActiveTimer,
        &mut ImageNode,
        Option<&mut AnvilCraftingState>,
    )>,
) {
    for (entity, mut timer, mut image, anvil_state) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Some(mut state) = anvil_state {
                if let Some(recipe_id) = state.complete_crafting() {
                    let spec = recipe_id.spec();
                    let item = spec.output.spawn();
                    let _ = inventory.add_to_inv(item);
                }
            }

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
    mut action_events: EventReader<GameAction>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for action in action_events.read() {
        if matches!(action, GameAction::Back) {
            next_state.set(AppState::Menu);
        }
    }
}
