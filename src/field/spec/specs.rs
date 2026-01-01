use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{entities::mob::MobKind, field::FieldId};

use super::definition::FieldSpec;

pub static VILLAGE_FIELD: Lazy<FieldSpec> = Lazy::new(|| FieldSpec {
    field_id: FieldId::VillageField,
    name: "The Village Field",
    mob_weights: HashMap::from([
        (MobKind::Slime, 5),
        (MobKind::Goblin, 5),
        (MobKind::Dragon, 1),
    ]),
});
