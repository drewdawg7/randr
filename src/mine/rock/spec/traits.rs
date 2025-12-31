use crate::{
    mine::rock::spec::specs::MIXED_ROCK, registry::{RegistryDefaults, SpawnFromSpec}, stats::{StatSheet, StatType}
};

use super::super::{Rock, RockId};
use super::definition::RockSpec;
use super::specs::{COAL_ROCK, COPPER_ROCK, TIN_ROCK};

impl SpawnFromSpec<RockId> for RockSpec {
    type Output = Rock;

    fn spawn_from_spec(_kind: RockId, spec: &Self) -> Self::Output {
        Rock {
            rock_id: spec.rock_id,
            stats: StatSheet::new().with(StatType::Health, spec.health),
            loot: spec.loot.clone(),
        }
    }
}

impl RegistryDefaults<RockId> for RockSpec {
    fn defaults() -> impl IntoIterator<Item = (RockId, Self)> {
        [
            (RockId::Copper, COPPER_ROCK.clone()),
            (RockId::Coal, COAL_ROCK.clone()),
            (RockId::Tin, TIN_ROCK.clone()),
            (RockId::Mixed, MIXED_ROCK.clone()),
        ]
    }
}
