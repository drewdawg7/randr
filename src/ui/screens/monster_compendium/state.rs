use bevy::prelude::*;

use crate::data::StatRange;
use crate::loot::definition::LootItem;
use crate::mob::MobId;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;
use crate::ui::{FocusPanel, FocusState, SelectionState};

use super::spawn::do_spawn_monster_compendium;

/// Component marker for the monster compendium UI.
#[derive(Component)]
pub struct MonsterCompendiumRoot;

/// Component marker for monster list items, with their index.
#[derive(Component)]
pub struct MonsterListItem(pub usize);

/// Component marker for the mob sprite display in the compendium.
#[derive(Component)]
pub struct CompendiumMobSprite;

#[derive(Component)]
pub struct CompendiumStatsSection;

#[derive(Component)]
pub struct CompendiumDropsSection;

#[derive(Component)]
pub struct DropListItem(pub usize);

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum CompendiumDetailView {
    #[default]
    Stats,
    Drops,
}

#[derive(Resource, Default)]
pub struct CompendiumViewState {
    pub view: CompendiumDetailView,
}

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

#[derive(Resource, Default)]
pub struct DropsListState {
    pub selected: usize,
    pub count: usize,
}

impl SelectionState for DropsListState {
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

#[derive(Clone)]
pub struct MonsterEntry {
    pub name: String,
    pub mob_id: MobId,
    pub drops: Vec<LootItem>,
    pub max_health: StatRange,
    pub attack: StatRange,
    pub defense: StatRange,
    pub dropped_gold: StatRange,
    pub dropped_xp: StatRange,
}

/// Pre-computed list of monsters for the compendium display.
#[derive(Resource, Clone)]
pub struct CompendiumMonsters(pub Vec<MonsterEntry>);

impl CompendiumMonsters {
    pub fn from_registry() -> Self {
        Self(
            MobId::ALL
                .iter()
                .map(|mob_id| {
                    let spec = mob_id.spec();
                    let mut drops: Vec<LootItem> = spec
                        .loot
                        .iter()
                        .cloned()
                        .collect();

                    drops.sort_by(|a, b| {
                        a.drop_chance_percent()
                            .partial_cmp(&b.drop_chance_percent())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });

                    MonsterEntry {
                        name: spec.name.clone(),
                        mob_id: *mob_id,
                        drops,
                        max_health: spec.max_health.clone(),
                        attack: spec.attack.clone(),
                        defense: spec.defense.clone(),
                        dropped_gold: spec.dropped_gold.clone(),
                        dropped_xp: spec.dropped_xp.clone(),
                    }
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
        let monsters = CompendiumMonsters::from_registry();
        let count = monsters.len();

        world.resource_mut::<CompendiumListState>().count = count;
        world.resource_mut::<CompendiumListState>().reset();
        world.resource_mut::<DropsListState>().reset();
        world.resource_mut::<CompendiumViewState>().view = CompendiumDetailView::Stats;
        world.insert_resource(monsters);

        world.insert_resource(FocusState::default());
        world
            .resource_mut::<FocusState>()
            .set_focus(FocusPanel::CompendiumMonsterList);

        world.run_system_cached(do_spawn_monster_compendium).ok();
    }

    fn cleanup(world: &mut World) {
        world.remove_resource::<CompendiumMonsters>();
        world.remove_resource::<FocusState>();
    }
}
