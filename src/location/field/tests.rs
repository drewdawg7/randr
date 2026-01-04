#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
use std::sync::Once;

#[cfg(test)]
use crate::{
    entities::mob::MobId,
    location::{Field, FieldData, FieldError, LocationId, LocationSpec},
    player::Player,
    init_game_state, GameState,
};

#[cfg(test)]
static INIT: Once = Once::new();

#[cfg(test)]
fn initialize_game_state() {
    INIT.call_once(|| {
        // Initialize game state for tests that need spawn_mob()
        // We use GameState::default() which will create a terminal
        // This is acceptable for integration tests
        init_game_state(GameState::default());
    });
}

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
        location_id: LocationId::VillageField,
        name: "Village Field",
        description: "A peaceful field near the village",
        refresh_interval: None,
        min_level: None,
        data: crate::location::LocationData::Field(data.clone()),
    };

    let field = Field::from_spec(&spec, &data);

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
        location_id: LocationId::VillageField,
        name: "Test Field",
        description: "A test description",
        refresh_interval: None,
        min_level: None,
        data: crate::location::LocationData::Field(data.clone()),
    };

    let field = Field::from_spec(&spec, &data);

    assert_eq!(field.description(), "A test description");
}

#[test]
fn field_description_returns_empty_string_for_new() {
    let mob_weights = HashMap::new();
    let field = Field::new("Test".to_string(), mob_weights);

    assert_eq!(field.description(), "");
}

// ==================== Field::spawn_mob() edge cases ====================

#[test]
fn field_spawn_mob_returns_error_when_total_weight_is_zero() {
    initialize_game_state();

    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 0);
    mob_weights.insert(MobId::Goblin, 0);

    let field = Field::new("Zero Weight Field".to_string(), mob_weights);
    let player = Player::default();

    let result = field.spawn_mob(&player);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), FieldError::MobSpawnError));
}

#[test]
fn field_spawn_mob_returns_error_when_mob_weights_empty() {
    initialize_game_state();

    let mob_weights = HashMap::new();

    let field = Field::new("Empty Field".to_string(), mob_weights);
    let player = Player::default();

    let result = field.spawn_mob(&player);

    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), FieldError::MobSpawnError));
}

#[test]
fn field_spawn_mob_succeeds_with_single_mob_type() {
    initialize_game_state();

    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 100);

    let field = Field::new("Single Mob Field".to_string(), mob_weights);
    let player = Player::default();

    // Run multiple times to ensure it always spawns
    for _ in 0..10 {
        let result = field.spawn_mob(&player);
        assert!(result.is_ok(), "Expected successful spawn with single mob type");
    }
}

#[test]
fn field_spawn_mob_succeeds_with_multiple_mobs_equal_weights() {
    initialize_game_state();

    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 50);
    mob_weights.insert(MobId::Goblin, 50);

    let field = Field::new("Equal Weight Field".to_string(), mob_weights);
    let player = Player::default();

    // Run multiple times to ensure spawning works
    for _ in 0..10 {
        let result = field.spawn_mob(&player);
        assert!(result.is_ok(), "Expected successful spawn with equal weights");
    }
}

#[test]
fn field_spawn_mob_succeeds_with_multiple_mobs_unequal_weights() {
    initialize_game_state();

    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 80);
    mob_weights.insert(MobId::Goblin, 20);

    let field = Field::new("Unequal Weight Field".to_string(), mob_weights);
    let player = Player::default();

    // Run multiple times to ensure spawning works
    for _ in 0..10 {
        let result = field.spawn_mob(&player);
        assert!(result.is_ok(), "Expected successful spawn with unequal weights");
    }
}

// ==================== Statistical distribution tests ====================

#[test]
fn field_spawn_mob_respects_weight_distribution_single_mob() {
    initialize_game_state();

    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 100);

    let field = Field::new("Single Mob Field".to_string(), mob_weights);
    let player = Player::default();

    let iterations = 100;
    let mut slime_count = 0;

    for _ in 0..iterations {
        if let Ok(mob) = field.spawn_mob(&player) {
            if mob.spec == MobId::Slime {
                slime_count += 1;
            }
        }
    }

    // With only one mob type, all spawns should be that type
    assert_eq!(slime_count, iterations);
}

#[test]
fn field_spawn_mob_respects_weight_distribution_equal_weights() {
    initialize_game_state();

    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 50);
    mob_weights.insert(MobId::Goblin, 50);

    let field = Field::new("Equal Weight Field".to_string(), mob_weights);
    let player = Player::default();

    let iterations = 1000;
    let mut slime_count = 0;
    let mut goblin_count = 0;

    for _ in 0..iterations {
        if let Ok(mob) = field.spawn_mob(&player) {
            match mob.spec {
                MobId::Slime => slime_count += 1,
                MobId::Goblin => goblin_count += 1,
                _ => {}
            }
        }
    }

    let total_spawned = slime_count + goblin_count;
    assert_eq!(total_spawned, iterations, "All spawns should succeed");

    // With equal weights (50/50), expect roughly 50% each (±10%)
    let expected = iterations / 2;
    let tolerance = (iterations as f64 * 0.10) as i32;

    assert!(
        (slime_count as i32 - expected as i32).abs() <= tolerance,
        "Slime count {} should be within {}±{} (got {})",
        slime_count,
        expected,
        tolerance,
        slime_count
    );

    assert!(
        (goblin_count as i32 - expected as i32).abs() <= tolerance,
        "Goblin count {} should be within {}±{} (got {})",
        goblin_count,
        expected,
        tolerance,
        goblin_count
    );
}

