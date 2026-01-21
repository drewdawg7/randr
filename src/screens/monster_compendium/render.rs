use bevy::prelude::*;

use crate::assets::{BookSlotSlice, GameSprites, SpriteSheetKey, UiAllSlice};
use crate::screens::modal::spawn_modal_overlay;
use crate::ui::{MobAnimation, MobSpriteSheets};

use super::constants::*;
use super::state::{
    CompendiumListState, CompendiumMobSprite, CompendiumMonsters, MonsterCompendiumRoot,
    MonsterListItem, SpawnMonsterCompendium,
};

/// System to spawn the monster compendium UI.
pub fn spawn_monster_compendium(
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
        s.get(BookSlotSlice::Slot.as_str())
            .map(|idx| (s.texture.clone(), s.layout.clone(), idx))
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
                        width: Val::Px(BOOK_WIDTH),
                        height: Val::Px(BOOK_HEIGHT),
                        position_type: PositionType::Relative,
                        ..default()
                    },
                ))
                .with_children(|book| {
                    spawn_left_page(book, &monsters);
                    spawn_right_page(book, slot_sprite);
                });
        });
}

/// Spawn the left page with the monster list.
fn spawn_left_page(book: &mut ChildBuilder, monsters: &CompendiumMonsters) {
    book.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(LEFT_PAGE_LEFT),
        top: Val::Px(LEFT_PAGE_TOP),
        width: Val::Px(LEFT_PAGE_WIDTH),
        height: Val::Px(LEFT_PAGE_HEIGHT),
        flex_direction: FlexDirection::Column,
        overflow: Overflow::clip(),
        row_gap: Val::Px(LEFT_PAGE_ROW_GAP),
        ..default()
    })
    .with_children(|left_page| {
        for (idx, entry) in monsters.iter().enumerate() {
            let is_selected = idx == 0;
            left_page.spawn((
                MonsterListItem(idx),
                Text::new(&entry.name),
                TextFont {
                    font_size: MONSTER_NAME_FONT_SIZE,
                    ..default()
                },
                TextColor(if is_selected {
                    SELECTED_COLOR
                } else {
                    NORMAL_COLOR
                }),
            ));
        }
    });
}

/// Spawn the right page with the slot and mob sprite.
fn spawn_right_page(
    book: &mut ChildBuilder,
    slot_sprite: Option<(Handle<Image>, Handle<TextureAtlasLayout>, usize)>,
) {
    let Some((texture, layout, slot_idx)) = slot_sprite else {
        return;
    };

    book.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(RIGHT_PAGE_LEFT),
        top: Val::Px(RIGHT_PAGE_TOP),
        width: Val::Px(RIGHT_PAGE_WIDTH),
        height: Val::Px(RIGHT_PAGE_HEIGHT),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    })
    .with_children(|right_page| {
        right_page
            .spawn((
                ImageNode::from_atlas_image(
                    texture,
                    TextureAtlas {
                        layout,
                        index: slot_idx,
                    },
                ),
                Node {
                    width: Val::Px(SLOT_SIZE),
                    height: Val::Px(SLOT_SIZE),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
            ))
            .with_children(|slot| {
                slot.spawn((
                    CompendiumMobSprite,
                    Node {
                        width: Val::Px(MOB_SPRITE_SIZE),
                        height: Val::Px(MOB_SPRITE_SIZE),
                        ..default()
                    },
                ));
            });
    });
}

/// System to update monster list item colors based on selection.
pub fn update_monster_list_display(
    list_state: Res<CompendiumListState>,
    mut items: Query<(&MonsterListItem, &mut TextColor)>,
) {
    if !list_state.is_changed() {
        return;
    }

    for (item, mut color) in items.iter_mut() {
        if item.0 == list_state.selected {
            *color = TextColor(SELECTED_COLOR);
        } else {
            *color = TextColor(NORMAL_COLOR);
        }
    }
}

/// System to update the mob sprite based on selection.
pub fn update_compendium_mob_sprite(
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
