use bevy::prelude::*;

/// Creates a horizontal row node with centered items and the specified gap between them.
pub fn row_node(gap: f32) -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(gap),
        ..default()
    }
}

/// Creates a vertical column node with the specified gap between items.
pub fn column_node(gap: f32) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(gap),
        ..default()
    }
}
