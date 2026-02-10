use avian2d::prelude::*;
use bevy::prelude::*;
use tracing::instrument;

use crate::crafting_station::{AnvilActiveTimer, CraftingStationType, ForgeActiveTimer};
use crate::dungeon::{
    ChestEntity, ChestMined, CraftingStationEntity, CraftingStationInteraction,
    DungeonEntityMarker, GameLayer, MerchantInteraction, MineableEntityType, MiningResult,
    NpcEntity, RockEntity, RockMined, TileWorldSize,
};
use crate::mob::MobId;
use crate::ui::screens::anvil_modal::ActiveAnvilEntity;
use crate::ui::screens::forge_modal::ActiveForgeEntity;
use crate::ui::screens::modal::{ModalType, OpenModal};
use crate::ui::screens::results_modal::ResultsModalData;

use super::components::DungeonPlayer;

#[instrument(level = "debug", skip_all)]
pub fn process_interaction(
    mut commands: Commands,
    mut action_reader: MessageReader<crate::input::GameAction>,
    mut crafting_events: MessageWriter<CraftingStationInteraction>,
    tile_size: Option<Res<TileWorldSize>>,
    spatial_query: SpatialQuery,
    marker_query: Query<&DungeonEntityMarker>,
    npc_query: Query<&NpcEntity>,
    crafting_query: Query<&CraftingStationEntity>,
    chest_query: Query<(), With<ChestEntity>>,
    rock_query: Query<&RockEntity>,
    player_query: Query<(Entity, &Position), With<DungeonPlayer>>,
) {
    let is_interact = action_reader
        .read()
        .any(|a| *a == crate::input::GameAction::Interact);
    if !is_interact {
        return;
    }

    let Ok((player_entity, &Position(player_pos))) = player_query.single() else {
        return;
    };

    let tile_size = tile_size
        .map(|t| t.0)
        .unwrap_or(crate::dungeon::constants::DEFAULT_TILE_SIZE);
    let interaction_radius = tile_size * crate::dungeon::constants::INTERACTION_RADIUS_MULTIPLIER;
    let interaction_shape = Collider::circle(interaction_radius);

    let filter = SpatialQueryFilter::from_mask([GameLayer::StaticEntity, GameLayer::Mob, GameLayer::Trigger])
        .with_excluded_entities([player_entity]);

    let nearby_entities = spatial_query.shape_intersections(&interaction_shape, player_pos, 0.0, &filter);

    for entity in nearby_entities {
        if let Ok(crafting) = crafting_query.get(entity) {
            crafting_events.write(CraftingStationInteraction {
                entity,
                station_type: crafting.station_type,
            });
            return;
        }

        if let Ok(npc) = npc_query.get(entity) {
            if npc.mob_id == MobId::Merchant {
                commands.trigger(MerchantInteraction { entity });
            }
            return;
        }

        let Ok(marker) = marker_query.get(entity) else {
            continue;
        };

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

#[instrument(level = "debug", skip_all, fields(event_count = events.len()))]
pub fn open_crafting_modal(
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

pub fn show_mining_results(mut commands: Commands, mut events: MessageReader<MiningResult>) {
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
