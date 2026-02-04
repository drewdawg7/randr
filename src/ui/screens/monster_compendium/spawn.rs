use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey, UiAllSlice};
use crate::ui::{Modal, ModalBackground, SpawnModalExt};

use super::constants::*;
use super::state::{
    CompendiumDropsSection, CompendiumMobSprite, CompendiumMonsters, CompendiumStatsSection,
    MonsterCompendiumRoot, MonsterListItem,
};

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
                        padding: UiRect::horizontal(Val::Px(SECTION_PADDING)),
                        ..default()
                    },
                ));

                content_area.spawn((
                    CompendiumDropsSection,
                    Node {
                        width: Val::Px(RIGHT_PAGE_WIDTH),
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow::clip(),
                        padding: UiRect::horizontal(Val::Px(SECTION_PADDING)),
                        ..default()
                    },
                ));
            });
    });
}
