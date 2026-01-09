use bevy::prelude::*;

/// Component marker for a health bar container.
#[derive(Component)]
pub struct HealthBar {
    pub entity_name: String,
}

/// Component marker for the filled portion of a health bar.
#[derive(Component)]
pub struct HealthBarFill;

/// Component marker for the health text display.
#[derive(Component)]
pub struct HealthBarText;

/// Spawn a health bar widget with name, current/max HP, and colored fill.
pub fn spawn_health_bar(
    parent: &mut ChildBuilder,
    name: &str,
    current: i32,
    max: i32,
    color: Color,
) -> Entity {
    let fill_percent = if max > 0 {
        (current as f32 / max as f32 * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    parent
        .spawn((
            HealthBar {
                entity_name: name.to_string(),
            },
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                width: Val::Px(200.0),
                ..default()
            },
        ))
        .with_children(|bar| {
            // Entity name
            bar.spawn((
                Text::new(name),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Bar container (background)
            bar.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(20.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            ))
            .with_children(|container| {
                // Fill bar
                container.spawn((
                    HealthBarFill,
                    Node {
                        width: Val::Percent(fill_percent),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    BackgroundColor(color),
                ));
            });

            // HP text
            bar.spawn((
                HealthBarText,
                Text::new(format!("{}/{}", current, max)),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));
        })
        .id()
}

/// Update a health bar's fill and text based on new values.
pub fn update_health_bar(
    commands: &mut Commands,
    bar_entity: Entity,
    current: i32,
    max: i32,
    children: &Query<&Children>,
    fill_query: &mut Query<&mut Node, With<HealthBarFill>>,
    text_query: &mut Query<&mut Text, With<HealthBarText>>,
) {
    let fill_percent = if max > 0 {
        (current as f32 / max as f32 * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };

    // Find fill and text components in children
    if let Ok(bar_children) = children.get(bar_entity) {
        for child in bar_children.iter() {
            // Update fill width
            if let Ok(mut fill_node) = fill_query.get_mut(*child) {
                fill_node.width = Val::Percent(fill_percent);
            }
            // Update text
            if let Ok(mut text) = text_query.get_mut(*child) {
                **text = format!("{}/{}", current, max);
            }
            // Recurse into container children
            if let Ok(grandchildren) = children.get(*child) {
                for grandchild in grandchildren.iter() {
                    if let Ok(mut fill_node) = fill_query.get_mut(*grandchild) {
                        fill_node.width = Val::Percent(fill_percent);
                    }
                }
            }
        }
    }
}
