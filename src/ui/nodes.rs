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

pub fn column_node(gap: f32) -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(gap),
        ..default()
    }
}

pub fn modal_content_row() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(16.0),
        align_items: AlignItems::FlexStart,
        ..default()
    }
}

pub fn separator_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Px(2.0),
        margin: UiRect::vertical(Val::Px(10.0)),
        ..default()
    }
}

pub const SCREEN_BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);

pub fn screen_root_node() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    }
}

pub fn screen_root_bundle() -> (Node, BackgroundColor) {
    (screen_root_node(), BackgroundColor(SCREEN_BG_COLOR))
}
