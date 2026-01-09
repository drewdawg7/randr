use bevy::prelude::*;

/// Resource tracking which modal is currently open, if any.
#[derive(Resource, Default)]
pub struct ActiveModal {
    pub modal: Option<ModalType>,
}

/// Types of modals available in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalType {
    Profile,
    Inventory,
    Keybinds,
    SpellTest,
}

/// Component marker for modal overlay background.
#[derive(Component)]
pub struct ModalOverlay;

/// Component marker for modal content container.
#[derive(Component)]
pub struct ModalContent;

/// Plugin that manages modal state and lifecycle.
pub struct ModalPlugin;

impl Plugin for ModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveModal>();
    }
}

/// Helper to spawn a modal overlay with semi-transparent background.
pub fn spawn_modal_overlay(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            ModalOverlay,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            ZIndex(100),
        ))
        .id()
}

/// Helper to create a modal container node.
pub fn create_modal_container() -> Node {
    Node {
        width: Val::Px(800.0),
        max_width: Val::Percent(90.0),
        height: Val::Auto,
        max_height: Val::Percent(80.0),
        flex_direction: FlexDirection::Column,
        padding: UiRect::all(Val::Px(30.0)),
        border: UiRect::all(Val::Px(3.0)),
        ..default()
    }
}

/// Helper to create a modal title text.
pub fn create_modal_title(title: &str) -> impl Bundle {
    (
        Text::new(title),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.9, 0.7)),
        Node {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        },
    )
}

/// Helper to create modal section text.
pub fn create_modal_section(text: &str, color: Color) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(color),
        Node {
            margin: UiRect::bottom(Val::Px(10.0)),
            ..default()
        },
    )
}

/// Helper to create modal instruction text.
pub fn create_modal_instruction(text: &str) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(Color::srgb(0.6, 0.6, 0.6)),
        Node {
            margin: UiRect::top(Val::Px(20.0)),
            ..default()
        },
    )
}
