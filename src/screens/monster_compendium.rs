use bevy::prelude::*;

use crate::assets::{BookSlotSlice, GameSprites, SpriteSheetKey, UiAllSlice};
use crate::ui::{MobAnimation, MobSpriteSheets};
use crate::input::{GameAction, NavigationDirection};
use crate::mob::MobId;
use crate::screens::modal::{spawn_modal_overlay, ActiveModal, ModalType};

/// Plugin that manages the monster compendium system.
pub struct MonsterCompendiumPlugin;

impl Plugin for MonsterCompendiumPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CompendiumListState>().add_systems(
            Update,
            (
                handle_compendium_toggle,
                handle_compendium_close,
                handle_compendium_navigation,
                update_monster_list_display,
                update_compendium_mob_sprite,
                spawn_monster_compendium.run_if(resource_exists::<SpawnMonsterCompendium>),
            ),
        );
    }
}

/// Component marker for the monster compendium UI.
#[derive(Component)]
pub struct MonsterCompendiumRoot;

/// Component marker for monster list items, with their index.
#[derive(Component)]
pub struct MonsterListItem(usize);

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

/// System to handle opening the monster compendium with 'b' key.
fn handle_compendium_toggle(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    mut list_state: ResMut<CompendiumListState>,
    existing_compendium: Query<Entity, With<MonsterCompendiumRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::OpenCompendium {
            // Close existing compendium if open
            if let Ok(entity) = existing_compendium.get_single() {
                commands.entity(entity).despawn_recursive();
                commands.remove_resource::<CompendiumMonsters>();
                active_modal.modal = None;
            } else if active_modal.modal.is_none() {
                // Reset selection and trigger spawn
                list_state.selected = 0;
                commands.insert_resource(CompendiumMonsters::from_registry());
                commands.insert_resource(SpawnMonsterCompendium);
                active_modal.modal = Some(ModalType::MonsterCompendium);
            }
        }
    }
}

/// System to handle closing the monster compendium with Escape.
fn handle_compendium_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    compendium_query: Query<Entity, With<MonsterCompendiumRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::CloseModal && active_modal.modal == Some(ModalType::MonsterCompendium) {
            if let Ok(entity) = compendium_query.get_single() {
                commands.entity(entity).despawn_recursive();
                commands.remove_resource::<CompendiumMonsters>();
                active_modal.modal = None;
            }
        }
    }
}

/// System to handle up/down navigation in the monster list.
fn handle_compendium_navigation(
    mut action_reader: EventReader<GameAction>,
    active_modal: Res<ActiveModal>,
    mut list_state: ResMut<CompendiumListState>,
    monsters: Option<Res<CompendiumMonsters>>,
) {
    if active_modal.modal != Some(ModalType::MonsterCompendium) {
        return;
    }

    let Some(monsters) = monsters else { return };
    let count = monsters.len();
    for action in action_reader.read() {
        if let GameAction::Navigate(dir) = action {
            match dir {
                NavigationDirection::Up => {
                    if list_state.selected > 0 {
                        list_state.selected -= 1;
                    }
                }
                NavigationDirection::Down => {
                    if list_state.selected < count.saturating_sub(1) {
                        list_state.selected += 1;
                    }
                }
                _ => {}
            }
        }
    }
}

/// System to update monster list item colors based on selection.
fn update_monster_list_display(
    list_state: Res<CompendiumListState>,
    mut items: Query<(&MonsterListItem, &mut TextColor)>,
) {
    if !list_state.is_changed() {
        return;
    }

    for (item, mut color) in items.iter_mut() {
        if item.0 == list_state.selected {
            // Selected: darker brown with highlight
            *color = TextColor(Color::srgb(0.5, 0.3, 0.1));
        } else {
            // Normal: dark brown
            *color = TextColor(Color::srgb(0.2, 0.15, 0.1));
        }
    }
}

