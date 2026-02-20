use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

use crate::dungeon::EntitySize;
use crate::loot::LootTable;
use crate::registry::RegistryDefaults;

pub use super::enums::MobQuality;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum MobId {
    Slime,
    Goblin,
    DwarfDefender,
    DwarfWarrior,
    DwarfMiner,
    DwarfKing,
    Merchant,
}

impl MobId {
    pub const ALL: &'static [MobId] = &[
        MobId::Slime,
        MobId::Goblin,
        MobId::DwarfDefender,
        MobId::DwarfWarrior,
        MobId::DwarfMiner,
        MobId::DwarfKing,
        MobId::Merchant,
    ];

    pub fn spec(&self) -> &'static MobSpec {
        super::data::get_spec(*self)
    }
}

#[derive(Debug, Clone)]
pub struct MobSpec {
    pub name: String,
    pub max_health: RangeInclusive<i32>,
    pub attack: RangeInclusive<i32>,
    pub defense: RangeInclusive<i32>,
    pub dropped_gold: RangeInclusive<i32>,
    pub dropped_xp: RangeInclusive<i32>,
    pub quality: MobQuality,
    pub loot: LootTable,
    pub entity_size: EntitySize,
}

impl MobSpec {
    pub fn with_multiplier(&self, multiplier: f32) -> MobSpec {
        let scale_range = |r: &RangeInclusive<i32>| {
            let start = (*r.start() as f32 * multiplier).round() as i32;
            let end = (*r.end() as f32 * multiplier).round() as i32;
            start..=end
        };

        MobSpec {
            name: self.name.clone(),
            max_health: scale_range(&self.max_health),
            attack: scale_range(&self.attack),
            defense: scale_range(&self.defense),
            dropped_gold: scale_range(&self.dropped_gold),
            dropped_xp: scale_range(&self.dropped_xp),
            quality: self.quality.clone(),
            loot: self.loot.clone(),
            entity_size: self.entity_size,
        }
    }

    pub fn with_name(&self, name: impl Into<String>) -> MobSpec {
        MobSpec {
            name: name.into(),
            max_health: self.max_health.clone(),
            attack: self.attack.clone(),
            defense: self.defense.clone(),
            dropped_gold: self.dropped_gold.clone(),
            dropped_xp: self.dropped_xp.clone(),
            quality: self.quality.clone(),
            loot: self.loot.clone(),
            entity_size: self.entity_size,
        }
    }

    pub fn with_quality(&self, quality: MobQuality) -> MobSpec {
        MobSpec {
            name: self.name.clone(),
            max_health: self.max_health.clone(),
            attack: self.attack.clone(),
            defense: self.defense.clone(),
            dropped_gold: self.dropped_gold.clone(),
            dropped_xp: self.dropped_xp.clone(),
            quality,
            loot: self.loot.clone(),
            entity_size: self.entity_size,
        }
    }
}

impl RegistryDefaults<MobId> for MobSpec {
    fn defaults() -> impl IntoIterator<Item = (MobId, Self)> {
        MobId::ALL.iter().map(|id| (*id, id.spec().clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        if crate::mob::data::specs_loaded() {
            return;
        }
        crate::item::data::init();
        crate::mob::data::init();
    }

    #[test]
    fn mob_id_spec_returns_valid_spec() {
        init();
        let spec = MobId::Goblin.spec();
        assert_eq!(spec.name, "Goblin");
    }

    #[test]
    fn mob_spec_with_multiplier_scales_stats() {
        init();
        let base = MobId::Goblin.spec();
        let scaled = base.with_multiplier(2.0);

        assert_eq!(*scaled.max_health.start(), (*base.max_health.start() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.max_health.end(), (*base.max_health.end() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.attack.start(), (*base.attack.start() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.attack.end(), (*base.attack.end() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.defense.start(), (*base.defense.start() as f32 * 2.0).round() as i32);
        assert_eq!(*scaled.defense.end(), (*base.defense.end() as f32 * 2.0).round() as i32);
    }

    #[test]
    fn mob_spec_with_multiplier_preserves_name() {
        init();
        let base = MobId::Goblin.spec();
        let scaled = base.with_multiplier(1.5);
        assert_eq!(scaled.name, "Goblin");
    }

    #[test]
    fn mob_spec_with_multiplier_preserves_quality() {
        init();
        let base = MobId::DwarfKing.spec();
        let scaled = base.with_multiplier(1.5);
        assert!(matches!(scaled.quality, MobQuality::Normal));
    }

    #[test]
    fn mob_spec_with_name_changes_name() {
        init();
        let base = MobId::Goblin.spec();
        let renamed = base.with_name("Elite Goblin");
        assert_eq!(renamed.name, "Elite Goblin");
    }

    #[test]
    fn mob_spec_with_name_preserves_stats() {
        init();
        let base = MobId::Goblin.spec();
        let renamed = base.with_name("Elite Goblin");
        assert_eq!(renamed.max_health, base.max_health);
        assert_eq!(renamed.attack, base.attack);
        assert_eq!(renamed.defense, base.defense);
    }

    #[test]
    fn mob_spec_with_quality_changes_quality() {
        init();
        let base = MobId::Goblin.spec();
        let boss = base.with_quality(MobQuality::Boss);
        assert!(matches!(boss.quality, MobQuality::Boss));
    }

    #[test]
    fn mob_spec_with_quality_preserves_stats() {
        init();
        let base = MobId::Goblin.spec();
        let boss = base.with_quality(MobQuality::Boss);
        assert_eq!(boss.max_health, base.max_health);
        assert_eq!(boss.attack, base.attack);
        assert_eq!(boss.name, base.name);
    }

    #[test]
    fn mob_id_all_contains_expected_mobs() {
        assert!(MobId::ALL.contains(&MobId::Goblin));
        assert!(MobId::ALL.contains(&MobId::Slime));
        assert!(MobId::ALL.contains(&MobId::DwarfKing));
    }

    #[test]
    fn slime_is_normal_quality() {
        init();
        let spec = MobId::Slime.spec();
        assert!(matches!(spec.quality, MobQuality::Normal));
    }

    #[test]
    fn merchant_has_zero_combat_stats() {
        init();
        let spec = MobId::Merchant.spec();
        assert_eq!(*spec.attack.start(), 0);
        assert_eq!(*spec.attack.end(), 0);
        assert_eq!(*spec.defense.start(), 0);
        assert_eq!(*spec.defense.end(), 0);
        assert_eq!(*spec.dropped_gold.start(), 0);
        assert_eq!(*spec.dropped_xp.start(), 0);
    }

    #[test]
    fn all_mobs_have_entity_size() {
        init();
        for mob_id in MobId::ALL {
            let spec = mob_id.spec();
            assert!(spec.entity_size.width > 0.0 && spec.entity_size.height > 0.0);
        }
    }
}
