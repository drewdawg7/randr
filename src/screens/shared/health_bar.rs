use bevy::prelude::*;

/// Component marker for a health bar container.
#[derive(Component)]
pub struct HealthBar;

/// Bundle for a header label (e.g., "PLAYER", "ENEMY").
#[derive(Bundle)]
pub struct HeaderLabelBundle {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
    pub node: Node,
}

impl HeaderLabelBundle {
    pub fn new(label: &str, color: Color) -> Self {
        Self {
            text: Text::new(label),
            font: TextFont {
                font_size: 24.0,
                ..default()
            },
            color: TextColor(color),
            node: Node {
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
        }
    }
}

/// Bundle for the health bar container.
#[derive(Bundle)]
pub struct HealthBarBundle {
    pub bar: HealthBar,
    pub node: Node,
}

impl HealthBarBundle {
    pub fn new(width: f32) -> Self {
        Self {
            bar: HealthBar,
            node: Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(5.0),
                width: Val::Px(width),
                ..default()
            },
        }
    }
}

/// Bundle for the name label inside a health bar.
#[derive(Bundle)]
pub struct HealthBarNameBundle {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl HealthBarNameBundle {
    pub fn new(name: &str) -> Self {
        Self {
            text: Text::new(name),
            font: TextFont {
                font_size: 18.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
        }
    }
}

/// Bundle for the health bar background.
#[derive(Bundle)]
pub struct HealthBarBackgroundBundle {
    pub node: Node,
    pub background: BackgroundColor,
}

impl Default for HealthBarBackgroundBundle {
    fn default() -> Self {
        Self {
            node: Node {
                width: Val::Percent(100.0),
                height: Val::Px(20.0),
                ..default()
            },
            background: BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
        }
    }
}

/// Bundle for the health bar fill portion.
#[derive(Bundle)]
pub struct HealthBarFillBundle {
    pub fill: HealthBarFill,
    pub node: Node,
    pub background: BackgroundColor,
}

impl HealthBarFillBundle {
    pub fn new(health: i32, max_health: i32, fill_color: Color) -> Self {
        let fill_percent = if max_health > 0 {
            (health as f32 / max_health as f32 * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        };
        Self {
            fill: HealthBarFill,
            node: Node {
                width: Val::Percent(fill_percent),
                height: Val::Percent(100.0),
                ..default()
            },
            background: BackgroundColor(fill_color),
        }
    }
}

/// Bundle for the health bar text display.
#[derive(Bundle)]
pub struct HealthBarTextBundle {
    pub marker: HealthBarText,
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl HealthBarTextBundle {
    pub fn new(health: i32, max_health: i32) -> Self {
        Self {
            marker: HealthBarText,
            text: Text::new(format!("{}/{}", health, max_health)),
            font: TextFont {
                font_size: 14.0,
                ..default()
            },
            color: TextColor(Color::srgb(0.8, 0.8, 0.8)),
        }
    }
}

/// Component marker for the filled portion of a health bar.
#[derive(Component)]
pub struct HealthBarFill;

/// Component marker for the health text display.
#[derive(Component)]
pub struct HealthBarText;

/// Update a health bar's fill and text based on new values.
pub fn update_health_bar(
    _commands: &mut Commands,
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
