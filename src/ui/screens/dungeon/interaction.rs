use bevy::prelude::*;
use tracing::instrument;

use crate::crafting_station::{AnvilActiveTimer, CraftingStationType, ForgeActiveTimer};
use crate::dungeon::{
    ChestEntity, ChestMined, CraftingStationEntity, CraftingStationInteraction,
    DungeonEntityMarker, InteractableNearby, MerchantInteraction, MineableEntityType,
    MiningResult, NpcEntity, RockEntity, RockMined,
};
use crate::mob::MobId;
use crate::ui::screens::anvil_modal::ActiveAnvilEntity;
use crate::ui::screens::forge_modal::ActiveForgeEntity;
use crate::ui::screens::modal::{ModalType, OpenModal};
use crate::ui::screens::results_modal::ResultsModalData;

#[instrument(level = "debug", skip_all)]
pub fn process_interaction(
    mut commands: Commands,
    mut action_reader: MessageReader<crate::input::GameAction>,
    mut crafting_events: MessageWriter<CraftingStationInteraction>,
    nearby: Res<InteractableNearby>,
    marker_query: Query<&DungeonEntityMarker>,
    npc_query: Query<&NpcEntity>,
    crafting_query: Query<&CraftingStationEntity>,
    chest_query: Query<(), With<ChestEntity>>,
    rock_query: Query<&RockEntity>,
) {
    let is_interact = action_reader
        .read()
        .any(|a| *a == crate::input::GameAction::Interact);
    if !is_interact {
        return;
    }

    let Some(entity) = nearby.0 else {
        return;
    };

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
        return;
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