/// System to spawn the monster compendium UI.
fn spawn_monster_compendium(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    monsters: Res<CompendiumMonsters>,
) {
    // Remove trigger resource
    commands.remove_resource::<SpawnMonsterCompendium>();

    let Some(ui_all) = game_sprites.get(SpriteSheetKey::UiAll) else {
        return;
    };
    let Some(book_idx) = ui_all.get(UiAllSlice::Book.as_str()) else {
        return;
    };

    let slot_sprite = game_sprites.get(SpriteSheetKey::BookSlot).and_then(|s| {
        s.get(BookSlotSlice::Slot.as_str()).map(|idx| (s.texture.clone(), s.layout.clone(), idx))
    });

    let overlay = spawn_modal_overlay(&mut commands);

    commands
        .entity(overlay)
        .insert(MonsterCompendiumRoot)
        .with_children(|parent| {
            // Book container (relative positioning for children)
            parent
                .spawn((
                    ImageNode::from_atlas_image(
                        ui_all.texture.clone(),
                        TextureAtlas {
                            layout: ui_all.layout.clone(),
                            index: book_idx,
                        },
                    ),
                    Node {
                        width: Val::Px(672.0),
                        height: Val::Px(399.0),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                ))
                .with_children(|book| {
                    // Left page container - positioned on left half with padding
                    book.spawn(Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(45.0),
                        top: Val::Px(40.0),
                        width: Val::Px(260.0),
                        height: Val::Px(320.0),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip(),
                        row_gap: Val::Px(4.0),
                        ..default()
                    })
                    .with_children(|left_page| {
                        // Monster list - uses pre-computed display data
                        for (idx, entry) in monsters.iter().enumerate() {
                            let is_selected = idx == 0;
                            left_page.spawn((
                                MonsterListItem(idx),
                                Text::new(&entry.name),
                                TextFont {
                                    font_size: 14.0,
                                    ..default()
                                },
                                TextColor(if is_selected {
                                    Color::srgb(0.5, 0.3, 0.1)
                                } else {
                                    Color::srgb(0.2, 0.15, 0.1)
                                }),
                            ));
                        }
                    });

                    // Right page container - centered slot sprite
                    if let Some((texture, layout, slot_idx)) = slot_sprite {
                        book.spawn(Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(360.0),
                            top: Val::Px(40.0),
                            width: Val::Px(280.0),
                            height: Val::Px(320.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        })
                        .with_children(|right_page| {
                            right_page
                                .spawn((
                                    ImageNode::from_atlas_image(
                                        texture,
                                        TextureAtlas { layout, index: slot_idx },
                                    ),
                                    Node {
                                        width: Val::Px(112.0),
                                        height: Val::Px(112.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                ))
                                .with_children(|slot| {
                                    slot.spawn((
                                        CompendiumMobSprite,
                                        Node {
                                            width: Val::Px(96.0),
                                            height: Val::Px(96.0),
                                            ..default()
                                        },
                                    ));
                                });
                        });
                    }
                });
        });
}

/// System to update the mob sprite based on selection.
fn update_compendium_mob_sprite(
    mut commands: Commands,
    list_state: Res<CompendiumListState>,
    monsters: Option<Res<CompendiumMonsters>>,
    mob_sheets: Res<MobSpriteSheets>,
    query: Query<Entity, With<CompendiumMobSprite>>,
    added: Query<Entity, Added<CompendiumMobSprite>>,
) {
    let needs_update = list_state.is_changed() || !added.is_empty();
    if !needs_update {
        return;
    }

    let Some(monsters) = monsters else { return };
    let Some(entry) = monsters.get(list_state.selected) else {
        return;
    };

    for entity in &query {
        if let Some(sheet) = mob_sheets.get(entry.mob_id) {
            commands.entity(entity).insert((
                ImageNode::from_atlas_image(
                    sheet.texture.clone(),
                    TextureAtlas {
                        layout: sheet.layout.clone(),
                        index: sheet.animation.first_frame,
                    },
                ),
                MobAnimation::new(&sheet.animation),
            ));
        } else {
            commands
                .entity(entity)
                .remove::<ImageNode>()
                .remove::<MobAnimation>();
        }
    }
}
