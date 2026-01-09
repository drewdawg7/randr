#[cfg(test)]
use std::collections::HashMap;

#[cfg(test)]
use crate::{
    mob::MobId,
    location::{Field, FieldData, LocationId, LocationSpec},
};

// ==================== Field::new() tests ====================

#[test]
fn field_new_creates_with_name_and_mob_weights() {
    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 50);
    mob_weights.insert(MobId::Goblin, 30);

    let field = Field::new("Test Field".to_string(), mob_weights.clone());

    assert_eq!(field.name, "Test Field");
    assert_eq!(field.mob_weights.len(), 2);
    assert_eq!(field.mob_weights.get(&MobId::Slime), Some(&50));
    assert_eq!(field.mob_weights.get(&MobId::Goblin), Some(&30));
}

#[test]
fn field_new_sets_default_location_id() {
    let mob_weights = HashMap::new();
    let field = Field::new("Test Field".to_string(), mob_weights);

    assert_eq!(field.location_id, LocationId::VillageField);
}

#[test]
fn field_new_creates_empty_description() {
    let mob_weights = HashMap::new();
    let field = Field::new("Test Field".to_string(), mob_weights);

    assert_eq!(field.description, "");
}

#[test]
fn field_new_accepts_empty_mob_weights() {
    let mob_weights = HashMap::new();
    let field = Field::new("Empty Field".to_string(), mob_weights);

    assert_eq!(field.mob_weights.len(), 0);
}

// ==================== Field::from_spec() tests ====================

#[test]
fn field_from_spec_creates_from_location_spec() {
    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 60);
    mob_weights.insert(MobId::Cow, 40);

    let data = FieldData {
        mob_weights: mob_weights.clone(),
    };

    let spec = LocationSpec {
        name: "Village Field",
        description: "A peaceful field near the village",
        refresh_interval: None,
        min_level: None,
        data: crate::location::LocationData::Field(data.clone()),
    };

    let field = Field::from_spec(LocationId::VillageField, &spec, &data);

    assert_eq!(field.location_id, LocationId::VillageField);
    assert_eq!(field.name, "Village Field");
    assert_eq!(field.description, "A peaceful field near the village");
    assert_eq!(field.mob_weights.len(), 2);
    assert_eq!(field.mob_weights.get(&MobId::Slime), Some(&60));
    assert_eq!(field.mob_weights.get(&MobId::Cow), Some(&40));
}

// ==================== Field::location_id() tests ====================

#[test]
fn field_location_id_returns_correct_id() {
    let mob_weights = HashMap::new();
    let field = Field::new("Test".to_string(), mob_weights);

    assert_eq!(field.location_id(), LocationId::VillageField);
}

// ==================== Field::description() tests ====================

#[test]
fn field_description_returns_description() {
    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 50);

    let data = FieldData {
        mob_weights: mob_weights.clone(),
    };

    let spec = LocationSpec {
        name: "Test Field",
        description: "A test description",
        refresh_interval: None,
        min_level: None,
        data: crate::location::LocationData::Field(data.clone()),
    };

    let field = Field::from_spec(LocationId::VillageField, &spec, &data);

    assert_eq!(field.description(), "A test description");
}

#[test]
fn field_description_returns_empty_string_for_new() {
    let mob_weights = HashMap::new();
    let field = Field::new("Test".to_string(), mob_weights);

    assert_eq!(field.description(), "");
}

// Note: spawn_mob tests have been removed as they depend on the old global game state
// system that was replaced during the Bevy rewrite. These tests should be rewritten
// to work with Bevy's ECS test utilities.
