use bevy::prelude::*;

/// Component marker for a health bar container.
#[derive(Component)]
pub struct HealthBar;

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
