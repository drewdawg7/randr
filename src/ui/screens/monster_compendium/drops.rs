use bevy::prelude::*;

use crate::assets::GameSprites;
use crate::ui::{FocusPanel, FocusState, SelectionState};

use super::constants::*;
use super::state::{
    CompendiumDetailView, CompendiumDropsSection, CompendiumListState, CompendiumMonsters,
    CompendiumViewState, DropEntry, DropListItem, DropsListState,
};

pub fn update_drops_display(
    mut commands: Commands,
    list_state: Res<CompendiumListState>,
    view_state: Res<CompendiumViewState>,
    mut drops_state: ResMut<DropsListState>,
    monsters: Option<Res<CompendiumMonsters>>,
    focus_state: Option<Res<FocusState>>,
    game_sprites: Res<GameSprites>,
    mut drops_section: Query<(Entity, &mut Node, Option<&Children>), With<CompendiumDropsSection>>,
    added: Query<Entity, Added<CompendiumDropsSection>>,
) {
    let needs_update = list_state.is_changed() || view_state.is_changed() || !added.is_empty();
    if !needs_update {
        return;
    }

    let Some(monsters) = monsters else { return };
    let Some(entry) = monsters.get(list_state.selected) else { return };
    let Ok((section_entity, mut node, children)) = drops_section.single_mut() else { return };

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

    if let Some(children) = children {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }
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
                column_gap: Val::Px(ICON_TEXT_GAP),
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
