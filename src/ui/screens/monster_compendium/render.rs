use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::assets::{GameSprites, ItemDetailIconsSlice, SpriteSheetKey, UiAllSlice};
use crate::ui::{FocusPanel, FocusState, Modal, ModalBackground, MobSpriteSheets, SelectionState, SpawnModalExt, SpriteAnimation};

use super::constants::*;
use super::state::{
    CompendiumDetailView, CompendiumDropsSection, CompendiumListState, CompendiumMobSprite,
    CompendiumMonsters, CompendiumStatsSection, CompendiumViewState, DropEntry, DropListItem,
    DropsListState, MonsterCompendiumRoot, MonsterListItem,
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

    let monsters = monsters.clone();

    commands.spawn_modal(
        Modal::builder()
            .background(ModalBackground::Atlas {
                texture: ui_all.texture.clone(),
                layout: ui_all.layout.clone(),
                index: book_idx,
            })
            .size((BOOK_WIDTH, BOOK_HEIGHT))
            .root_marker(Box::new(|e| {
                e.insert(MonsterCompendiumRoot);
            }))
            .content(Box::new(move |book| {
                spawn_left_page(book, &monsters);
                spawn_right_page(book);
            }))
            .build(),
    );
}

/// Spawn the left page with the monster list.
fn spawn_left_page(book: &mut ChildSpawnerCommands, monsters: &CompendiumMonsters) {
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

fn spawn_right_page(book: &mut ChildSpawnerCommands) {
    book.spawn(Node {
        position_type: PositionType::Absolute,
        left: Val::Px(RIGHT_PAGE_LEFT),
        top: Val::Px(RIGHT_PAGE_TOP),
        width: Val::Px(RIGHT_PAGE_WIDTH),
        height: Val::Px(RIGHT_PAGE_HEIGHT),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        overflow: Overflow::clip(),
        ..default()
    })
    .with_children(|right_page| {
        right_page.spawn((
            CompendiumMobSprite,
            Node {
                width: Val::Px(MOB_SPRITE_SIZE),
                height: Val::Px(MOB_SPRITE_SIZE),
                ..default()
            },
        ));

        right_page
            .spawn(Node {
                width: Val::Px(RIGHT_PAGE_WIDTH),
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            })
            .with_children(|content_area| {
                content_area.spawn((
                    CompendiumStatsSection,
                    Node {
                        width: Val::Px(RIGHT_PAGE_WIDTH),
                        flex_direction: FlexDirection::Column,
                        padding: UiRect::horizontal(Val::Px(10.0)),
                        ..default()
                    },
                ));

                content_area.spawn((
                    CompendiumDropsSection,
                    Node {
                        width: Val::Px(RIGHT_PAGE_WIDTH),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip(),
                        padding: UiRect::horizontal(Val::Px(10.0)),
                        ..default()
                    },
                ));
            });
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

pub fn update_stats_display(
    mut commands: Commands,
    list_state: Res<CompendiumListState>,
    view_state: Res<CompendiumViewState>,
    monsters: Option<Res<CompendiumMonsters>>,
    game_sprites: Res<GameSprites>,
    mut stats_section: Query<(Entity, &mut Node), With<CompendiumStatsSection>>,
    added: Query<Entity, Added<CompendiumStatsSection>>,
) {
    let needs_update = list_state.is_changed() || view_state.is_changed() || !added.is_empty();
    if !needs_update {
        return;
    }

    let Some(monsters) = monsters else { return };
    let Some(entry) = monsters.get(list_state.selected) else { return };
    let Ok((section_entity, mut node)) = stats_section.single_mut() else { return };

    let is_visible = view_state.view == CompendiumDetailView::Stats;
    node.display = if is_visible {
        Display::Flex
    } else {
        Display::None
    };

    if !is_visible {
        return;
    }

    commands.entity(section_entity).despawn();
    commands.entity(section_entity).with_children(|parent| {
        spawn_stat_item(
            parent,
            ItemDetailIconsSlice::HealthIcon,
            "HP",
            &entry.max_health,
            &game_sprites,
        );

        parent
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|row| {
                spawn_stat_item(
                    row,
                    ItemDetailIconsSlice::AttackIcon,
                    "ATK",
                    &entry.attack,
                    &game_sprites,
                );
                spawn_stat_item(
                    row,
                    ItemDetailIconsSlice::DefenseIcon,
                    "DEF",
                    &entry.defense,
                    &game_sprites,
                );
            });

        parent
            .spawn(Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            })
            .with_children(|row| {
                spawn_stat_item(
                    row,
                    ItemDetailIconsSlice::GoldIcon,
                    "Gold",
                    &entry.dropped_gold,
                    &game_sprites,
                );
                spawn_stat_item(
                    row,
                    ItemDetailIconsSlice::DefaultStatIcon,
                    "XP",
                    &entry.dropped_xp,
                    &game_sprites,
                );
            });
    });
}

fn spawn_stat_item(
    parent: &mut ChildSpawnerCommands,
    icon_slice: ItemDetailIconsSlice,
    label: &str,
    range: &RangeInclusive<i32>,
    game_sprites: &GameSprites,
) {
    let value_str = if range.start() == range.end() {
        format!("{}: {}", label, range.start())
    } else {
        format!("{}: {}-{}", label, range.start(), range.end())
    };

    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            column_gap: Val::Px(4.0),
            height: Val::Px(STAT_ROW_HEIGHT),
            ..default()
        })
        .with_children(|row| {
            if let Some(sheet) = game_sprites.get(icon_slice.sprite_sheet_key()) {
                if let Some(bundle) =
                    sheet.image_bundle(icon_slice.as_str(), STAT_ICON_SIZE, STAT_ICON_SIZE)
                {
                    row.spawn(bundle);
                }
            }

            row.spawn((
                Text::new(value_str),
                TextFont {
                    font_size: STAT_FONT_SIZE,
                    ..default()
                },
                TextColor(NORMAL_COLOR),
            ));
        });
}

pub fn update_drops_display(
    mut commands: Commands,
    list_state: Res<CompendiumListState>,
    view_state: Res<CompendiumViewState>,
    mut drops_state: ResMut<DropsListState>,
    monsters: Option<Res<CompendiumMonsters>>,
    focus_state: Option<Res<FocusState>>,
    game_sprites: Res<GameSprites>,
    mut drops_section: Query<(Entity, &mut Node), With<CompendiumDropsSection>>,
    added: Query<Entity, Added<CompendiumDropsSection>>,
) {
    let needs_update = list_state.is_changed() || view_state.is_changed() || !added.is_empty();
    if !needs_update {
        return;
    }

    let Some(monsters) = monsters else { return };
    let Some(entry) = monsters.get(list_state.selected) else { return };
    let Ok((section_entity, mut node)) = drops_section.single_mut() else { return };

    let is_visible = view_state.view == CompendiumDetailView::Drops;
    node.display = if is_visible {
        Display::Flex
    } else {
        Display::None
    };

    if !is_visible {
        return;
    }

    drops_state.count = entry.drops.len();
    drops_state.reset();

    let drops_focused = focus_state
        .as_ref()
        .map_or(false, |f| f.is_focused(FocusPanel::CompendiumDropsList));

    commands.entity(section_entity).despawn();
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
    parent: &mut ChildSpawnerCommands,
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

        for child in children.iter() {
            if let Ok(mut text_color) = texts.get_mut(child) {
                *text_color = TextColor(color);
            }
        }
    }
}
