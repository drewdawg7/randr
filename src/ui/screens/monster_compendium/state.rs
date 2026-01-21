use bevy::prelude::*;

use crate::mob::MobId;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;
use crate::ui::SelectionState;

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
    pub count: usize,
}

impl SelectionState for CompendiumListState {
    fn selected(&self) -> usize {
        self.selected
    }

    fn count(&self) -> usize {
        self.count
    }

    fn set_selected(&mut self, index: usize) {
        self.selected = index;
    }
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

/// Type-safe handle for the monster compendium modal.
///
/// Used with `ModalCommands`:
/// ```ignore
/// commands.toggle_modal::<MonsterCompendiumModal>();
/// commands.close_modal::<MonsterCompendiumModal>();
/// ```
pub struct MonsterCompendiumModal;

impl RegisteredModal for MonsterCompendiumModal {
    type Root = MonsterCompendiumRoot;
    const MODAL_TYPE: ModalType = ModalType::MonsterCompendium;

    fn spawn(world: &mut World) {
        // Build monster list and reset selection
        let monsters = CompendiumMonsters::from_registry();
        let count = monsters.len();

        world.resource_mut::<CompendiumListState>().count = count;
        world.resource_mut::<CompendiumListState>().reset();
        world.insert_resource(monsters);
        world.insert_resource(SpawnMonsterCompendium);
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<CompendiumMonsters>();
    }
}
