use std::ops::RangeInclusive;

use crate::registry::Registry;

use super::super::enums::{MobKind, MobQuality};

#[derive(Clone)]
pub struct MobSpec {
    pub name: &'static str,
    pub max_health: RangeInclusive<i32>,
    pub attack: RangeInclusive<i32>,
    pub dropped_gold: RangeInclusive<i32>,
    pub dropped_xp: RangeInclusive<i32>,
    pub quality: MobQuality,
}

pub type MobRegistry = Registry<MobKind, MobSpec>;
