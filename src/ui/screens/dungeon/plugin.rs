use bevy::prelude::*;
use tracing::instrument;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::combat::ActiveCombat;
use crate::crafting_station::{
    AnvilActiveTimer, AnvilCraftingState, CraftingStationType, ForgeActiveTimer,
    ForgeCraftingState,
};
use crate::dungeon::{
    CraftingStationInteraction, DungeonEntity, DungeonEntityMarker, DungeonRegistry, DungeonState,
    FloorReady, GridOccupancy, MineEntity, MiningResult, MoveResult, NpcInteraction,
    PlayerMoveIntent, SpawnFloor, TilesetGrid,
};
use crate::input::{GameAction, HeldDirection, NavigationDirection};
use crate::inventory::{Inventory, ManagesItems};
use crate::skills::{SkillType, SkillXpGained, Skills};
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

use super::components::{DungeonPlayer, DungeonRoot, TargetPosition, TileSizes};
use super::constants::ENTITY_VISUAL_SCALE;
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
                    handle_floor_ready.run_if(on_event::<FloorReady>),
                    handle_dungeon_movement,
                    handle_move_result.run_if(on_event::<MoveResult>),
                    interpolate_player_position,
                    handle_interact_action.run_if(on_event::<GameAction>),
                    handle_npc_interaction.run_if(on_event::<NpcInteraction>),
                    handle_crafting_station_interaction.run_if(on_event::<CraftingStationInteraction>),
                    handle_mining_result.run_if(on_event::<MiningResult>),
                    handle_back_action,
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

#[instrument(level = "debug", skip_all)]
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

const MOVE_INTERVAL: f32 = 0.1;
const MOVE_SPEED: f32 = 8.0;

fn handle_dungeon_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut action_reader: EventReader<GameAction>,
    mut move_events: EventWriter<PlayerMoveIntent>,
    held_direction: Res<HeldDirection>,
    active_modal: Res<ActiveModal>,
    mut move_timer: Local<f32>,
) {
    if active_modal.modal.is_some() {
        return;
    }

    let fresh_direction = action_reader.read().find_map(|a| match a {
        GameAction::Navigate(dir) => Some(*dir),
        _ => None,
    });

    if let Some(direction) = fresh_direction {
        commands.insert_resource(LastMoveDirection(direction));
        move_events.send(PlayerMoveIntent { direction });
        *move_timer = MOVE_INTERVAL;
        return;
    }

    if let Some(direction) = held_direction.0 {
        *move_timer -= time.delta_secs();
        if *move_timer <= 0.0 {
            commands.insert_resource(LastMoveDirection(direction));
            move_events.send(PlayerMoveIntent { direction });
            *move_timer = MOVE_INTERVAL;
        }
    } else {
        *move_timer = 0.0;
    }
}

fn handle_move_result(
    mut commands: Commands,
    mut events: EventReader<MoveResult>,
    last_direction: Option<Res<LastMoveDirection>>,
    tile_sizes: Res<TileSizes>,
    sheet: Res<PlayerSpriteSheet>,
    fight_mob: Option<Res<FightModalMob>>,
    mut player_query: Query<
        (&mut TargetPosition, &mut ImageNode, &mut SpriteAnimation, &mut PlayerWalkTimer),
        With<DungeonPlayer>,
    >,
) {
    for event in events.read() {
        match event {
            MoveResult::Moved { new_pos } => {
                let Ok((mut target_pos, mut player_image, mut anim, mut walk_timer)) =
                    player_query.get_single_mut()
                else {
                    continue;
                };

                let tile_size = tile_sizes.tile_size;
                let entity_sprite_size = ENTITY_VISUAL_SCALE * tile_sizes.base_tile_size;
                let entity_offset = -(entity_sprite_size - tile_size) / 2.0;
                target_pos.0 = Vec2::new(
                    new_pos.x as f32 * tile_size + entity_offset,
                    new_pos.y as f32 * tile_size + entity_offset,
                );

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
        commands.trigger(OpenModal(ModalType::ResultsModal));
    }
}

fn revert_forge_idle(
    mut commands: Commands,
    time: Res<Time>,
    game_sprites: Res<GameSprites>,
    skills: Res<Skills>,
    mut xp_events: EventWriter<SkillXpGained>,
    mut query: Query<(
        Entity,
        &mut ForgeActiveTimer,
        &mut ImageNode,
        Option<&mut ForgeCraftingState>,
    )>,
) {
    use crate::skills::blacksmith_bonus_item_chance;

    let blacksmith_level = skills
        .skill(SkillType::Blacksmith)
        .map(|s| s.level)
        .unwrap_or(1);
    let bonus_chance = blacksmith_bonus_item_chance(blacksmith_level);

    for (entity, mut timer, mut image, forge_state) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Some(mut state) = forge_state {
                let coal_qty = state.coal_slot.as_ref().map(|(_, q)| *q).unwrap_or(0);
                let ore_qty = state.ore_slot.as_ref().map(|(_, q)| *q).unwrap_or(0);
                let ingot_count = coal_qty.min(ore_qty);

                state.complete_crafting_with_bonus(bonus_chance);

                if ingot_count > 0 {
                    xp_events.send(SkillXpGained {
                        skill: SkillType::Blacksmith,
                        amount: ingot_count as u64 * 25,
                    });
                }
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
    skills: Res<Skills>,
    mut xp_events: EventWriter<SkillXpGained>,
    mut query: Query<(
        Entity,
        &mut AnvilActiveTimer,
        &mut ImageNode,
        Option<&mut AnvilCraftingState>,
    )>,
) {
    let blacksmith_level = skills
        .skill(SkillType::Blacksmith)
        .map(|s| s.level)
        .unwrap_or(1);

    for (entity, mut timer, mut image, anvil_state) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            if let Some(mut state) = anvil_state {
                if let Some(recipe_id) = state.complete_crafting() {
                    let spec = recipe_id.spec();
                    let item = spec.output.spawn_with_quality_bonus(blacksmith_level);
                    let _ = inventory.add_to_inv(item);

                    let ingredient_count: u32 = spec.ingredients.values().sum();
                    let xp_amount = 75 + (ingredient_count.saturating_sub(1) * 25);
                    xp_events.send(SkillXpGained {
                        skill: SkillType::Blacksmith,
                        amount: xp_amount as u64,
                    });
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

fn interpolate_player_position(
    time: Res<Time>,
    tile_sizes: Option<Res<TileSizes>>,
    mut query: Query<(&TargetPosition, &mut Node), With<DungeonPlayer>>,
) {
    let Some(tile_sizes) = tile_sizes else { return };

    for (target, mut node) in &mut query {
        let current_x = match node.left {
            Val::Px(px) => px,
            _ => continue,
        };
        let current_y = match node.top {
            Val::Px(px) => px,
            _ => continue,
        };
        let current = Vec2::new(current_x, current_y);
        let delta = target.0 - current;
        let distance = delta.length();

        if distance < 0.5 {
            node.left = Val::Px(target.0.x);
            node.top = Val::Px(target.0.y);
        } else {
            let speed = MOVE_SPEED * tile_sizes.tile_size;
            let step = speed * time.delta_secs();
            let new_pos = current + delta.normalize() * step.min(distance);
            node.left = Val::Px(new_pos.x);
            node.top = Val::Px(new_pos.y);
        }
    }
}
