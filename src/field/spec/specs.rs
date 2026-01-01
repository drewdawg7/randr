use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{entities::mob::MobId, field::FieldId};

use super::definition::FieldSpec;

pub static VILLAGE_FIELD: Lazy<FieldSpec> = Lazy::new(|| FieldSpec {
    field_id: FieldId::VillageField,
    name: "The Village Field",
    mob_weights: HashMap::from([
        (MobId::Slime, 5),
        (MobId::Cow, 5),
        (MobId::Goblin, 3),
        (MobId::Dragon, 1),
    ]),
});
