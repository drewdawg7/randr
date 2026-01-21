use bevy::prelude::*;

use crate::mob::MobId;

/// Component marker for the monster compendium UI.
#[derive(Component)]
pub struct MonsterCompendiumRoot;

/// Component marker for monster list items, with their index.
#[derive(Component)]
pub struct MonsterListItem(pub usize);

/// Component marker for the mob sprite display in the compendium.
#[derive(Component)]
pub struct CompendiumMobSprite;

/// Resource tracking the selected monster in the compendium.
#[derive(Resource, Default)]
pub struct CompendiumListState {
    pub selected: usize,
}

/// Marker resource to trigger spawning the monster compendium.
#[derive(Resource)]
pub struct SpawnMonsterCompendium;

/// Display information for a monster in the compendium.
/// Decouples UI from game entity registries (MobId::ALL).
#[derive(Clone)]
pub struct MonsterEntry {
    pub name: String,
    pub mob_id: MobId,
}

/// Pre-computed list of monsters for the compendium display.
#[derive(Resource)]
pub struct CompendiumMonsters(pub Vec<MonsterEntry>);

impl CompendiumMonsters {
    /// Create the compendium monster list from the mob registry.
    pub fn from_registry() -> Self {
        Self(
            MobId::ALL
                .iter()
                .map(|mob_id| MonsterEntry {
                    name: mob_id.spec().name.clone(),
                    mob_id: *mob_id,
                })
                .collect(),
        )
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&MonsterEntry> {
        self.0.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &MonsterEntry> {
        self.0.iter()
    }
}
