use crate::{
    field::FieldId,
    location::Field,
    registry::{RegistryDefaults, SpawnFromSpec},
};

use super::definition::FieldSpec;
use super::specs::VILLAGE_FIELD;

impl SpawnFromSpec<FieldId> for FieldSpec {
    type Output = Field;

    fn spawn_from_spec(_kind: FieldId, spec: &Self) -> Self::Output {
        Field::new(spec.name.to_string(), spec.mob_weights.clone())
    }
}

impl RegistryDefaults<FieldId> for FieldSpec {
    fn defaults() -> impl IntoIterator<Item = (FieldId, Self)> {
        [(FieldId::VillageField, VILLAGE_FIELD.clone())]
    }
}
