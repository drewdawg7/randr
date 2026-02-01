use std::collections::HashMap;

use avian2d::prelude::{Collider, CollisionStart, Gravity, PhysicsPlugins, RigidBody, Sensor};
use bevy::color::palettes::css::RED;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{ColliderCreated, TiledEvent, TiledPhysicsAvianBackend, TiledPhysicsPlugin};
use tracing::{debug, instrument};

use crate::dungeon::config::DungeonConfig;
use crate::dungeon::events::{
    CraftingStationInteraction, FloorReady, FloorTransition, MineEntity, MiningResult, MoveResult,
    NpcInteraction, PlayerMoveIntent,
};
use crate::plugins::MobDefeated;
use crate::dungeon::floor::FloorId;
use crate::dungeon::state::DungeonState;
use crate::dungeon::systems::{
    handle_floor_transition, handle_mine_entity, handle_mob_defeated,
    handle_player_collisions, handle_player_move, on_map_created, prepare_floor,
    stop_player_when_idle, SpawnFloor,
};
use crate::dungeon::tile_components::{can_have_entity, can_spawn_player, is_door, is_solid};
use crate::location::LocationId;

#[derive(Resource, Default)]
pub struct FloorMonsterCount(pub usize);


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
        app.add_plugins(PhysicsPlugins::default().with_length_unit(32.0))
            .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
            .insert_resource(Gravity::ZERO)
            .register_type::<is_solid>()
            .register_type::<can_have_entity>()
            .register_type::<can_spawn_player>()
            .register_type::<is_door>()
            .insert_resource(self.registry.clone())
            .init_resource::<DungeonState>()
            .add_message::<FloorTransition>()
            .add_message::<FloorReady>()
            .add_message::<SpawnFloor>()
            .add_message::<PlayerMoveIntent>()
            .add_message::<MoveResult>()
            .add_message::<NpcInteraction>()
            .add_message::<CraftingStationInteraction>()
            .add_message::<MineEntity>()
            .add_message::<MiningResult>()
            .add_observer(on_map_created)
            .add_observer(on_collider_created)
            .add_systems(
                Update,
                (
                    prepare_floor.run_if(on_message::<SpawnFloor>),
                    handle_player_move.run_if(on_message::<PlayerMoveIntent>),
                    stop_player_when_idle,
                    handle_player_collisions.run_if(on_message::<CollisionStart>),
                    handle_floor_transition.run_if(on_message::<FloorTransition>),
                    handle_mine_entity.run_if(on_message::<MineEntity>),
                    handle_mob_defeated.run_if(on_message::<MobDefeated>),
                    debug_draw_colliders,
                ),
            );
    }
}

impl DungeonPlugin {
    pub fn new() -> DungeonBuilder {
        DungeonBuilder {
            configs: HashMap::new(),
            current_location: None,
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

pub struct DungeonBuilder {
    configs: HashMap<LocationId, DungeonConfig>,
    current_location: Option<LocationId>,
}

impl DungeonBuilder {
    pub fn location(mut self, id: LocationId) -> Self {
        self.current_location = Some(id);
        self.configs
            .entry(id)
            .or_insert(DungeonConfig::new(Vec::new()));
        self
    }

    pub fn floor(mut self, floor: FloorId) -> Self {
        let location = self
            .current_location
            .expect("floor() called before location()");
        if let Some(config) = self.configs.get_mut(&location) {
            let mut floors = config.floors().to_vec();
            floors.push(floor);
            *config = DungeonConfig::new(floors);
        }
        self
    }

    pub fn build(self) -> DungeonPlugin {
        assert!(
            !self.configs.is_empty(),
            "DungeonPlugin requires at least one location to be registered"
        );

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

    commands.entity(collider_entity).insert(RigidBody::Static);
}

fn debug_draw_colliders(
    colliders: Query<(&GlobalTransform, &Collider), With<RigidBody>>,
    mut gizmos: Gizmos,
) {
    for (transform, collider) in &colliders {
        let pos = transform.translation().truncate();
        let aabb = collider.shape_scaled().compute_local_aabb();
        let half_extents = Vec2::new(
            (aabb.maxs.x - aabb.mins.x) / 2.0,
            (aabb.maxs.y - aabb.mins.y) / 2.0,
        );
        let center_offset = Vec2::new(
            (aabb.maxs.x + aabb.mins.x) / 2.0,
            (aabb.maxs.y + aabb.mins.y) / 2.0,
        );
        gizmos.rect_2d(
            Isometry2d::from_translation(pos + center_offset),
            half_extents * 2.0,
            RED,
        );
    }
}
