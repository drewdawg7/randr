use std::collections::HashMap;

use crate::{
    location::{FieldData, LocationId, LocationSpec},
    mob::{Mob, MobId},
    utils::weighted_select,
};

use super::enums::FieldError;

pub struct Field {
    pub(crate) location_id: LocationId,
    pub name: String,
    pub(crate) description: String,
    pub mob_weights: HashMap<MobId, i32>,
}

impl Field {
    /// Create a Field from a LocationSpec
    pub fn from_spec(location_id: LocationId, spec: &LocationSpec, data: &FieldData) -> Self {
        Field {
            location_id,
            name: spec.name.to_string(),
            description: spec.description.to_string(),
            mob_weights: data.mob_weights.clone(),
        }
    }

    pub fn new(name: String, mob_weights: HashMap<MobId, i32>) -> Self {
        Self {
            location_id: LocationId::VillageField,
            name,
            description: String::new(),
            mob_weights,
        }
    }

    pub fn spawn_mob(&self) -> Result<Mob, FieldError> {
        weighted_select(&self.mob_weights)
            .map(|mob_id| mob_id.spawn())
            .ok_or(FieldError::MobSpawnError)
    }

    // Location trait accessors
    pub fn location_id(&self) -> LocationId {
        self.location_id
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
