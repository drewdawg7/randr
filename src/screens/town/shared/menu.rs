use bevy::prelude::*;

/// A menu option with label and optional description.
#[derive(Clone)]
pub struct MenuOption {
    pub label: &'static str,
    pub description: Option<&'static str>,
}


/// Marker component for menu option items.
#[derive(Component)]
pub struct MenuOptionItem;

/// Spawn a menu option as a Bevy UI node.
pub fn spawn_menu_option(
    parent: &mut ChildBuilder,
    option: &MenuOption,
    _index: usize,
    is_selected: bool,
) {
    let (bg_color, text_color) = if is_selected {
        (Color::srgb(0.3, 0.3, 0.6), Color::WHITE)
    } else {
        (Color::NONE, Color::srgb(0.8, 0.8, 0.8))
    };

    let prefix = if is_selected { "> " } else { "  " };

    parent
        .spawn((
            MenuOptionItem,
            Node {
                padding: UiRect::axes(Val::Px(10.0), Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(bg_color),
        ))
        .with_children(|item| {
            item.spawn((
                Text::new(format!("{}{}", prefix, option.label)),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(text_color),
            ));
        });
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
