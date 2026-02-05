use std::collections::HashMap;

use avian2d::prelude::{Collider, CollisionEnd, CollisionStart, Gravity, PhysicsPlugins, RigidBody, Sensor};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{ColliderCreated, TiledEvent, TiledPhysicsAvianBackend, TiledPhysicsPlugin};
use tracing::{debug, instrument};

use crate::dungeon::config::DungeonConfig;
use crate::dungeon::events::{
    CraftingStationInteraction, FloorReady, FloorTransition, MiningResult, MoveResult,
    OverlappingCraftingStation, PlayerMoveIntent,
};
use crate::plugins::MobDefeated;
use crate::dungeon::floor::FloorId;
use crate::dungeon::state::{DungeonState, MovementConfig, TileWorldSize};
use crate::combat::Attacking;
use crate::dungeon::systems::{
    cleanup_mob_health_bar, handle_floor_transition, handle_mob_defeated,
    handle_player_collision_end, handle_player_collisions, handle_player_move, prepare_floor,
    spawn_mob_health_bars, stop_attacking_player, stop_player_when_idle,
    update_mob_health_bar_positions, update_mob_health_bar_values, SpawnFloor,
};
use crate::dungeon::tile_components::{can_have_entity, can_spawn_player, is_door, is_solid};
use crate::location::LocationId;
use crate::states::AppState;

#[derive(Resource, Default)]
pub struct FloorMonsterCount(pub usize);

#[derive(Component)]
pub struct TiledWallCollider;

#[derive(Resource, Clone, Debug)]
pub struct DungeonRegistry {
    configs: HashMap<LocationId, DungeonConfig>,
}

impl DungeonRegistry {
    pub fn config(&self, location: LocationId) -> Option<&DungeonConfig> {
        self.configs.get(&location)
    }

    pub fn floors(&self, location: LocationId) -> &[FloorId] {
        self.configs
            .get(&location)
            .map(|c| c.floors())
            .unwrap_or(&[])
    }

    pub fn next_floor(&self, location: LocationId, current: FloorId) -> Option<FloorId> {
        let floors = self.floors(location);
        floors
            .iter()
            .position(|&f| f == current)
            .and_then(|idx| floors.get(idx + 1))
            .copied()
    }

    pub fn is_final_floor(&self, location: LocationId, floor: FloorId) -> bool {
        let floors = self.floors(location);
        floors.last() == Some(&floor)
    }
}

pub struct DungeonPlugin {
    registry: DungeonRegistry,
}

impl Plugin for DungeonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default().with_length_unit(TileWorldSize::default().0))
            .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
            .insert_resource(Gravity::ZERO)
            .register_type::<is_solid>()
            .register_type::<can_have_entity>()
            .register_type::<can_spawn_player>()
            .register_type::<is_door>()
            .insert_resource(self.registry.clone())
            .init_resource::<DungeonState>()
            .init_resource::<TileWorldSize>()
            .init_resource::<MovementConfig>()
            .init_resource::<OverlappingCraftingStation>()
            .add_message::<FloorTransition>()
            .add_message::<FloorReady>()
            .add_message::<SpawnFloor>()
            .add_message::<PlayerMoveIntent>()
            .add_message::<MoveResult>()
            .add_message::<CraftingStationInteraction>()
            .add_message::<MiningResult>()
            .add_observer(on_collider_created)
            .add_observer(cleanup_mob_health_bar)
            .add_systems(
                FixedPreUpdate,
                (
                    handle_player_move
                        .run_if(on_message::<PlayerMoveIntent>)
                        .run_if(not(any_with_component::<Attacking>)),
                    stop_player_when_idle.run_if(not(any_with_component::<Attacking>)),
                    stop_attacking_player.run_if(any_with_component::<Attacking>),
                )
                    .run_if(in_state(AppState::Dungeon)),
            )
            .add_systems(
                Update,
                (
                    prepare_floor.run_if(on_message::<SpawnFloor>),
                    handle_player_collisions.run_if(on_message::<CollisionStart>),
                    handle_player_collision_end.run_if(on_message::<CollisionEnd>),
                    handle_floor_transition.run_if(on_message::<FloorTransition>),
                    handle_mob_defeated.run_if(on_message::<MobDefeated>),
                    spawn_mob_health_bars,
                    update_mob_health_bar_positions,
                    update_mob_health_bar_values,
                )
                    .run_if(in_state(AppState::Dungeon)),
            );
    }
}

impl DungeonPlugin {
    pub fn new() -> DungeonBuilder<NoLocation> {
        DungeonBuilder {
            configs: HashMap::new(),
            state: NoLocation,
        }
    }
}

impl Default for DungeonPlugin {
    fn default() -> Self {
        Self {
            registry: DungeonRegistry {
                configs: HashMap::new(),
            },
        }
    }
}

pub struct NoLocation;
pub struct HasLocation(LocationId);

pub struct DungeonBuilder<S = NoLocation> {
    configs: HashMap<LocationId, DungeonConfig>,
    state: S,
}

impl DungeonBuilder<NoLocation> {
    pub fn location(mut self, id: LocationId) -> DungeonBuilder<HasLocation> {
        self.configs
            .entry(id)
            .or_insert(DungeonConfig::new(Vec::new()));
        DungeonBuilder {
            configs: self.configs,
            state: HasLocation(id),
        }
    }
}

impl DungeonBuilder<HasLocation> {
    pub fn location(mut self, id: LocationId) -> Self {
        self.configs
            .entry(id)
            .or_insert(DungeonConfig::new(Vec::new()));
        self.state = HasLocation(id);
        self
    }

    pub fn floor(mut self, floor: FloorId) -> Self {
        let location = self.state.0;
        if let Some(config) = self.configs.get_mut(&location) {
            let mut floors = config.floors().to_vec();
            floors.push(floor);
            *config = DungeonConfig::new(floors);
        }
        self
    }

    pub fn build(self) -> DungeonPlugin {
        DungeonPlugin {
            registry: DungeonRegistry {
                configs: self.configs,
            },
        }
    }
}

#[instrument(level = "debug", skip_all)]
fn on_collider_created(
    trigger: On<TiledEvent<ColliderCreated>>,
    mut commands: Commands,
    parent_query: Query<&ChildOf>,
    door_query: Query<&is_door>,
    collider_query: Query<(&Collider, &GlobalTransform)>,
) {
    let collider_entity = trigger.event().origin;

    if let Ok((collider, transform)) = collider_query.get(collider_entity) {
        let pos = transform.translation();
        let shape = collider.shape_scaled();
        let aabb = shape.compute_local_aabb();
        debug!(
            entity = ?collider_entity,
            pos_x = pos.x,
            pos_y = pos.y,
            aabb_min_x = aabb.mins.x,
            aabb_min_y = aabb.mins.y,
            aabb_max_x = aabb.maxs.x,
            aabb_max_y = aabb.maxs.y,
            "wall collider created"
        );
    }

    if let Ok(child_of) = parent_query.get(collider_entity) {
        if door_query.get(child_of.parent()).is_ok() {
            commands.entity(collider_entity).insert((Sensor, is_door(true)));
            return;
        }
    }

    commands.entity(collider_entity).insert((RigidBody::Static, TiledWallCollider));
}
