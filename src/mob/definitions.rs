use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::data::StatRange;
use crate::dungeon::EntitySize;
use crate::loot::LootTable;

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

#[derive(Debug, Clone, Deserialize)]
pub enum MobQuality {
    Normal,
    Boss,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MobSpriteData {
    pub aseprite_path: String,
    pub idle_tag: String,
    pub hurt_tag: Option<String>,
    pub death_tag: Option<String>,
    pub frame_size: (u32, u32),
}

#[derive(Debug, Clone, Deserialize, Asset, TypePath)]
pub struct MobSpec {
    pub id: MobId,
    pub name: String,
    pub quality: MobQuality,
    pub max_health: StatRange,
    pub attack: StatRange,
    pub defense: StatRange,
    pub dropped_gold: StatRange,
    pub dropped_xp: StatRange,
    #[serde(default)]
    pub loot: LootTable,
    #[serde(default)]
    pub entity_size: EntitySize,
    pub sprite: MobSpriteData,
}

impl MobSpec {
    pub fn with_multiplier(&self, multiplier: f32) -> MobSpec {
        MobSpec {
            id: self.id,
            name: self.name.clone(),
            max_health: self.max_health.scale(multiplier),
            attack: self.attack.scale(multiplier),
            defense: self.defense.scale(multiplier),
            dropped_gold: self.dropped_gold.scale(multiplier),
            dropped_xp: self.dropped_xp.scale(multiplier),
            quality: self.quality.clone(),
            loot: self.loot.clone(),
            entity_size: self.entity_size,
            sprite: self.sprite.clone(),
        }
    }

    pub fn with_name(&self, name: impl Into<String>) -> MobSpec {
        MobSpec {
            id: self.id,
            name: name.into(),
            max_health: self.max_health,
            attack: self.attack,
            defense: self.defense,
            dropped_gold: self.dropped_gold,
            dropped_xp: self.dropped_xp,
            quality: self.quality.clone(),
            loot: self.loot.clone(),
            entity_size: self.entity_size,
            sprite: self.sprite.clone(),
        }
    }

    pub fn with_quality(&self, quality: MobQuality) -> MobSpec {
        MobSpec {
            id: self.id,
            name: self.name.clone(),
            max_health: self.max_health,
            attack: self.attack,
            defense: self.defense,
            dropped_gold: self.dropped_gold,
            dropped_xp: self.dropped_xp,
            quality,
            loot: self.loot.clone(),
            entity_size: self.entity_size,
            sprite: self.sprite.clone(),
        }
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

        assert_eq!(scaled.max_health.start(), (base.max_health.start() as f32 * 2.0).round() as i32);
        assert_eq!(scaled.max_health.end(), (base.max_health.end() as f32 * 2.0).round() as i32);
        assert_eq!(scaled.attack.start(), (base.attack.start() as f32 * 2.0).round() as i32);
        assert_eq!(scaled.attack.end(), (base.attack.end() as f32 * 2.0).round() as i32);
        assert_eq!(scaled.defense.start(), (base.defense.start() as f32 * 2.0).round() as i32);
        assert_eq!(scaled.defense.end(), (base.defense.end() as f32 * 2.0).round() as i32);
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
        assert_eq!(spec.attack.start(), 0);
        assert_eq!(spec.attack.end(), 0);
        assert_eq!(spec.defense.start(), 0);
        assert_eq!(spec.defense.end(), 0);
        assert_eq!(spec.dropped_gold.start(), 0);
        assert_eq!(spec.dropped_xp.start(), 0);
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