#[test]
fn field_spawn_mob_respects_weight_distribution_unequal_weights() {
    initialize_game_state();

    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 70); // 70% expected
    mob_weights.insert(MobId::Goblin, 30); // 30% expected

    let field = Field::new("Unequal Weight Field".to_string(), mob_weights);
    let player = Player::default();

    let iterations = 1000;
    let mut slime_count = 0;
    let mut goblin_count = 0;

    for _ in 0..iterations {
        if let Ok(mob) = field.spawn_mob(&player) {
            match mob.spec {
                MobId::Slime => slime_count += 1,
                MobId::Goblin => goblin_count += 1,
                _ => {}
            }
        }
    }

    let total_spawned = slime_count + goblin_count;
    assert_eq!(total_spawned, iterations, "All spawns should succeed");

    // Slime should be ~70% (±10%)
    let slime_expected = (iterations as f64 * 0.70) as i32;
    let slime_tolerance = (iterations as f64 * 0.10) as i32;

    assert!(
        (slime_count as i32 - slime_expected).abs() <= slime_tolerance,
        "Slime count {} should be within {}±{} (got {})",
        slime_count,
        slime_expected,
        slime_tolerance,
        slime_count
    );

    // Goblin should be ~30% (±10%)
    let goblin_expected = (iterations as f64 * 0.30) as i32;
    let goblin_tolerance = (iterations as f64 * 0.10) as i32;

    assert!(
        (goblin_count as i32 - goblin_expected).abs() <= goblin_tolerance,
        "Goblin count {} should be within {}±{} (got {})",
        goblin_count,
        goblin_expected,
        goblin_tolerance,
        goblin_count
    );
}

#[test]
fn field_spawn_mob_respects_weight_distribution_three_mobs() {
    initialize_game_state();

    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, 50); // 50% expected
    mob_weights.insert(MobId::Goblin, 30); // 30% expected
    mob_weights.insert(MobId::Cow, 20); // 20% expected

    let field = Field::new("Three Mob Field".to_string(), mob_weights);
    let player = Player::default();

    let iterations = 1000;
    let mut slime_count = 0;
    let mut goblin_count = 0;
    let mut cow_count = 0;

    for _ in 0..iterations {
        if let Ok(mob) = field.spawn_mob(&player) {
            match mob.spec {
                MobId::Slime => slime_count += 1,
                MobId::Goblin => goblin_count += 1,
                MobId::Cow => cow_count += 1,
                _ => {}
            }
        }
    }

    let total_spawned = slime_count + goblin_count + cow_count;
    assert_eq!(total_spawned, iterations, "All spawns should succeed");

    // Check each distribution (±10% tolerance)
    let tolerance_percent = 0.10;

    // Slime should be ~50%
    let slime_expected = (iterations as f64 * 0.50) as i32;
    let slime_tolerance = (iterations as f64 * tolerance_percent) as i32;
    assert!(
        (slime_count as i32 - slime_expected).abs() <= slime_tolerance,
        "Slime count {} should be within {}±{}",
        slime_count,
        slime_expected,
        slime_tolerance
    );

    // Goblin should be ~30%
    let goblin_expected = (iterations as f64 * 0.30) as i32;
    let goblin_tolerance = (iterations as f64 * tolerance_percent) as i32;
    assert!(
        (goblin_count as i32 - goblin_expected).abs() <= goblin_tolerance,
        "Goblin count {} should be within {}±{}",
        goblin_count,
        goblin_expected,
        goblin_tolerance
    );

    // Cow should be ~20%
    let cow_expected = (iterations as f64 * 0.20) as i32;
    let cow_tolerance = (iterations as f64 * tolerance_percent) as i32;
    assert!(
        (cow_count as i32 - cow_expected).abs() <= cow_tolerance,
        "Cow count {} should be within {}±{}",
        cow_count,
        cow_expected,
        cow_tolerance
    );
}

// ==================== Edge case: negative weights (shouldn't happen but test system behavior) ====================

#[test]
fn field_spawn_mob_handles_negative_weights_gracefully() {
    initialize_game_state();

    // The spawn_mob method applies max(0) to adjusted weights from passive effects,
    // but base weights should be non-negative. This tests the edge case.
    let mut mob_weights = HashMap::new();
    mob_weights.insert(MobId::Slime, -10);
    mob_weights.insert(MobId::Goblin, 50);

    let field = Field::new("Negative Weight Field".to_string(), mob_weights);
    let player = Player::default();

    // With negative weight, total = -10 + 50 = 40
    // The weighted_select should still work (treating -10 as a weird edge case)
    // In practice this shouldn't happen, but the system handles it
    let result = field.spawn_mob(&player);

    // System behavior: weighted_select may behave unexpectedly with negative weights,
    // but it won't crash. Most likely only Goblin will spawn since Slime has negative weight.
    // We just verify it doesn't panic.
    let _ = result;
}
