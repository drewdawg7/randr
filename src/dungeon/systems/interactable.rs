use avian2d::prelude::*;
use bevy::prelude::*;

use crate::dungeon::constants::{DEFAULT_TILE_SIZE, INTERACTION_RADIUS_MULTIPLIER};
use crate::dungeon::events::InteractableNearby;
use crate::dungeon::{
    ChestEntity, CraftingStationEntity, GameLayer, NpcEntity, RockEntity, TileWorldSize,
};
use crate::ui::screens::DungeonPlayer;

pub fn detect_nearby_interactables(
    mut nearby: ResMut<InteractableNearby>,
    spatial_query: SpatialQuery,
    tile_size: Option<Res<TileWorldSize>>,
    player_query: Query<(Entity, &Position), With<DungeonPlayer>>,
    interactable_query: Query<
        (),
        Or<(
            With<CraftingStationEntity>,
            With<NpcEntity>,
            With<ChestEntity>,
            With<RockEntity>,
        )>,
    >,
) {
    let Ok((player_entity, &Position(player_pos))) = player_query.single() else {
        nearby.0 = None;
        return;
    };

    let tile_size = tile_size.map(|t| t.0).unwrap_or(DEFAULT_TILE_SIZE);
    let radius = tile_size * INTERACTION_RADIUS_MULTIPLIER;
    let shape = Collider::circle(radius);

    let filter =
        SpatialQueryFilter::from_mask([GameLayer::StaticEntity, GameLayer::Mob, GameLayer::Trigger])
            .with_excluded_entities([player_entity]);

    let entities = spatial_query.shape_intersections(&shape, player_pos, 0.0, &filter);

    nearby.0 = entities
        .iter()
        .find(|&&e| interactable_query.get(e).is_ok())
        .copied();
}
