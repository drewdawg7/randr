use bevy::prelude::*;

use crate::item::ItemId;
use crate::loot::definition::LootItem;
use crate::mob::MobId;
use crate::ui::modal_registry::RegisteredModal;
use crate::ui::screens::modal::ModalType;
use crate::ui::{FocusPanel, FocusState, SelectionState};

use super::render::do_spawn_monster_compendium;

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
pub struct CompendiumDropsSection;

#[derive(Component)]
pub struct DropListItem(pub usize);

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
pub struct DropEntry {
    pub item_id: ItemId,
    pub item_name: String,
    pub drop_percent: f32,
    pub quantity_min: i32,
    pub quantity_max: i32,
}

impl DropEntry {
    pub fn from_loot_item(item: &LootItem) -> Self {
        let item_id = item.item_id();
        let spec = item_id.spec();
        Self {
            item_id,
            item_name: spec.name.clone(),
            drop_percent: item.drop_chance_percent(),
            quantity_min: *item.quantity_range().start(),
            quantity_max: *item.quantity_range().end(),
        }
    }
}

#[derive(Clone)]
pub struct MonsterEntry {
    pub name: String,
    pub mob_id: MobId,
    pub drops: Vec<DropEntry>,
}

/// Pre-computed list of monsters for the compendium display.
#[derive(Resource)]
pub struct CompendiumMonsters(pub Vec<MonsterEntry>);

impl CompendiumMonsters {
    pub fn from_registry() -> Self {
        Self(
            MobId::ALL
                .iter()
                .map(|mob_id| {
                    let spec = mob_id.spec();
                    let mut drops: Vec<DropEntry> = spec
                        .loot
                        .iter()
                        .map(DropEntry::from_loot_item)
                        .collect();

                    drops.sort_by(|a, b| {
                        a.drop_percent
                            .partial_cmp(&b.drop_percent)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    });

                    MonsterEntry {
                        name: spec.name.clone(),
                        mob_id: *mob_id,
                        drops,
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
