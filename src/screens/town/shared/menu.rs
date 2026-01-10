use bevy::prelude::*;

use crate::ui::{selection_colors, selection_prefix};

/// A menu option with label and optional description.
#[derive(Clone)]
pub struct MenuOption {
    pub label: &'static str,
    pub description: Option<&'static str>,
}

/// Marker component for menu option items with their index and label.
#[derive(Component)]
pub struct MenuOptionItem {
    pub index: usize,
    pub label: &'static str,
}

/// Marker for the text child of a menu option item.
#[derive(Component)]
pub struct MenuOptionText;

/// Spawn a menu option as a Bevy UI node.
pub fn spawn_menu_option(
    parent: &mut ChildBuilder,
    option: &MenuOption,
    index: usize,
    is_selected: bool,
) {
    let (bg_color, text_color) = selection_colors(is_selected);

    let prefix = selection_prefix(is_selected);

    parent
        .spawn((
            MenuOptionItem {
                index,
                label: option.label,
            },
            Node {
                padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(bg_color),
        ))
        .with_children(|item| {
            item.spawn((
                MenuOptionText,
                Text::new(format!("{}{}", prefix, option.label)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
}

/// Update menu selection highlighting. Call this from your tab's update system.
pub fn update_menu_selection<F1, F2>(
    selected_index: usize,
    menu_query: &mut Query<(&MenuOptionItem, &mut BackgroundColor, &Children), F1>,
    text_query: &mut Query<(&mut Text, &mut TextColor), F2>,
)
where
    F1: bevy::ecs::query::QueryFilter,
    F2: bevy::ecs::query::QueryFilter,
{
    for (item, mut bg_color, children) in menu_query.iter_mut() {
        let is_selected = item.index == selected_index;
        let (new_bg, text_color) = selection_colors(is_selected);
        *bg_color = new_bg.into();

        // Update child text
        for &child in children.iter() {
            if let Ok((mut text, mut color)) = text_query.get_mut(child) {
                let prefix = selection_prefix(is_selected);
                **text = format!("{}{}", prefix, item.label);
                *color = text_color.into();
            }
        }
    }
}

/// Spawn a complete menu with multiple options.
pub fn spawn_menu(
    parent: &mut ChildBuilder,
    options: &[MenuOption],
    selected_index: usize,
    title: Option<&str>,
) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                ..default()
            },
        ))
        .with_children(|menu| {
            // Optional title
            if let Some(title) = title {
                menu.spawn((
                    Text::new(title),
                    TextFont {
                        font_size: 24.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.5)),
                    Node {
                        margin: UiRect::bottom(Val::Px(10.0)),
                        ..default()
                    },
                ));
            }

            // Menu options
            for (i, option) in options.iter().enumerate() {
                spawn_menu_option(menu, option, i, i == selected_index);
            }
        });
}
