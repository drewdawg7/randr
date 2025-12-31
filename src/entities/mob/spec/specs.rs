use once_cell::sync::Lazy;

use crate::entities::mob::enums::MobQuality;

use super::definition::MobSpec;

pub static SLIME: Lazy<MobSpec> = Lazy::new(|| MobSpec {
    name: "Slime",
    quality: MobQuality::Normal,
    max_health: 8..=12,
    attack: 2..=4,
    dropped_gold: 1..=3,
    dropped_xp: 5..=9,
});

pub static GOBLIN: Lazy<MobSpec> = Lazy::new(|| MobSpec {
    name: "Goblin",
    quality: MobQuality::Normal,
    max_health: 33..=41,
    attack: 4..=6,
    dropped_gold: 5..=7,
    dropped_xp: 13..=20,
});

pub static DRAGON: Lazy<MobSpec> = Lazy::new(|| MobSpec {
    name: "Dragon",
    quality: MobQuality::Boss,
    max_health: 500..=700,
    attack: 25..=30,
    dropped_gold: 250..=350,
    dropped_xp: 500..=750,
});
