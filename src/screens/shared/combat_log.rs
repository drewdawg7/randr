use bevy::prelude::*;

/// A single entry in the combat log.
#[derive(Debug, Clone)]
pub struct CombatLogEntry {
    pub text: String,
    pub color: Color,
}

impl CombatLogEntry {
    pub fn player_attack(damage: i32, target: &str) -> Self {
        Self {
            text: format!("You attack {} for {} damage!", target, damage),
            color: Color::srgb(0.5, 0.8, 0.5),
        }
    }

    pub fn enemy_attack(damage: i32, attacker: &str) -> Self {
        Self {
            text: format!("{} attacks you for {} damage!", attacker, damage),
            color: Color::srgb(0.8, 0.5, 0.5),
        }
    }

    pub fn enemy_defeated(target: &str) -> Self {
        Self {
            text: format!("{} has been defeated!", target),
            color: Color::srgb(0.9, 0.9, 0.3),
        }
    }

    pub fn player_defeated() -> Self {
        Self {
            text: "You have been defeated...".to_string(),
            color: Color::srgb(0.8, 0.3, 0.3),
        }
    }

    pub fn info(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: Color::srgb(0.7, 0.7, 0.7),
        }
    }
}

/// Component marker for the combat log widget container.
#[derive(Component)]
pub struct CombatLogWidget;

/// Component marker for individual log entries.
#[derive(Component)]
pub struct CombatLogLine {
    pub index: usize,
}

/// Spawn a combat log widget that displays combat messages.
pub fn spawn_combat_log(parent: &mut ChildBuilder, entries: &[CombatLogEntry], max_visible: usize) -> Entity {
    parent
        .spawn((
            CombatLogWidget,
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                min_height: Val::Px(100.0),
                max_height: Val::Px(150.0),
                padding: UiRect::all(Val::Px(10.0)),
                row_gap: Val::Px(4.0),
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
        ))
        .with_children(|log| {
            // Title
            log.spawn((
                Text::new("Combat Log"),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.6, 0.6, 0.6)),
                Node {
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                },
            ));

            // Show last N entries
            let start = entries.len().saturating_sub(max_visible);
            for (i, entry) in entries.iter().skip(start).enumerate() {
                log.spawn((
                    CombatLogLine { index: start + i },
                    Text::new(format!("> {}", entry.text)),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(entry.color),
                ));
            }
        })
        .id()
}
