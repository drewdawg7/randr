use std::ops::RangeInclusive;

use bevy::prelude::*;

use crate::assets::{GameSprites, ItemDetailIconsSlice};

use super::constants::*;
use super::state::{
    CompendiumDetailView, CompendiumListState, CompendiumMonsters, CompendiumStatsSection,
    CompendiumViewState,
};

pub fn update_stats_display(
    mut commands: Commands,
    list_state: Res<CompendiumListState>,
    view_state: Res<CompendiumViewState>,
    monsters: Option<Res<CompendiumMonsters>>,
    game_sprites: Res<GameSprites>,
    mut stats_section: Query<(Entity, &mut Node, Option<&Children>), With<CompendiumStatsSection>>,
    added: Query<Entity, Added<CompendiumStatsSection>>,
) {
    let needs_update = list_state.is_changed() || view_state.is_changed() || !added.is_empty();
    if !needs_update {
        return;
    }

    let Some(monsters) = monsters else { return };
    let Some(entry) = monsters.get(list_state.selected) else { return };
    let Ok((section_entity, mut node, children)) = stats_section.single_mut() else { return };

    let is_visible = view_state.view == CompendiumDetailView::Stats;
    node.display = if is_visible {
        Display::Flex
    } else {
        Display::None
    };

    if !is_visible {
        return;
    }

    if let Some(children) = children {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }
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
                column_gap: Val::Px(STAT_COLUMN_GAP),
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
                column_gap: Val::Px(STAT_COLUMN_GAP),
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
            column_gap: Val::Px(ICON_TEXT_GAP),
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
