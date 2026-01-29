use bevy::prelude::*;

use crate::assets::{BookSlotSlice, GameSprites, SpriteSheetKey, UiAllSlice};
use crate::ui::{FocusPanel, FocusState, Modal, ModalBackground, MobSpriteSheets, SelectionState, SpawnModalExt, SpriteAnimation};

use super::constants::*;
use super::state::{
    CompendiumDropsSection, CompendiumListState, CompendiumMobSprite, CompendiumMonsters,
    DropEntry, DropListItem, DropsListState, MonsterCompendiumRoot, MonsterListItem,
};

/// System to spawn the monster compendium UI.
pub fn do_spawn_monster_compendium(
    mut commands: Commands,
    game_sprites: Res<GameSprites>,
    monsters: Res<CompendiumMonsters>,
) {
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

    let monsters = monsters.clone();

    commands.spawn_modal(
        Modal::new()
            .background(ModalBackground::Atlas {
                texture: ui_all.texture.clone(),
                layout: ui_all.layout.clone(),
                index: book_idx,
            })
            .size(BOOK_WIDTH, BOOK_HEIGHT)
            .with_root_marker(|e| {
                e.insert(MonsterCompendiumRoot);
            })
            .content(move |book| {
                spawn_left_page(book, &monsters);
                spawn_right_page(book, slot_sprite);
            }),
    );
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
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    })
    .with_children(|right_page| {
        // Slot with mob sprite at top
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

        // Drops section below slot
        right_page.spawn((
            CompendiumDropsSection,
            Node {
                width: Val::Px(RIGHT_PAGE_WIDTH),
                height: Val::Px(DROPS_SECTION_HEIGHT),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip(),
                margin: UiRect::top(Val::Px(8.0)),
                padding: UiRect::horizontal(Val::Px(10.0)),
                ..default()
            },
        ));
    });
}

pub fn update_monster_list_display(
    list_state: Res<CompendiumListState>,
    focus_state: Option<Res<FocusState>>,
    mut items: Query<(&MonsterListItem, &mut TextColor)>,
) {
    let Some(focus_state) = focus_state else { return };
    if !list_state.is_changed() && !focus_state.is_changed() {
        return;
    }

    let monsters_focused = focus_state.is_focused(FocusPanel::CompendiumMonsterList);

    for (item, mut color) in items.iter_mut() {
        if item.0 == list_state.selected && monsters_focused {
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
                SpriteAnimation::new(&sheet.animation.clone().into()),
            ));
        } else {
            commands
                .entity(entity)
                .remove::<ImageNode>()
                .remove::<SpriteAnimation>();
        }
    }
}

pub fn update_drops_display(
    mut commands: Commands,
    list_state: Res<CompendiumListState>,
    mut drops_state: ResMut<DropsListState>,
    monsters: Option<Res<CompendiumMonsters>>,
    focus_state: Option<Res<FocusState>>,
    game_sprites: Res<GameSprites>,
    drops_section: Query<Entity, With<CompendiumDropsSection>>,
    added: Query<Entity, Added<CompendiumDropsSection>>,
) {
    let needs_update = list_state.is_changed() || !added.is_empty();
    if !needs_update {
        return;
    }

    let Some(monsters) = monsters else { return };
    let Some(entry) = monsters.get(list_state.selected) else { return };
    let Ok(section_entity) = drops_section.get_single() else { return };

    drops_state.count = entry.drops.len();
    drops_state.reset();

    let drops_focused = focus_state
        .as_ref()
        .map_or(false, |f| f.is_focused(FocusPanel::CompendiumDropsList));

    commands.entity(section_entity).despawn_descendants();
    commands.entity(section_entity).with_children(|parent| {
        parent.spawn((
            Text::new("Drops:"),
            TextFont { font_size: DROP_FONT_SIZE, ..default() },
            TextColor(NORMAL_COLOR),
        ));

        if entry.drops.is_empty() {
            parent.spawn((
                Text::new("No item drops"),
                TextFont { font_size: DROP_FONT_SIZE, ..default() },
                TextColor(NORMAL_COLOR),
            ));
        } else {
            for (idx, drop) in entry.drops.iter().enumerate() {
                let is_selected = drops_focused && idx == drops_state.selected;
                let text_color = if is_selected { SELECTED_COLOR } else { NORMAL_COLOR };
                spawn_drop_row(parent, idx, drop, &*game_sprites, text_color);
            }
        }
    });
}

fn spawn_drop_row(
    parent: &mut ChildBuilder,
    idx: usize,
    drop: &DropEntry,
    game_sprites: &GameSprites,
    text_color: Color,
) {
    let quantity_str = if drop.quantity_min == drop.quantity_max {
        format!("({})", drop.quantity_min)
    } else {
        format!("({}-{})", drop.quantity_min, drop.quantity_max)
    };

    let display_text = format!(
        "{} - {:.0}% {}",
        drop.item_name, drop.drop_percent, quantity_str
    );

    parent
        .spawn((
            DropListItem(idx),
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(4.0),
                height: Val::Px(DROP_ROW_HEIGHT),
                ..default()
            },
        ))
        .with_children(|row| {
            if let Some(sheet) = game_sprites.get(drop.item_id.sprite_sheet_key()) {
                let sprite_name = drop.item_id.sprite_name();
                if let Some(bundle) = sheet.image_bundle(sprite_name, DROP_ICON_SIZE, DROP_ICON_SIZE)
                {
                    row.spawn(bundle);
                }
            }

            row.spawn((
                Text::new(display_text),
                TextFont { font_size: DROP_FONT_SIZE, ..default() },
                TextColor(text_color),
            ));
        });
}

pub fn update_drops_list_colors(
    drops_state: Option<Res<DropsListState>>,
    focus_state: Option<Res<FocusState>>,
    items: Query<(&DropListItem, &Children)>,
    mut texts: Query<&mut TextColor>,
) {
    let Some(drops_state) = drops_state else { return };
    let Some(focus_state) = focus_state else { return };

    if !drops_state.is_changed() && !focus_state.is_changed() {
        return;
    }

    let drops_focused = focus_state.is_focused(FocusPanel::CompendiumDropsList);

    for (item, children) in items.iter() {
        let is_selected = drops_focused && item.0 == drops_state.selected;
        let color = if is_selected { SELECTED_COLOR } else { NORMAL_COLOR };

        for &child in children.iter() {
            if let Ok(mut text_color) = texts.get_mut(child) {
                *text_color = TextColor(color);
            }
        }
    }
}
